use anyhow::Result;
use image::{Rgba, RgbaImage};

pub fn length_scale(len: u32, scale: f32) -> u32 {
    f32::round(len as f32 * scale) as u32
}

pub struct RGBA8AnimatedImageData {
    pub width: u32,
    pub height: u32,
    pub durations: Vec<u32>,
    pub frames: Vec<RgbaImage>,
    pub loop_count: u32,
    pub bg_color: Rgba<u8>,
}

impl RGBA8AnimatedImageData {
    pub fn decode(img_frames: Vec<image::Frame>) -> Result<Self> {
        let mut width = 0;
        let mut height = 0;
        let mut frames = vec![];
        let mut durations = vec![];
        for (_i, frame) in img_frames.into_iter().enumerate() {
            let duration: std::time::Duration = frame.delay().into();
            durations.push(duration.as_millis() as u32);
            let img = frame.into_buffer();
            width = img.width();
            height = img.height();
            frames.push(img);
        }
        Ok(Self {
            width,
            height,
            durations,
            frames,
            loop_count: 0,
            bg_color: Rgba([255, 255, 255, 0]),
        })
    }

    pub fn ease_frames(&mut self, min_delay_ms: u32) {
        let mut next_durations = vec![];
        let mut next_frames = vec![];

        let durations = &self.durations;
        let frames = std::mem::take(&mut self.frames);

        // 压缩帧数
        let mut prev: Option<(RgbaImage, u32)> = None;
        for (i, frame) in frames.into_iter().enumerate() {
            let duration = durations[i];
            if duration >= min_delay_ms {
                if let Some((prev_frame, prev_duration)) = prev {
                    next_durations.push(prev_duration);
                    next_frames.push(prev_frame);
                    prev = None;
                }
                next_durations.push(duration);
                next_frames.push(frame);
            } else {
                prev = match prev {
                    Some((_, prev_duration)) => {
                        next_durations.push(prev_duration + duration);
                        next_frames.push(frame);
                        None
                    }
                    None => Some((frame, duration)),
                }
            }
        }
        if let Some((last_frame, last_duration)) = prev {
            next_durations.push(last_duration);
            next_frames.push(last_frame);
        };

        self.durations = next_durations;
        self.frames = next_frames;
    }

    pub fn resize(&mut self, scale: f32) {
        let next_width = length_scale(self.width, scale);
        let next_height = length_scale(self.height, scale);

        self.frames = self
            .frames
            .iter()
            .map(|f| image::imageops::resize(f, next_width, next_height, image::imageops::Lanczos3))
            .collect();
        self.width = next_width;
        self.height = next_height;
    }
}

pub struct RGBA8StaticImageData {
    pub data: RgbaImage,
    pub width: u32,
    pub height: u32,
}

impl RGBA8StaticImageData {
    pub fn decode(data: &[u8]) -> Result<Self> {
        let img = image::load_from_memory(data)?;
        let width = img.width();
        let height = img.height();

        Ok(Self {
            data: img.into_rgba8(),
            width,
            height,
        })
    }

    pub fn ease_frames(&mut self, _min_delay_ms: u32) {}

    pub fn resize(&mut self, scale: f32) {
        let next_width = length_scale(self.width, scale);
        let next_height = length_scale(self.height, scale);

        self.data = image::imageops::resize(
            &self.data,
            next_width, 
            next_height, 
            image::imageops::Lanczos3
        );
        self.width = next_width;
        self.height = next_height;
    }
}

pub enum RGBA8ImageDataType {
    Static(RGBA8StaticImageData),
    Animated(RGBA8AnimatedImageData),
}
