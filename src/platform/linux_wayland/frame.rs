use crate::prelude::VideoCaptureFrame;

pub(crate) struct WaylandVideoFrame {}

impl VideoCaptureFrame for WaylandVideoFrame {
    fn size(&self) -> crate::prelude::Size {
        todo!()
    }

    fn dpi(&self) -> f64 {
        todo!()
    }

    fn duration(&self) -> std::time::Duration {
        todo!()
    }

    fn origin_time(&self) -> std::time::Duration {
        todo!()
    }

    fn capture_time(&self) -> std::time::Instant {
        todo!()
    }

    fn frame_id(&self) -> u64 {
        todo!()
    }

    fn content_rect(&self) -> crate::prelude::Rect {
        todo!()
    }
}
