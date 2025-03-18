use crate::core::{RGBA8AnimatedImageData, RGBA8ImageDataType, RGBA8StaticImageData};
use anyhow::{Result, anyhow};
use image::{EncodableLayout, Rgba, RgbaImage};
use libwebp_sys::{
    MODE_RGBA, VP8_ENC_ERROR_BAD_DIMENSION, VP8_ENC_ERROR_BAD_WRITE,
    VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY, VP8_ENC_ERROR_FILE_TOO_BIG,
    VP8_ENC_ERROR_INVALID_CONFIGURATION, VP8_ENC_ERROR_LAST, VP8_ENC_ERROR_NULL_PARAMETER,
    VP8_ENC_ERROR_OUT_OF_MEMORY, VP8_ENC_ERROR_PARTITION_OVERFLOW,
    VP8_ENC_ERROR_PARTITION0_OVERFLOW, VP8_ENC_ERROR_USER_ABORT, VP8_ENC_OK,
    VP8_STATUS_BITSTREAM_ERROR, VP8_STATUS_INVALID_PARAM, VP8_STATUS_NOT_ENOUGH_DATA,
    VP8_STATUS_OK, VP8_STATUS_OUT_OF_MEMORY, VP8_STATUS_SUSPENDED, VP8_STATUS_UNSUPPORTED_FEATURE,
    VP8_STATUS_USER_ABORT, VP8StatusCode, WEBP_FF_BACKGROUND_COLOR, WEBP_FF_CANVAS_HEIGHT,
    WEBP_FF_CANVAS_WIDTH, WEBP_FF_FORMAT_FLAGS, WEBP_FF_FRAME_COUNT, WEBP_FF_LOOP_COUNT,
    WEBP_MUX_BAD_DATA, WEBP_MUX_BLEND, WEBP_MUX_DISPOSE_BACKGROUND, WEBP_MUX_INVALID_ARGUMENT,
    WEBP_MUX_MEMORY_ERROR, WEBP_MUX_NOT_ENOUGH_DATA, WEBP_MUX_NOT_FOUND, WEBP_MUX_OK,
    WEBP_PRESET_DEFAULT, WebPAnimEncoder, WebPAnimEncoderAdd, WebPAnimEncoderAssemble,
    WebPAnimEncoderDelete, WebPAnimEncoderGetError, WebPAnimEncoderNew, WebPAnimEncoderOptions,
    WebPAnimEncoderOptionsInit, WebPBitstreamFeatures, WebPConfig, WebPConfigPreset, WebPData,
    WebPDataClear, WebPDataInit, WebPDecode, WebPDecoderConfig, WebPDemux, WebPDemuxDelete,
    WebPDemuxGetFrame, WebPDemuxGetI, WebPDemuxNextFrame, WebPDemuxReleaseIterator, WebPDemuxer,
    WebPEncode, WebPEncodingError, WebPFormatFeature, WebPFreeDecBuffer, WebPGetFeatures,
    WebPInitDecoderConfig, WebPIterator, WebPMemoryWrite, WebPMemoryWriter, WebPMemoryWriterClear,
    WebPMemoryWriterInit, WebPMux, WebPMuxAnimBlend, WebPMuxAnimDispose, WebPMuxAnimParams,
    WebPMuxAssemble, WebPMuxCreate, WebPMuxDelete, WebPMuxError, WebPMuxSetAnimationParams,
    WebPPicture, WebPPictureFree, WebPPictureImportRGBA, WebPPictureInit,
};
use std::{
    ffi::{CStr, c_int, c_void},
    mem::MaybeUninit,
};

pub fn webp_encoding_errcode_to_string(error_code: WebPEncodingError) -> &'static str {
    match error_code {
        error_code if error_code == VP8_ENC_ERROR_OUT_OF_MEMORY => "out of memory",
        error_code if error_code == VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY => {
            "not enough memory to flush bits"
        }
        error_code if error_code == VP8_ENC_ERROR_NULL_PARAMETER => "NULL parameter",
        error_code if error_code == VP8_ENC_ERROR_INVALID_CONFIGURATION => "invalid configuration",
        error_code if error_code == VP8_ENC_ERROR_BAD_DIMENSION => "bad image dimensions",
        error_code if error_code == VP8_ENC_ERROR_PARTITION0_OVERFLOW => {
            "partition is bigger than 512K"
        }
        error_code if error_code == VP8_ENC_ERROR_PARTITION_OVERFLOW => {
            "partition is bigger than 16M"
        }
        error_code if error_code == VP8_ENC_ERROR_BAD_WRITE => "unable to flush bytes",
        error_code if error_code == VP8_ENC_ERROR_FILE_TOO_BIG => "file is larger than 4GiB",
        error_code if error_code == VP8_ENC_ERROR_USER_ABORT => "user aborted encoding",
        error_code if error_code == VP8_ENC_ERROR_LAST => "list terminator",
        _ => "unknown error",
    }
}

