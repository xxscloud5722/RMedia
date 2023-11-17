use std::collections::HashMap;
use std::fs;

use fast_qr::convert::Builder;
use fast_qr::convert::image::ImageBuilder;
use fast_qr::QRBuilder;
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba, RgbaImage};
use image::imageops::Lanczos3;
use url::Url;

use crate::error::MediaError;

pub struct QrCode {
    /// 二维码内容.
    pub text: String,

    /// 二维码大小.
    pub size: u32,

    /// 二维码间距.
    pub margin: u8,

    /// 二维码质量.
    pub quality: u8,

    /// Logo 文件.
    pub logo: Option<Vec<u8>>,

    /// Logo 路径.
    pub logo_path: Option<String>,

    /// Logo 大小.
    pub logo_size: Option<u32>,

    /// Logo 间距.
    pub logo_margin: Option<u32>,

    /// 海报 文件.
    pub poster: Option<Vec<u8>>,

    /// 海报 文件.
    pub poster_path: Option<String>,

    /// 二维码在海报的位置:X .
    pub poster_qr_code_x: Option<u32>,

    /// 二维码在海报的位置:Y .
    pub poster_qr_code_y: Option<u32>,
}

/// 创建QRCode 实例.
pub fn new(query_params: String, logo: Option<Vec<u8>>, poster: Option<Vec<u8>>) -> Result<QrCode, MediaError> {
    let url = format!("https://www.rust-lang.org?{}", query_params);
    let url = Url::parse(&url)?;
    let mut query_params_map = HashMap::new();
    for item in url.query_pairs() {
        query_params_map.insert(item.0.to_string(),
                                item.1.to_string());
    }

    let default_value_400 = String::from("400");
    let default_value_1 = String::from("1");
    Ok(QrCode {
        text: query_params_map.get("text").ok_or(MediaError::Error("text not found".to_string()))?.to_string(),
        size: query_params_map.get("size").unwrap_or(&default_value_400).parse::<u32>()?,
        margin: query_params_map.get("margin").unwrap_or(&default_value_1).parse::<u8>()?,
        quality: query_params_map.get("quality").unwrap_or(&default_value_1).parse::<u8>()?,
        logo: match logo {
            Some(_) => logo,
            None => {
                match query_params_map.get("logo_path") {
                    Some(path) => {
                        Some(fs::read(path)?)
                    }
                    None => None,
                }
            }
        },
        logo_path: query_params_map.get("logo_path").map(|item| { item.to_string() }),
        logo_size: match query_params_map.get("logo_size") {
            None => None,
            Some(v) => Some(v.parse::<u32>()?)
        },
        logo_margin: match query_params_map.get("logo_margin") {
            None => None,
            Some(v) => Some(v.parse::<u32>()?)
        },
        poster: match poster {
            Some(_) => poster,
            None => {
                match query_params_map.get("poster_path") {
                    Some(path) => {
                        Some(fs::read(path)?)
                    }
                    None => None,
                }
            }
        },
        poster_path: query_params_map.get("poster_path").map(|item| { item.to_string() }),
        poster_qr_code_x: match query_params_map.get("poster_qr_code_x") {
            None => None,
            Some(v) => Some(v.parse::<u32>()?)
        },
        poster_qr_code_y: match query_params_map.get("poster_qr_code_y") {
            None => None,
            Some(v) => Some(v.parse::<u32>()?)
        },
    })
}

impl QrCode {

    /// 生成二维码.
    pub fn generate(&self) -> Result<DynamicImage, MediaError> {
        let qr_code = self.qr_code_generate()?;
        let result = match &self.poster {
            None => qr_code,
            Some(poster) => {
                let mut poster = image::load_from_memory(&poster)?;
                self.copy(&mut poster, match self.poster_qr_code_x {
                    None => 0,
                    Some(value) => value
                }, match self.poster_qr_code_y {
                    None => 0,
                    Some(value) => value
                }, qr_code);
                poster
            }
        };
        Ok(result)
    }

