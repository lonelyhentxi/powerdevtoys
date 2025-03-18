pub mod core;
mod utils;
pub mod webp;

use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use image::{AnimationDecoder, EncodableLayout};
use wasm_bindgen::prelude::*;
use webp::decode_webp;

use crate::core::{RGBA8AnimatedImageData, RGBA8ImageDataType, RGBA8StaticImageData};
use crate::webp::{encode_animated_webp, encode_static_webp};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

impl RGBA8ImageDataType {
    pub fn decode(extname: &str, data: &[u8]) -> Result<Self> {
        if extname.ends_with(".webp") {
            decode_webp(data)
        } else if extname.ends_with(".png") {
            let cursor = std::io::Cursor::new(data = image::codecs::png::PngDecoder::new(cursor).map_err(|e| anyhow!("PngDecoder error: {}", e))?;
            
            let frames_result = decoder.into_frames().collect_frames();
            
            match frames_result {
                Ok(frames) => RGBA8AnimatedImageData::decode(frames).map(|img| RGBA8ImageDataType::Animated(img)),
                Err(_) => RGBA8StaticImageData::decode(data).map(|img| RGBA8ImageDataType::Static(img)),
            }
            
        } else {
            unimplemented!("unsupported image format now!")
        }
    }

    pub fn ease_frames(&mut self, min_delay_ms: u32) {
        match self {
            Self::Animated(a) => a.ease_frames(min_delay_ms),
            Self::Static(a) => a.ease_frames(min_delay_ms),
        }
    }

    pub fn resize(&mut self, scale: f32) {
        match self {
            Self::Animated(a) => a.resize(scale),
            Self::Static(a) => a.resize(scale),
        }
    }

    pub fn encode(self, extname: &str, quality: f32) -> Result<Vec<u8>> {
        if extname.ends_with(".webp") {
            match self {
                RGBA8ImageDataType::Animated(ani_img) => encode_animated_webp(ani_img, quality),
                RGBA8ImageDataType::Static(st_img) => encode_static_webp(st_img, quality),
            }
        } else if extname.ends_with(".png") {
            todo!()
        } else {
            todo!()
        }
    }
}

pub fn transform_one_image_impl(
    extname: &str,
    data: &[u8],
    scale: f32,
    min_delay: u32,
    quality: f32,
) -> Result<Vec<u8>> {
    let mut image_data = RGBA8ImageDataType::decode(extname, data)?;

    image_data.ease_frames(min_delay);
    image_data.resize(scale);

    let bytes = image_data.encode(extname, quality)?;

    Ok(bytes)
}

#[wasm_bindgen]
pub fn transform_one_image(
    extname: &str,
    base64_data: &str,
    scale: f32,
    min_delay: u32,
    quality: f32,
) -> String {
    let data = general_purpose::STANDARD_NO_PAD
        .decode(base64_data)
        .expect_throw("decode base64 error");

    let transformed = transform_one_image_impl(extname, &data, scale, min_delay, quality)
        .expect_throw("transform error");

    general_purpose::STANDARD_NO_PAD.encode(transformed)
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;

    #[test]
    fn test_transform_one_image() {
        let path = Path::new("./examples/example_1/example_1.webp");
        let content = fs::read(path).unwrap();

        let img = transform_one_image_impl(".webp", &content, 0.5f32, 80, 60f32).unwrap();

        fs::write(Path::new("./examples/example_1/example_1_test.webp"), img).unwrap();
    }
}