pub fn webp_decoding_errcode_to_string(error_code: VP8StatusCode) -> &'static str {
    match error_code {
        error_code if error_code == VP8_STATUS_OUT_OF_MEMORY => "out of memory",
        error_code if error_code == VP8_STATUS_INVALID_PARAM => "invalid param",
        error_code if error_code == VP8_STATUS_BITSTREAM_ERROR => "bitstream error",
        error_code if error_code == VP8_STATUS_UNSUPPORTED_FEATURE => "unsupported feature",
        error_code if error_code == VP8_STATUS_SUSPENDED => "suspended",
        error_code if error_code == VP8_STATUS_USER_ABORT => "user abort",
        error_code if error_code == VP8_STATUS_NOT_ENOUGH_DATA => "not enough data",
        _ => "unknown error",
    }
}

pub fn webp_mux_errcode_to_string(error_code: WebPMuxError) -> &'static str {
    match error_code {
        error_code if error_code == WEBP_MUX_NOT_FOUND => "mux not found",
        error_code if error_code == WEBP_MUX_INVALID_ARGUMENT => "mux invalid argument",
        error_code if error_code == WEBP_MUX_BAD_DATA => "mux bad data",
        error_code if error_code == WEBP_MUX_MEMORY_ERROR => "mux memory error",
        error_code if error_code == WEBP_MUX_NOT_ENOUGH_DATA => "mux not enoungh data",
        _ => "unknown error",
    }
}

pub fn webp_check_decoding(prefix: &str, error_code: VP8StatusCode) -> Result<()> {
    if error_code != VP8_STATUS_OK {
        return Err(anyhow!(
            "{}: {}",
            prefix,
            webp_decoding_errcode_to_string(error_code)
        ));
    }
    Ok(())
}

pub fn webp_check_encoding(prefix: &str, error_code: WebPEncodingError) -> Result<()> {
    if error_code != VP8_ENC_OK {
        return Err(anyhow!(
            "{}: {}",
            prefix,
            webp_encoding_errcode_to_string(error_code)
        ));
    }
    Ok(())
}

pub fn webp_check_muxing(prefix: &str, error_code: WebPMuxError) -> Result<()> {
    if error_code != WEBP_MUX_OK {
        return Err(anyhow!(
            "{}: {}",
            prefix,
            webp_mux_errcode_to_string(error_code)
        ));
    }
    Ok(())
}

pub struct WebPDataAdapter {
    pub webp_data: MaybeUninit<WebPData>,
    _data: Option<Vec<u8>>,
}

impl WebPDataAdapter {
    pub fn from_empty() -> Self {
        Self::from_slice(&vec![])
    }

    pub fn from_slice(data: &[u8]) -> Self {
        let cloned = Vec::from(data);
        let mut webp_data = MaybeUninit::<WebPData>::uninit();
        unsafe {
            WebPDataInit(webp_data.as_mut_ptr());

            {
                let webp_data = webp_data.assume_init_mut();
                webp_data.bytes = cloned.as_ptr();
                webp_data.size = cloned.len();
            }

            Self {
                _data: Some(cloned),
                webp_data,
            }
        }
    }

