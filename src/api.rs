use crate::vp8::vp8_dx_iface::vpx_codec_vp8_dx;
use crate::vpx::src::vpx_codec::vpx_codec_destroy;
use crate::vpx::src::vpx_decoder::{
    vpx_codec_ctx_t, vpx_codec_dec_init_ver, vpx_codec_decode, vpx_codec_get_frame,
    vpx_codec_iter_t, vpx_image_t, VPX_CODEC_OK, VPX_DECODER_ABI_VERSION,
};

/// A safe wrapper around the decoded image planes.
pub struct Image<'a> {
    img: *const vpx_image_t,
    _marker: core::marker::PhantomData<&'a ()>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Plane {
    Y,
    U,
    V,
    Alpha,
}

impl<'a> Image<'a> {
    /// Get the displayed width of the image.
    pub fn width(&self) -> u32 {
        unsafe { (*self.img).d_w }
    }

    /// Get the displayed height of the image.
    pub fn height(&self) -> u32 {
        unsafe { (*self.img).d_h }
    }

    /// Safely access a specific plane slice.
    pub fn plane(&self, plane: Plane) -> Option<&[u8]> {
        unsafe {
            let idx = match plane {
                Plane::Y => 0,
                Plane::U => 1,
                Plane::V => 2,
                Plane::Alpha => 3,
            };

            let ptr = (*self.img).planes[idx];
            if ptr.is_null() {
                return None;
            }

            let stride = (*self.img).stride[idx];
            if stride <= 0 {
                return None;
            }

            let height = if idx > 0 && idx < 3 {
                (*self.img).d_h >> (*self.img).y_chroma_shift
            } else {
                (*self.img).d_h
            };

            let len = (height as usize) * (stride as usize);
            Some(core::slice::from_raw_parts(ptr as *const u8, len))
        }
    }

    /// Get the bit depth of the image.
    pub fn bit_depth(&self) -> u32 {
        unsafe { (*self.img).bit_depth }
    }

    /// Compute the MD5 hash of the image planes (matching Oracle behavior).
    pub fn md5(&self) -> String {
        unsafe {
            let img = &*self.img;
            let mut context = md5::Context::new();

            // Y, U, V planes
            for plane in 0..3 {
                let data = img.planes[plane];
                let stride = img.stride[plane] as usize;
                let w = if plane == 0 { img.d_w } else { (img.d_w + 1) >> 1 };
                let h = if plane == 0 { img.d_h } else { (img.d_h + 1) >> 1 };

                for row in 0..h {
                    let row_ptr = data.add(row as usize * stride);
                    let row_data = core::slice::from_raw_parts(row_ptr, w as usize);
                    context.consume(row_data);
                }
            }
            format!("{:x}", context.compute())
        }
    }
}

/// A generic Video Decoder trait that can be implemented by different codecs
/// (e.g., VP8, VP9, AV1, H264).
pub trait Decoder {
    /// The decoded frame representation.
    type Frame<'a> where Self: 'a;
    /// The error representation.
    type Error;

    /// Initialize the decoder.
    fn init(&mut self) -> Result<(), Self::Error>;

    /// Decode a compressed frame payload.
    fn decode(&mut self, payload: &[u8]) -> Result<(), Self::Error>;

    /// Retrieve the next available decoded frame.
    fn get_frame<'a>(&'a mut self) -> Result<Option<Self::Frame<'a>>, Self::Error>;
}

/// A safe wrapper around the unsafe VP8 `vpx_codec_ctx_t` decoder lifecycle.
pub struct Vp8Decoder {
    ctx: vpx_codec_ctx_t,
    initialized: bool,
}

impl Vp8Decoder {
    pub fn new() -> Self {
        Self {
            ctx: unsafe { core::mem::zeroed() },
            initialized: false,
        }
    }
}

impl Default for Vp8Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Vp8Decoder {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                vpx_codec_destroy(
                    &raw mut self.ctx as *mut _ as *mut crate::vpx::src::vpx_codec::vpx_codec_ctx_t,
                );
            }
        }
    }
}

impl Decoder for Vp8Decoder {
    type Frame<'a> = Image<'a>;
    type Error = String;

    fn init(&mut self) -> Result<(), Self::Error> {
        let threads = std::env::var("CRABVPX_THREADS")
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        let cfg = crate::vpx::src::vpx_decoder::vpx_codec_dec_cfg {
            threads,
            w: 0,
            h: 0,
        };
        let res = unsafe {
            vpx_codec_dec_init_ver(
                &raw mut self.ctx,
                vpx_codec_vp8_dx() as *const _,
                &cfg as *const _ as *const _,
                0,
                VPX_DECODER_ABI_VERSION,
            )
        };
        if res == VPX_CODEC_OK {
            self.initialized = true;
            Ok(())
        } else {
            Err(format!("vpx_codec_dec_init_ver failed: {}", res))
        }
    }

    fn decode(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        if !self.initialized {
            return Err("Decoder not initialized".to_string());
        }

        let res = unsafe {
            vpx_codec_decode(
                &raw mut self.ctx,
                payload.as_ptr(),
                payload.len() as u32,
                core::ptr::null_mut(),
                0,
            )
        };

        if res == VPX_CODEC_OK {
            Ok(())
        } else {
            Err(format!("vpx_codec_decode failed: {}", res))
        }
    }

    fn get_frame<'a>(&'a mut self) -> Result<Option<Self::Frame<'a>>, Self::Error> {
        if !self.initialized {
            return Err("Decoder not initialized".to_string());
        }

        let mut iter: vpx_codec_iter_t = core::ptr::null();
        let img = unsafe { vpx_codec_get_frame(&raw mut self.ctx, &raw mut iter) };

        if img.is_null() {
            Ok(None)
        } else {
            Ok(Some(Image {
                img,
                _marker: core::marker::PhantomData,
            }))
        }
    }
}
