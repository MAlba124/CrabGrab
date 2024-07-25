use std::{
    cell::RefCell,
    ffi::CString,
    mem::size_of,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread::JoinHandle,
    time::Duration,
};

use pipewire::{
    context::Context,
    main_loop::MainLoop,
    spa::{
        self,
        param::{
            self,
            format::{self, MediaSubtype, MediaType},
            video::{VideoFormat, VideoInfoRaw},
            ParamType,
        },
        pod::{self, Pod, Property},
        sys::{
            spa_buffer, spa_meta_bitmap, spa_meta_cursor, spa_meta_header,
            SPA_META_Bitmap, SPA_META_Cursor, SPA_META_Header,
            SPA_PARAM_META_size, SPA_PARAM_META_type,
        },
        utils::{Direction, Fraction, Rectangle, SpaTypes},
    },
    stream::{Stream, StreamFlags, StreamRef, StreamState},
};

use crate::{
    frame::VideoFrame,
    prelude::{
        CaptureConfig, CapturePixelFormat, StreamCreateError, StreamError, StreamEvent,
        StreamStopError,
    },
};

use super::frame::WaylandVideoFrame;

#[derive(Default, Debug, PartialEq)]
struct PwMetas<'a> {
    pub header: Option<&'a spa_meta_header>,
    pub cursor: Option<&'a spa_meta_cursor>,
    // TODO: Get the bitmap: https://docs.pipewire.org/video-play_8c-example.html#_a30
    pub bitmap: Option<&'a spa_meta_bitmap>,
}

impl<'a> PwMetas<'a> {
    pub fn from_raw(raw: &'a *mut spa_buffer) -> Self {
        let mut self_ = Self::default();

        unsafe {
            let n_metas = (*(*raw)).n_metas;
            if n_metas == 0 {
                return self_;
            }

            let mut meta_ptr = (*(*raw)).metas;
            let metas_end = (*(*raw)).metas.wrapping_add(n_metas as usize);
            while meta_ptr != metas_end {
                if (*meta_ptr).type_ == SPA_META_Header {
                    self_.header = Some(&mut *((*meta_ptr).data as *mut spa_meta_header));
                } else if (*meta_ptr).type_ == SPA_META_Cursor {
                    self_.cursor = Some(&mut *((*meta_ptr).data as *mut spa_meta_cursor));
                } else if (*meta_ptr).type_ == SPA_META_Bitmap {
                    self_.bitmap = Some(&mut *((*meta_ptr).data as *mut spa_meta_bitmap))
                }
                meta_ptr = meta_ptr.wrapping_add(1);
            }
        }

        self_
    }
}

#[derive(Default)]
struct PwDatas<'a> {
    pub data: &'a [u8],
}

impl<'a> PwDatas<'a> {
    pub fn from_raw(raw: &'a *mut spa_buffer) -> Vec<PwDatas<'a>> {
        let mut datas = Vec::new();

        unsafe {
            let n_datas = (*(*raw)).n_datas;
            if n_datas == 0 {
                return datas;
            }

            let mut data_ptr = (*(*raw)).datas;
            let datas_end = (*(*raw)).datas.wrapping_add(n_datas as usize);
            while data_ptr != datas_end {
                if !(*data_ptr).data.is_null() {
                    datas.push(PwDatas {
                        data: std::slice::from_raw_parts(
                            (*data_ptr).data as *mut u8,
                            (*data_ptr).maxsize as usize,
                        ),
                    });
                }
                data_ptr = data_ptr.wrapping_add(1);
            }
        }

        datas
    }
}

struct WaylandCapturerUD {
    pub format: VideoInfoRaw,
    pub format_negotiating: Rc<RefCell<bool>>,
    pub show_cursor_as_metadata: bool,
    pub start_time: i64,
    pub callback: Box<dyn FnMut(Result<StreamEvent, StreamError>) + Send + 'static>,
}

pub struct WaylandCaptureStream {
    handle: Option<JoinHandle<()>>,
    should_run: Arc<AtomicBool>,
}

impl WaylandCaptureStream {
    pub fn supported_pixel_formats() -> &'static [CapturePixelFormat] {
        &[CapturePixelFormat::Bgra8888]
    }

    pub fn check_access(_borderless: bool) -> Option<WaylandCaptureAccessToken> {
        Some(WaylandCaptureAccessToken(()))
    }

    pub async fn request_access(_borderless: bool) -> Option<WaylandCaptureAccessToken> {
        Some(WaylandCaptureAccessToken(()))
    }