    pub fn new(webp_data: MaybeUninit<WebPData>) -> Self {
        Self {
            webp_data,
            _data: None,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const WebPData {
        self.webp_data.as_ptr()
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut WebPData {
        self.webp_data.as_mut_ptr()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        unsafe {
            let webp_data = self.webp_data.assume_init_ref();
            let size = webp_data.size;
            let mut dst = Vec::<u8>::with_capacity(size);
            std::ptr::copy(webp_data.bytes, dst.as_mut_ptr(), size);

            dst.set_len(size);

            dst
        }
    }

    pub unsafe fn mut_bytes(&mut self) -> *mut u8 {
        unsafe { self.webp_data.assume_init_mut().bytes as *mut _ }
    }

    pub unsafe fn bytes(&self) -> *const u8 {
        unsafe { self.webp_data.assume_init_ref().bytes }
    }

    pub unsafe fn size(&self) -> usize {
        unsafe { self.webp_data.assume_init_ref().size }
    }
}

impl Drop for WebPDataAdapter {
    fn drop(&mut self) {
        if self._data.is_none() {
            unsafe { WebPDataClear(self.webp_data.as_mut_ptr()) }
        };
    }
}

pub struct WebPMemoryWriterAdapter {
    wrt: MaybeUninit<WebPMemoryWriter>,
}

impl WebPMemoryWriterAdapter {
    pub fn new() -> Self {
        let mut wrt = MaybeUninit::<WebPMemoryWriter>::uninit();

        unsafe {
            WebPMemoryWriterInit(wrt.as_mut_ptr());
        }
        Self { wrt }
    }

    pub(crate) extern "C" fn memory_writer(
        data: *const u8,
        data_size: usize,
        picture: *const WebPPicture,
    ) -> c_int {
        unsafe { WebPMemoryWrite(data, data_size, picture) }
    }

    pub unsafe fn as_custom_ptr(&mut self) -> *mut c_void {
        self.wrt.as_mut_ptr() as *mut _
    }
}

impl Default for WebPMemoryWriterAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl Into<Vec<u8>> for WebPMemoryWriterAdapter {
    fn into(self) -> Vec<u8> {
        unsafe {
            let wrt = self.wrt.assume_init_ref();
            let mut dst = Vec::<u8>::with_capacity(wrt.max_size);
            std::ptr::copy(wrt.mem, dst.as_mut_ptr(), wrt.size);

            dst.set_len(wrt.size);

            dst
        }
    }
}

impl Drop for WebPMemoryWriterAdapter {
    fn drop(&mut self) {
        unsafe { WebPMemoryWriterClear(self.wrt.as_mut_ptr()) }
    }
}

pub struct WebPPictureAdapter {
    pub pic: MaybeUninit<WebPPicture>,
    pub wrt: WebPMemoryWriterAdapter,
}

impl WebPPictureAdapter {
    pub fn from_rgba8(data: &[u8], width: u32, height: u32) -> Result<Self> {
        let stride = width * 4;
        let mut pic = MaybeUninit::<WebPPicture>::uninit();

        unsafe {
            if WebPPictureInit(pic.as_mut_ptr()) == 0 {
                return Err(anyhow!(
                    "WebPPictureInit error: {}",
                    webp_encoding_errcode_to_string(pic.assume_init_ref().error_code)
                ));
            }
        }

        let mut wrt = WebPMemoryWriterAdapter::new();

        unsafe {
            let pic = pic.assume_init_mut();

            pic.use_argb = 1;

            pic.width = width as i32;
            pic.height = height as i32;
            pic.writer = Some(WebPMemoryWriterAdapter::memory_writer);
            pic.custom_ptr = wrt.as_custom_ptr();

            let len = WebPPictureImportRGBA(pic as *mut _, data.as_ptr(), stride as i32);

            if len == 0 {
                WebPPictureFree(pic as *mut _);
                return Err(anyhow!(
                    "WebPPictureImportRGBA error: {}",
                    webp_encoding_errcode_to_string(pic.error_code)
                ));
            }
        }

        Ok(Self { pic, wrt })
    }

    pub fn encode(&mut self, config: &WebPConfig) -> Result<()> {
        unsafe {
            if WebPEncode(config as *const _, self.as_mut_ptr()) == 0 {
                return Err(anyhow!("WebPEncode error: {}", self.get_error()));
            }
            Ok(())
        }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut WebPPicture {
        self.pic.as_mut_ptr()
    }

    pub unsafe fn as_ptr(&self) -> *const WebPPicture {
        self.pic.as_ptr()
    }

    pub fn get_error(&self) -> String {
        let error_code = unsafe { self.pic.assume_init_ref().error_code };
        webp_encoding_errcode_to_string(error_code).into()
    }
}

impl From<WebPPictureAdapter> for Vec<u8> {
    fn from(mut val: WebPPictureAdapter) -> Self {
        std::mem::take(&mut val.wrt).into()
    }
}

impl Drop for WebPPictureAdapter {
    fn drop(&mut self) {
        unsafe { WebPPictureFree(self.as_mut_ptr()) }
    }
}

pub struct WebPAnimEncoderAdapter(pub *mut WebPAnimEncoder);

impl WebPAnimEncoderAdapter {
    pub fn new(width: u32, height: u32, enc_options: &WebPAnimEncoderOptions) -> Self {
        unsafe {
            let enc = WebPAnimEncoderNew(
                width as i32,
                height as i32,
                enc_options as *const WebPAnimEncoderOptions,
            );
            Self(enc)
        }
    }

    pub fn assemble(&mut self) -> Result<WebPDataAdapter> {
        unsafe {
            let mut webp_data = MaybeUninit::<WebPData>::uninit();

            if WebPAnimEncoderAssemble(self.0, webp_data.as_mut_ptr()) == 0 {
                return Err(anyhow!(
                    "WebPAnimEncoderAssemble error: {}",
                    self.get_error()
                ));
            }

            Ok(WebPDataAdapter::new(webp_data))
        }
    }

    pub fn add_rgba8_frame(
        &mut self,
        frame: &[u8],
        width: u32,
        height: u32,
        timestamp_ms: u32,
        quality: f32,
    ) -> Result<()> {
        let mut config = MaybeUninit::<WebPConfig>::uninit();

        unsafe {
            WebPConfigPreset(config.as_mut_ptr(), WEBP_PRESET_DEFAULT, quality);

            let config = config.assume_init_mut();

            config.quality = quality;
            config.lossless = 0;
            config.method = 6;

            let mut frame_pic = WebPPictureAdapter::from_rgba8(frame, width, height)?;

            if WebPAnimEncoderAdd(self.0, frame_pic.as_mut_ptr(), timestamp_ms as i32, config) == 0
            {
                return Err(anyhow!("WebPAnimEncoderAdd error: {}", self.get_error()));
            }
        }
        Ok(())
    }

    pub fn get_error(&self) -> String {
        unsafe {
            CStr::from_ptr(WebPAnimEncoderGetError(self.0))
                .to_str()
                .unwrap_or("unknown error")
                .into()
        }
    }
}

impl Drop for WebPAnimEncoderAdapter {
    fn drop(&mut self) {
        unsafe { WebPAnimEncoderDelete(self.0) }
    }
}

pub struct WebPMuxAdapter<'a> {
    mux: *mut WebPMux,
    webp_data: &'a mut WebPDataAdapter,
}

impl<'a> WebPMuxAdapter<'a> {
    pub fn new(webp_data: &'a mut WebPDataAdapter) -> Self {
        unsafe {
            Self {
                mux: WebPMuxCreate(webp_data.webp_data.as_ptr(), 1),
                webp_data,
            }
        }
    }

    pub fn set_animation_params(&mut self, anim_params: &WebPMuxAnimParams) -> Result<()> {
        unsafe {
            webp_check_muxing(
                "WebPMuxSetAnimationParams error",
                WebPMuxSetAnimationParams(self.mux, anim_params as *const _),
            )
        }
    }

    pub fn assemble(&mut self) -> Result<()> {
        unsafe {
            webp_check_muxing(
                "WebPMuxAssemble error",
                WebPMuxAssemble(self.mux, self.webp_data.as_mut_ptr()),
            )
        }
    }
}

impl<'a> Drop for WebPMuxAdapter<'a> {
    fn drop(&mut self) {
        unsafe { WebPMuxDelete(self.mux) }
    }
}

pub struct WebPAnimFrameAdapter {
    pub data: RgbaImage,
    pub duration: u32,
    pub timestamp: u32,
    pub has_alpha: bool,
    pub blend_mode: WebPMuxAnimBlend,
    pub dispose_method: WebPMuxAnimDispose,
    pub frame_x: u32,
    pub frame_y: u32,
    pub frame_w: u32,
    pub frame_h: u32,
}

pub struct WebPAnimAdapter {
    pub frames: Vec<WebPAnimFrameAdapter>,
    pub width: u32,
    pub height: u32,
    pub bgcolor: u32,
    pub loop_count: u32,
    pub frame_count: u32,
}

pub struct WebPDemuxAdapter<'a> {
    pub demux: *mut WebPDemuxer,
    pub webp_data: &'a WebPDataAdapter,
}

impl<'a> WebPDemuxAdapter<'a> {
    pub fn new(webp_data: &'a WebPDataAdapter) -> Self {
        unsafe {
            let demux = WebPDemux(webp_data.as_ptr());
            Self { demux, webp_data }
        }
    }

    pub fn get_info(&self, feature: WebPFormatFeature) -> u32 {
        unsafe { WebPDemuxGetI(self.demux, feature) }
    }

    pub fn get_bg_color(&self) -> Rgba<u8> {
        let [b, g, r, a] = self.get_info(WEBP_FF_BACKGROUND_COLOR).to_be_bytes();
        Rgba([r, g, b, a])
    }

    pub fn frames_iter(&self) -> WebPAnimIteratorAdapter {
        let frame_count = self.get_info(WEBP_FF_FRAME_COUNT);

        if frame_count < 1 {
            return WebPAnimIteratorAdapter::new(self, None);
        }

        WebPAnimIteratorAdapter::new(self, Some(1))
    }
}

impl<'a> Drop for WebPDemuxAdapter<'a> {
    fn drop(&mut self) {
        unsafe { WebPDemuxDelete(self.demux) }
    }
}

#[allow(dead_code)]
pub struct WebPAnimIteratorAdapter<'a, 'b> {
    timestamp: u32,
    iter: Option<MaybeUninit<WebPIterator>>,
    demux: &'a WebPDemuxAdapter<'b>,
    width: u32,
    height: u32,
    bg_color: Rgba<u8>,
    loop_count: u32,
    frame_count: u32,
    flags: u32,
    canvas: RgbaImage,
}

impl<'a, 'b> WebPAnimIteratorAdapter<'a, 'b> {
    pub fn new(demux: &'a WebPDemuxAdapter<'b>, iter: Option<u32>) -> Self {
        let iter = if let Some(num) = iter {
            let mut iter = MaybeUninit::<WebPIterator>::uninit();
            unsafe {
                if WebPDemuxGetFrame(demux.demux, num as i32, iter.as_mut_ptr()) != 0 {
                    Some(iter)
                } else {
                    None
                }
            }
        } else {
            None
        };

        let width = demux.get_info(WEBP_FF_CANVAS_WIDTH);
        let height = demux.get_info(WEBP_FF_CANVAS_HEIGHT);
        let frame_count = demux.get_info(WEBP_FF_FRAME_COUNT);
        let loop_count = demux.get_info(WEBP_FF_LOOP_COUNT);
        let bg_color = demux.get_bg_color();
        let flags = demux.get_info(WEBP_FF_FORMAT_FLAGS);

        let canvas = RgbaImage::from_pixel(width, height, bg_color);

        Self {
            timestamp: 0,
            iter,
            demux,
            width,
            height,
            frame_count,
            loop_count,
            bg_color,
            flags,
            canvas,
        }
    }

    fn get_ani_frame(&mut self) -> Result<WebPAnimFrameAdapter> {
        unsafe {
            let iter = self.iter.as_ref().unwrap().assume_init_ref();
            let frame_x = iter.x_offset as u32;
            let frame_y = iter.y_offset as u32;
            let frame_w = iter.width as u32;
            let frame_h = iter.height as u32;
            let frame_data = iter.fragment;
            let frame_blend_mode = iter.blend_method;
            let frame_dispose_method = iter.dispose_method;

            let width = self.width;
            let height = self.height;

            let buf_size = frame_w * frame_h * 4;
            let mut buf = Vec::with_capacity(buf_size as usize);

            let mut config = MaybeUninit::<WebPDecoderConfig>::uninit();

            if WebPInitDecoderConfig(config.as_mut_ptr()) == 0 {
                return Err(anyhow!("WebPInitDecoderConfig error"));
            }

            let mut config = config.assume_init_mut();

            config.options.use_threads = 1;
            config.output.colorspace = MODE_RGBA;
            config.output.u.RGBA.rgba = buf.as_mut_ptr();
            config.output.u.RGBA.stride = (frame_w * 4) as i32;
            config.output.u.RGBA.size = buf_size as usize;
            config.output.is_external_memory = 1;

            webp_check_decoding(
                "WebPDecode error",
                WebPDecode(frame_data.bytes, frame_data.size, config as *mut _),
            )?;

            buf.set_len(buf_size as usize);

            let src = {
                let mut tmp = RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 0]));
                let buf_img = RgbaImage::from_raw(frame_w, frame_h, buf)
                    .ok_or_else(|| anyhow!("failed to get frame"))?;
                for i in 0..frame_w {
                    for j in 0..frame_h {
                        tmp.put_pixel(i + frame_x, j + frame_y, buf_img.get_pixel(i, j).clone());
                    }
                }
                tmp
            };

            let dst = &mut self.canvas;

            if frame_dispose_method == WEBP_MUX_DISPOSE_BACKGROUND {
                for i in 0..frame_w {
                    for j in 0..frame_h {
                        dst.put_pixel(i + frame_x, j + frame_y, self.bg_color);
                    }
                }
            };

            for i in 0..width {
                for j in 0..height {
                    let src_pixel = src.get_pixel(i, j);
                    let dst_pixel = dst.get_pixel_mut(i, j);

                    if frame_blend_mode == WEBP_MUX_BLEND {
                        let src_alpha = f64::from(src_pixel.0[3]);
                        let dst_alpha = f64::from(dst_pixel.0[3]);

                        let blend_alpha_f64 = src_alpha + dst_alpha * (1.0 - src_alpha / 255.0);
                        //value should be between 0 and 255, this truncates the fractional part
                        let blend_alpha = blend_alpha_f64 as u8;

                        let blend_rgb: [u8; 3] = if blend_alpha == 0 {
                            [0, 0, 0]
                        } else {
                            let mut rgb = [0u8; 3];
                            for i in 0..3 {
                                let src_f64 = f64::from(src_pixel.0[i]);
                                let dst_f64 = f64::from(dst_pixel.0[i]);

                                let val = (src_f64 * src_alpha
                                    + dst_f64 * dst_alpha * (1.0 - src_alpha / 255.0))
                                    / blend_alpha_f64;
                                //value should be between 0 and 255, this truncates the fractional part
                                rgb[i] = val as u8;
                            }

                            rgb
                        };

                        *dst_pixel = Rgba([blend_rgb[0], blend_rgb[1], blend_rgb[2], blend_alpha])
                    } else {
                        *dst_pixel = src_pixel.clone();
                    }
                }
            }

            Ok(WebPAnimFrameAdapter {
                data: dst.clone(),
                duration: iter.duration as u32,
                timestamp: self.timestamp + iter.duration as u32,
                has_alpha: iter.has_alpha != 0,
                blend_mode: iter.blend_method,
                dispose_method: iter.dispose_method,
                frame_x,
                frame_y,
                frame_w,
                frame_h,
            })
        }
    }
}