    fn qr_code_generate(&self) -> Result<DynamicImage, MediaError> {
        // 计算图片质量
        let image_quality: u8;
        if self.quality > 5 {
            image_quality = 5;
        } else if self.quality < 1 {
            image_quality = 1
        } else {
            image_quality = self.quality;
        }

        // 二维码最大 800 最小 100
        let image_size: u32;
        if self.size > 1200 {
            image_size = 1200;
        } else if self.size < 100 {
            image_size = 100;
        } else {
            image_size = self.size;
        }


        let qr_image_size = image_size * (if self.logo.is_some() { image_quality } else { 1 }) as u32;
        let qr_image = ImageBuilder::default()
            .background_color([255, 255, 255, 255])
            .margin(self.margin as usize)
            .fit_width(qr_image_size)
            .to_bytes(&QRBuilder::new((&self.text).to_string()).build()?)?;

        let mut qr_image = image::load_from_memory(&qr_image)?;
        match &self.logo {
            None => {}
            Some(login_image) => {
                let mut logo = self.create_logo(image::load_from_memory(&login_image)?, self.logo_margin, image_quality)?;
                let logo_size = match &self.logo_size {
                    None => (qr_image_size as f32 / 3.1) as u32,
                    Some(value) => *value
                };
                logo = logo.resize(logo_size, logo_size, Lanczos3);
                let x_position = (qr_image_size - logo.width()) / 2;
                let y_position = x_position;
                self.copy(&mut qr_image, x_position, y_position, logo);
            }
        }

        // 压缩返回图片
        return Ok(qr_image.resize(image_size, image_size, Lanczos3));
    }

    fn create_logo(&self, logo: DynamicImage, logo_margin: Option<u32>, image_quality: u8) -> Result<DynamicImage, MediaError> {
        let logo_size = 500;
        let logo_border = match logo_margin {
            None => 30,
            Some(value) => value
        };
        // Logo 不管多大或者多小默认500
        let mut logo = logo.resize(logo_size, logo_size, Lanczos3);

        // 创建圆角矩形
        let rectangle_size = logo_size + logo_border * 2;
        let rectangle_width = rectangle_size * image_quality as u32;
        let rectangle_height = rectangle_width;
        // 圆角 border-radius 的半径
        let rectangle_border_radius = 60 * image_quality as u32;
        // 绘制矩形
        let mut rectangle_image = RgbaImage::new(rectangle_width, rectangle_height);
        for y in 0..rectangle_height {
            for x in 0..rectangle_width {
                let x_distance = if x < rectangle_border_radius {
                    rectangle_border_radius - x
                } else if x >= rectangle_width - rectangle_border_radius {
                    x - (rectangle_width - rectangle_border_radius)
                } else {
                    0
                };

                let y_distance = if y < rectangle_border_radius {
                    rectangle_border_radius - y
                } else if y >= rectangle_height - rectangle_border_radius {
                    y - (rectangle_height - rectangle_border_radius)
                } else {
                    0
                };

                let distance_to_corner = ((x_distance.pow(2) + y_distance.pow(2)) as f32).sqrt() as u32;

                if distance_to_corner <= rectangle_border_radius {
                    rectangle_image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                } else {
                    rectangle_image.put_pixel(x, y, Rgba([0, 0, 0, 255]));
                }
            }
        }

        // 应用抗锯齿滤镜以平滑边缘
        let mut image = if image_quality == 1 {
            DynamicImage::ImageRgba8(rectangle_image)
        } else {
            DynamicImage::ImageRgba8(rectangle_image)
                .resize(rectangle_size, rectangle_size, Lanczos3)
        };

        // 绘制Logo 圆角
        let logo_border_radius = 60;
        for y in 0..logo_size {
            for x in 0..logo_size {
                let x_distance = if x < logo_border_radius {
                    logo_border_radius - x
                } else if x >= logo_size - logo_border_radius {
                    x - (logo_size - logo_border_radius)
                } else {
                    0
                };

                let y_distance = if y < logo_border_radius {
                    logo_border_radius - y
                } else if y >= logo_size - logo_border_radius {
                    y - (logo_size - logo_border_radius)
                } else {
                    0
                };

                let distance_to_corner = ((x_distance.pow(2) + y_distance.pow(2)) as f32).sqrt() as u32;

                if distance_to_corner > logo_border_radius {
                    // 否则，将其设置为透明
                    logo.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                }
            }
        }

        // 合并图片
        self.copy(&mut image, logo_border, logo_border, logo);

        return Ok(image.resize(logo_size, logo_size, Lanczos3));
    }

    fn copy(&self, target: &mut DynamicImage, x_position: u32, y_position: u32, source: DynamicImage) {
        for (x, y, pixel) in source.pixels() {
            if x_position + x < target.width() && y + y_position < target.height() {
                if pixel[3] != 255 {
                    continue;
                }
                target.put_pixel(x + x_position, y + y_position, pixel.to_rgba());
            }
        }
    }
}