    fn pod_supported_pixel_formats() -> pod::Property {
        pipewire::spa::pod::property!(
            format::FormatProperties::VideoFormat,
            Choice,
            Enum,
            Id,
            VideoFormat::RGBA, // Big-endian
            VideoFormat::RGBx, // Big-endian
            VideoFormat::BGRx, // Big-endian
            VideoFormat::BGRA, // Big-endian
            VideoFormat::ABGR, // Big-endian
            VideoFormat::ARGB, // Big-endian
            VideoFormat::xRGB, // Big-endian
            VideoFormat::xBGR  // Big-endian
        )
    }

    fn pod_supported_resolutions() -> pod::Property {
        pod::property!(
            format::FormatProperties::VideoSize,
            Choice,
            Range,
            Rectangle,
            Rectangle {
                width: 512,
                height: 512
            },
            Rectangle {
                width: 1,
                height: 1
            },
            Rectangle {
                width: 15360,
                height: 8640
            }
        )
    }

    fn pod_supported_framerates() -> pod::Property {
        pod::property!(
            format::FormatProperties::VideoFramerate,
            Choice,
            Range,
            Fraction,
            Fraction { num: 30, denom: 1 },
            Fraction { num: 0, denom: 1 },
            Fraction { num: 512, denom: 1 }
        )
    }

    fn param_changed(stream: &StreamRef, ud: &mut WaylandCapturerUD, id: u32, param: Option<&Pod>) {
        let Some(param) = param else {
            return;
        };

        use pipewire::spa::param;
        if id != ParamType::Format.as_raw() {
            return;
        }

        match param::format_utils::parse_format(param) {
            Ok((media_type, media_subtype)) => {
                if media_type != MediaType::Video || media_subtype != MediaSubtype::Raw {
                    return;
                }
            }
            Err(e) => {
                unsafe {
                    pipewire::sys::pw_stream_set_error(
                        stream.as_raw_ptr(),
                        -1,
                        CString::new(e.to_string()).unwrap().as_c_str().as_ptr(),
                    );
                }
                return;
            }
        };

        let metas_obj = if ud.show_cursor_as_metadata {
            pod::object!(
                SpaTypes::ObjectParamMeta,
                ParamType::Meta,
                Property::new(
                    SPA_PARAM_META_type,
                    pod::Value::Id(spa::utils::Id(SPA_META_Header))
                ),
                Property::new(
                    SPA_PARAM_META_type,
                    pod::Value::Id(spa::utils::Id(SPA_META_Cursor))
                ),
                Property::new(
                    SPA_PARAM_META_type,
                    pod::Value::Id(spa::utils::Id(SPA_META_Bitmap))
                ),
                Property::new(
                    SPA_PARAM_META_size,
                    pod::Value::Int(
                        size_of::<spa::sys::spa_meta_header>() as i32
                            + size_of::<spa::sys::spa_meta_cursor>() as i32
                            + size_of::<spa::sys::spa_meta_bitmap>() as i32
                    )
                ),
            )
        } else {
            pod::object!(
                SpaTypes::ObjectParamMeta,
                ParamType::Meta,
                Property::new(
                    SPA_PARAM_META_type,
                    pod::Value::Id(spa::utils::Id(SPA_META_Header))
                ),
                Property::new(
                    SPA_PARAM_META_size,
                    pod::Value::Int(size_of::<spa::sys::spa_meta_header>() as i32)
                ),
            )
        };

        let metas_values: Vec<u8> = pod::serialize::PodSerializer::serialize(
            std::io::Cursor::new(Vec::new()),
            &pod::Value::Object(metas_obj),
        )
        .unwrap()
        .0
        .into_inner();

        let mut params = [pod::Pod::from_bytes(&metas_values).unwrap()];

        if let Err(e) = stream.update_params(&mut params) {
            unsafe {
                pipewire::sys::pw_stream_set_error(
                    stream.as_raw_ptr(),
                    -1,
                    CString::new(e.to_string()).unwrap().as_c_str().as_ptr(),
                );
            }
            return;
        }

        ud.format.parse(param).unwrap();
        println!( // DEBUGGING
            "Got pixel format: {} ({:?})",
            ud.format.format().as_raw(),
            ud.format.format()
        );

        ud.format_negotiating.replace(false);
    }