impl<'a, 'b> Iterator for WebPAnimIteratorAdapter<'a, 'b> {
    type Item = Result<WebPAnimFrameAdapter>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_some() {
            let item = self.get_ani_frame();
            unsafe {
                let iter = self.iter.as_mut().unwrap().assume_init_mut();
                self.timestamp += iter.duration as u32;
                if WebPDemuxNextFrame(iter as *mut _) == 0 {
                    WebPDemuxReleaseIterator(iter as *mut _);
                    self.iter = None;
                }
            }
            Some(item)
        } else {
            None
        }
    }
}

impl<'a, 'b> Drop for WebPAnimIteratorAdapter<'a, 'b> {
    fn drop(&mut self) {
        if self.iter.is_some() {
            unsafe {
                WebPDemuxReleaseIterator(self.iter.as_mut().unwrap().assume_init_mut() as *mut _);
            }
        }
    }
}

struct WebPDecoderAdapter<'a> {
    features: MaybeUninit<WebPBitstreamFeatures>,
    webp_data: &'a WebPDataAdapter,
}

impl<'a> WebPDecoderAdapter<'a> {
    pub fn new(webp_data: &'a WebPDataAdapter) -> Result<Self> {
        unsafe {
            let mut features = MaybeUninit::<WebPBitstreamFeatures>::uninit();

            webp_check_decoding(
                "WebPGetFeatures error",
                WebPGetFeatures(webp_data.bytes(), webp_data.size(), features.as_mut_ptr()),
            )?;

            Ok(WebPDecoderAdapter {
                features,
                webp_data,
            })
        }
    }

