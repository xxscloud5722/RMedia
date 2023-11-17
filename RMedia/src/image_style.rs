use std::collections::HashMap;
use std::fs;
use std::io::Cursor;

use image::{DynamicImage, GenericImageView};
use image::imageops::FilterType;
use url::Url;

use crate::error::MediaError;
use crate::image_webp;

/// 编辑图片 （切割、缩放）
pub fn edit(image: Option<Vec<u8>>, query_params: &str) -> Result<Option<Vec<u8>>, MediaError> {
    let url = format!("https://www.rust-lang.org?{}", query_params);
    let url = Url::parse(&url)?;
    let mut query_params_map = HashMap::new();
    for item in url.query_pairs() {
        query_params_map.insert(item.0.to_string(),
                                item.1.to_string());
    }
    let mut image = match image {
        None => {
            let file_path = query_params_map.get("from")
                .ok_or(MediaError::Error("file not found".to_string()))?;
            image::open(file_path)?
        }
        Some(value) => {
            image::load_from_memory(&value)?
        }
    };
    let image = match &query_params_map.get("args") {
        None => image,
        Some(args) => {
            let mut args = args.split("A").collect::<Vec<&str>>();
            if args.len() > 5 {
                args = args[0..5].to_owned();
            }
            for arg in args {
                if arg.starts_with("Z") {
                    let module = &arg[1..];
                    if module.starts_with("W") {
                        match module[1..].trim().parse::<u32>() {
                            Ok(value) => {
                                if image.width() < value {
                                    continue;
                                }
                                let scale = image.width() as f32 / value as f32;
                                let height = (image.height() as f32 / scale) as u32;
                                image = image.resize(value, height, FilterType::CatmullRom);
                            }
                            Err(_) => continue
                        }
                    }
                    if module.starts_with("H") {
                        match module[1..].trim().parse::<u32>() {
                            Ok(value) => {
                                if image.height() < value {
                                    continue;
                                }
                                let scale = image.height() as f32 / value as f32;
                                let width = (image.width() as f32 / scale) as u32;
                                image = image.resize(width, value, FilterType::CatmullRom);
                            }
                            Err(_) => continue
                        }
                    }
                    if module.starts_with("S") {
                        match module[1..].trim().parse::<u32>() {
                            Ok(value) => {
                                if value > 10 {
                                    continue;
                                }
                                let width = (image.width() as f32 * (value as f32 / 10f32)) as u32;
                                let height = (image.height() as f32 * (value as f32 / 10f32)) as u32;
                                image = image.resize(width, height, FilterType::CatmullRom);
                            }
                            Err(_) => continue
                        }
                    }
                    continue;
                }
                if arg.starts_with("C") && arg.len() > 2 {
                    let module = &arg[1..];
                    let values = module[2..].split("X").collect::<Vec<&str>>();
                    let width = if values.len() > 0 {
                        match values[0].trim().parse::<u32>() {
                            Ok(value) => value,
                            Err(_) => continue
                        }
                    } else {
                        0
                    };
                    let height = if values.len() > 1 {
                        match values[1].trim().parse::<u32>() {
                            Ok(value) => value,
                            Err(_) => continue
                        }
                    } else {
                        width
                    };
                    // 如果宽高读取失败
                    if width <= 0 || height <= 0 {
                        continue;
                    }
                    // 如果图片宽高不够
                    if width > image.width() || height > image.height() {
                        continue;
                    }
                    if module.starts_with("LT") {
                        let x = 0;
                        let y = 0;
                        image = image.crop(x, y, width, height);
                    }
                    if module.starts_with("RT") {
                        let x = image.width() - width;
                        let y = 0;
                        image = image.crop(x, y, width, height);
                    }
                    if module.starts_with("LB") {
                        let x = 0;
                        let y = image.height() - height;
                        image = image.crop(x, y, width, height);
                    }
                    if module.starts_with("RB") {
                        let x = image.width() - width;
                        let y = image.height() - height;
                        image = image.crop(x, y, width, height);
                    }
                    if module.starts_with("LC") {
                        let x = 0;
                        let y = image.height() / 2 - height / 2;
                        image = image.crop(x, y, width, height);
                    }
                    if module.starts_with("RC") {
                        let x = image.width() - width;
                        let y = image.height() / 2 - height / 2;
                        image = image.crop(x, y, width, height);
                    }
                    if module.starts_with("CC") {
                        let x = image.width() / 2 - width / 2;
                        let y = image.height() / 2 - height / 2;
                        image = image.crop(x, y, width, height);
                    }
                    continue;
                }
            }
            image
        }
    };

    match &query_params_map.get("type") {
        None => {
            match query_params_map.get("to") {
                None => {
                    let mut buffer = Cursor::new(Vec::new());
                    image.write_to(&mut buffer, image::ImageOutputFormat::Png)?;
                    Ok(Some(buffer.into_inner()))
                }
                Some(output_path) => {
                    image.save(output_path.to_string())?;
                    Ok(None)
                }
            }
        }
        Some(_) => {
            let result = &image_webp::encoded_webp(image, match &query_params_map.get("quality") {
                None => { 75f32 }
                Some(value) => {
                    value.parse::<f32>()?
                }
            })?.to_owned();
            match query_params_map.get("to") {
                None => {
                    Ok(Some(result.to_vec()))
                }
                Some(output_path) => {
                    fs::write(output_path, result)?;
                    Ok(None)
                }
            }
        }
    }
}

/// 获取图片元数据.
pub fn metadata(image: DynamicImage) -> HashMap<&'static str, String> {
    let mut result = HashMap::new();
    // 获取图像分辨率
    let (width, height) = image.dimensions();

    // 获取图像的颜色类型和位深度
    let color_type = image.color();


    result.insert("width", width.to_string());
    result.insert("height", height.to_string());
    result.insert("colorType", format!("{:?}", color_type));
    return result;
}