    fn process(stream: &StreamRef, ud: &mut WaylandCapturerUD) {
        let raw_buffer = unsafe { stream.dequeue_raw_buffer() };
        if raw_buffer.is_null() {
            unsafe { stream.queue_raw_buffer(raw_buffer) };
            return;
        }

        let buffer = unsafe { (*raw_buffer).buffer };
        if buffer.is_null() {
            unsafe { stream.queue_raw_buffer(raw_buffer) };
            return;
        }

        let metas = PwMetas::from_raw(&buffer);
        let datas = PwDatas::from_raw(&buffer);
        if let (Some(header), Some(data)) = (metas.header, datas.iter().next()) {
            if ud.start_time == 0 {
                ud.start_time = header.pts;
            }

            let frame = WaylandVideoFrame {
                size: crate::prelude::Size {
                    width: ud.format.size().width as f64,
                    height: ud.format.size().height as f64,
                },
                id: header.seq,
                captured: std::time::Instant::now(),
                pts: std::time::Duration::from_nanos((header.pts - ud.start_time) as u64),
                format: ud.format,
                data: data.data.to_vec(),
            };

            (*ud.callback)(Ok(StreamEvent::Video(VideoFrame {
                impl_video_frame: frame,
            })));
        }

        unsafe { stream.queue_raw_buffer(raw_buffer) };
    }

    fn capture_main(
        capture_config: CaptureConfig,
        callback: Box<impl FnMut(Result<StreamEvent, StreamError>) + Send + 'static>,
        should_run: Arc<AtomicBool>,
        init_tx: &Sender<Result<(), StreamCreateError>>,
    ) -> Result<(), StreamCreateError> {
        pipewire::init();
        let mainloop = MainLoop::new(None).map_err(|e| StreamCreateError::Other(e.to_string()))?;
        let context =
            Context::new(&mainloop).map_err(|e| StreamCreateError::Other(e.to_string()))?;
        let core = context
            .connect(None)
            .map_err(|e| StreamCreateError::Other(e.to_string()))?;

        use pipewire::keys;
        let stream = Stream::new(
            &core,
            "crabgrab",
            pipewire::properties::properties! {
                *keys::MEDIA_TYPE => "Video",
                *keys::MEDIA_CATEGORY => "Capture",
                *keys::MEDIA_ROLE => "Screen",
            },
        )
        .map_err(|e| StreamCreateError::Other(e.to_string()))?;

        let format_negotiating = Rc::new(RefCell::new(true));
        let user_data = WaylandCapturerUD {
            format: Default::default(),
            format_negotiating: format_negotiating.clone(),
            show_cursor_as_metadata: capture_config.show_cursor
                && match &capture_config.target {
                    crate::prelude::Capturable::Window(w) => {
                        w.impl_capturable_window.cursor_as_metadata
                    }
                    crate::prelude::Capturable::Display(d) => {
                        d.impl_capturable_display.cursor_as_metadata
                    }
                },
            start_time: 0,
            callback,
        };

        let _listener = stream
            .add_local_listener_with_user_data(user_data)
            .param_changed(Self::param_changed)
            .process(Self::process)
            .register()
            .map_err(|e| StreamCreateError::Other(e.to_string()))?;

        let stream_param_obj = pod::object!(
            spa::utils::SpaTypes::ObjectParamFormat,
            param::ParamType::EnumFormat,
            pod::property!(
                format::FormatProperties::MediaType,
                Id,
                format::MediaType::Video
            ),
            pod::property!(
                format::FormatProperties::MediaSubtype,
                Id,
                format::MediaSubtype::Raw
            ),
            Self::pod_supported_pixel_formats(),
            Self::pod_supported_resolutions(),
            Self::pod_supported_framerates(),
        );

        let stream_param_obj_values: Vec<u8> = pod::serialize::PodSerializer::serialize(
            std::io::Cursor::new(Vec::new()),
            &pod::Value::Object(stream_param_obj),
        )
        .map_err(|e| StreamCreateError::Other(e.to_string()))?
        .0
        .into_inner();

        let mut params = [pod::Pod::from_bytes(&stream_param_obj_values).unwrap()];

        stream
            .connect(
                Direction::Input,
                Some(match capture_config.target {
                    crate::prelude::Capturable::Window(w) => w.impl_capturable_window.pw_node_id,
                    crate::prelude::Capturable::Display(d) => d.impl_capturable_display.pw_node_id,
                }),
                StreamFlags::AUTOCONNECT | StreamFlags::MAP_BUFFERS,
                &mut params,
            )
            .map_err(|e| StreamCreateError::Other(e.to_string()))?;

        let loop_ = mainloop.loop_();

        // Iterate the stream and check for errors while negotiating pixel format
        // BUG: Does not exit when supported format is not found. Find out if it timed out
        while *(*format_negotiating).borrow() {
            loop_.iterate(Duration::from_millis(100));
            match stream.state() {
                StreamState::Error(_) => {
                    return Err(StreamCreateError::UnsupportedPixelFormat);
                }
                _ => {}
            }
        }

        init_tx.send(Ok(())).unwrap();

        while should_run.load(Ordering::Acquire) {
            loop_.iterate(Duration::from_millis(100));
        }

        let _ = stream.disconnect();

        Ok(())
    }

