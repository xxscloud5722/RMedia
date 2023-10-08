use image::DynamicImage;
use webp::{Decoder, Encoder, WebPImage, WebPMemory};

use crate::error::MediaError;

/// 编码WebP 图形, image 传入图形文件, quality 输出图片质量 0-100 之间.
pub fn encoded_webp(image: DynamicImage, quality: f32) -> Result<WebPMemory, MediaError> {
    let encoder: Encoder = Encoder::from_image(&image)?;
    Ok(encoder.encode(quality))
}

/// 解码WebP 图形, image 传入内存图形文件, quality 输出图片质量 0-100 之间.
pub fn decode_webp(image: Vec<u8>) -> Result<DynamicImage, MediaError> {
    let decoder: Decoder = Decoder::new(&image);
    let webp: WebPImage = decoder.decode().ok_or(MediaError::Error("decoder fail".to_string()))?;
    Ok(webp.to_image())
}