    pub fn has_animation(&self) -> bool {
        unsafe { self.features.assume_init_ref().has_animation != 0 }
    }

    pub fn decode_to_rgba8(&mut self, config: &mut WebPDecoderConfig) -> Result<Vec<u8>> {
        let features = unsafe { self.features.assume_init_ref() };
        let width = features.width;
        let height = features.height;
        let buf_size = width * height * 4;
        let mut buf = Vec::with_capacity(buf_size as usize);

        config.output.colorspace = MODE_RGBA;
        config.output.u.RGBA.rgba = buf.as_mut_ptr();
        config.output.u.RGBA.stride = width * 4;
        config.output.u.RGBA.size = buf_size as usize;
        config.output.is_external_memory = 1;
        config.options.use_threads = 1;

        unsafe {
            webp_check_decoding(
                "WebPDecode error",
                WebPDecode(
                    self.webp_data.bytes(),
                    self.webp_data.size(),
                    config as *mut _,
                ),
            )?;

            buf.set_len(buf_size as usize);

            WebPFreeDecBuffer(&mut config.output);

            Ok(buf)
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { self.features.assume_init_ref().width as u32 }
    }

    pub fn height(&self) -> u32 {
        unsafe { self.features.assume_init_ref().height as u32 }
    }
}

pub fn decode_webp(data: &[u8]) -> Result<RGBA8ImageDataType> {
    let webp_data = WebPDataAdapter::from_slice(data);

    let mut base_dec = WebPDecoderAdapter::new(&webp_data)?;

    let width = base_dec.width();
    let height = base_dec.height();

    if base_dec.has_animation() {
        let demux = WebPDemuxAdapter::new(&webp_data);

        let mut frames = vec![];
        let mut durations = vec![];

        for f in demux.frames_iter() {
            let f = f?;
            frames.push(f.data);
            durations.push(f.duration);
        }

        Ok(RGBA8ImageDataType::Animated(RGBA8AnimatedImageData {
            width,
            height,
            durations,
            frames,
            loop_count: demux.get_info(WEBP_FF_LOOP_COUNT),
            bg_color: demux.get_bg_color(),
        }))
    } else {
        let mut config = MaybeUninit::<WebPDecoderConfig>::uninit();

        unsafe {
            if WebPInitDecoderConfig(config.as_mut_ptr()) == 0 {
                return Err(anyhow!("WebPInitDecoderConfig error"));
            }

            let config = config.assume_init_mut();

            let data = base_dec.decode_to_rgba8(config)?;

            let img_buf = RgbaImage::from_raw(width, height, data)
                .ok_or(anyhow!("WebPDecode RgbaImage::from_raw error"))?;

            Ok(RGBA8ImageDataType::Static(RGBA8StaticImageData {
                data: img_buf,
                width,
                height,
            }))
        }
    }
}

pub fn encode_animated_webp(image_data: RGBA8AnimatedImageData, quality: f32) -> Result<Vec<u8>> {
    unsafe {
        let mut enc_options = MaybeUninit::<WebPAnimEncoderOptions>::uninit();
        WebPAnimEncoderOptionsInit(enc_options.as_mut_ptr());

        let enc_options = enc_options.assume_init_ref();

        let mut enc = WebPAnimEncoderAdapter::new(image_data.width, image_data.height, enc_options);

        let mut timestamp_ms = 0;

        for (i, frame) in image_data.frames.into_iter().enumerate() {
            let duration = image_data.durations[i];
            timestamp_ms += duration;

            let width = frame.width();
            let height = frame.height();

            enc.add_rgba8_frame(frame.as_bytes(), width, height, timestamp_ms, quality)?;
        }

        let mut webp_data = enc.assemble()?;

        let mut anim_params = MaybeUninit::<WebPMuxAnimParams>::zeroed().assume_init();

        anim_params.loop_count = image_data.loop_count as i32;
        let [r, g, b, a] = image_data.bg_color.0;
        anim_params.bgcolor = u32::from_be_bytes([b, g, r, a]);

        {
            let mut mux = WebPMuxAdapter::new(&mut webp_data);

            mux.set_animation_params(&anim_params)?;

            mux.assemble()?;
        }

        Ok(webp_data.to_vec())
    }
}

pub fn encode_static_webp(image_data: RGBA8StaticImageData, quality: f32) -> Result<Vec<u8>> {
    let width = image_data.width;
    let height = image_data.height;

    let mut config = MaybeUninit::<WebPConfig>::uninit();

    unsafe {
        WebPConfigPreset(config.as_mut_ptr(), WEBP_PRESET_DEFAULT, quality);

        let mut config = config.assume_init_mut();

        config.quality = quality;
        config.lossless = 0;
        config.method = 6;

        let mut pic = WebPPictureAdapter::from_rgba8(image_data.data.as_bytes(), width, height)?;

        pic.encode(config)?;

        Ok(pic.into())
    }
}