    pub fn new(
        _token: WaylandCaptureAccessToken,
        capture_config: CaptureConfig,
        callback: Box<impl FnMut(Result<StreamEvent, StreamError>) + Send + 'static>,
    ) -> Result<Self, StreamCreateError> {
        let should_run = Arc::new(AtomicBool::new(true));
        let should_run_clone = Arc::clone(&should_run);
        let (init_tx, init_rx) = std::sync::mpsc::channel::<Result<(), StreamCreateError>>();
        let handle = std::thread::spawn(move || {
            if let Err(e) = Self::capture_main(capture_config, callback, should_run_clone, &init_tx)
            {
                init_tx.send(Err(e)).unwrap();
            }
        });

        init_rx.recv().unwrap()?;

        Ok(Self {
            handle: Some(handle),
            should_run,
        })
    }

    pub(crate) fn stop(&mut self) -> Result<(), StreamStopError> {
        if self.should_run.load(Ordering::Acquire) {
            self.should_run
                .store(false, std::sync::atomic::Ordering::SeqCst);
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        Ok(())
    }
}

impl Drop for WaylandCaptureStream {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

#[derive(Clone, Debug)]
pub struct WaylandCaptureConfig {}

impl WaylandCaptureConfig {
    pub fn new() -> Self {
        Self {}
    }
}

#[allow(dead_code)]
pub struct WaylandPixelFormat {}

#[derive(Clone, Copy, Debug)]
pub struct WaylandCaptureAccessToken(());

impl WaylandCaptureAccessToken {
    pub(crate) fn allows_borderless(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use pipewire::spa::sys::spa_meta;

    use super::*;

    #[test]
    fn buffer_metas_extraction_header() {
        let mut meta_header_data = spa_meta_header {
            flags: 1,
            offset: 2,
            pts: 3,
            dts_offset: 4,
            seq: 5,
        };
        let mut metas = [spa_meta {
            type_: SPA_META_Header,
            size: std::mem::size_of_val(&meta_header_data) as u32,
            data: std::ptr::addr_of_mut!(meta_header_data) as *mut c_void,
        }];
        let mut buffer = spa_buffer {
            n_metas: metas.len() as u32,
            n_datas: 0,
            metas: std::ptr::addr_of_mut!(metas) as *mut spa_meta,
            datas: std::ptr::null_mut(),
        };
        let buffer_addr = std::ptr::addr_of_mut!(buffer);
        let extracted_metas = PwMetas::from_raw(&buffer_addr);
        assert_eq!(
            extracted_metas,
            PwMetas {
                header: Some(&meta_header_data),
                cursor: None,
                bitmap: None
            }
        );
    }

    #[test]
    fn buffer_metas_extraction_header_cursor_bitmap() {
        let mut meta_header_data = spa_meta_header {
            flags: 1,
            offset: 2,
            pts: 3,
            dts_offset: 4,
            seq: 5,
        };
        let mut meta_cursor_data = spa_meta_cursor {
            id: 0,
            flags: 123,
            position: spa::sys::spa_point { x: 10, y: 12 },
            hotspot: spa::sys::spa_point { x: 20, y: 22 },
            bitmap_offset: 321,
        };
        let mut meta_bitmap_data = spa_meta_bitmap {
            format: 0,
            size: spa::sys::spa_rectangle {
                width: 0,
                height: 0,
            },
            stride: 0,
            offset: 5,
        };
        let mut metas = [
            spa_meta {
                type_: SPA_META_Header,
                size: std::mem::size_of_val(&meta_header_data) as u32,
                data: std::ptr::addr_of_mut!(meta_header_data) as *mut c_void,
            },
            spa_meta {
                type_: SPA_META_Cursor,
                size: std::mem::size_of_val(&meta_cursor_data) as u32,
                data: std::ptr::addr_of_mut!(meta_cursor_data) as *mut c_void,
            },
            spa_meta {
                type_: SPA_META_Bitmap,
                size: std::mem::size_of_val(&meta_bitmap_data) as u32,
                data: std::ptr::addr_of_mut!(meta_bitmap_data) as *mut c_void,
            },
        ];
        let mut buffer = spa_buffer {
            n_metas: metas.len() as u32,
            n_datas: 0,
            metas: std::ptr::addr_of_mut!(metas) as *mut spa_meta,
            datas: std::ptr::null_mut(),
        };
        let buffer_addr = std::ptr::addr_of_mut!(buffer);
        let extracted_metas = PwMetas::from_raw(&buffer_addr);
        assert_eq!(
            extracted_metas,
            PwMetas {
                header: Some(&meta_header_data),
                cursor: Some(&meta_cursor_data),
                bitmap: Some(&meta_bitmap_data)
            }
        );
    }
}
