use crate::prelude::AudioCaptureFrame;

pub(crate) struct LinuxAudioFrame;

impl AudioCaptureFrame for LinuxAudioFrame {
    fn sample_rate(&self) -> crate::prelude::AudioSampleRate {
        todo!()
    }

    fn channel_count(&self) -> crate::prelude::AudioChannelCount {
        todo!()
    }

    fn audio_channel_buffer(
        &mut self,
        _channel: usize,
    ) -> Result<crate::prelude::AudioChannelData<'_>, crate::prelude::AudioBufferError> {
        todo!()
    }

    fn duration(&self) -> std::time::Duration {
        todo!()
    }

    fn origin_time(&self) -> std::time::Duration {
        todo!()
    }

    fn frame_id(&self) -> u64 {
        todo!()
    }
}
