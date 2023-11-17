use std::fs;
use std::io::Cursor;
use std::ptr::null_mut;

use jni::JNIEnv;
use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jbyteArray, jfloat, jstring};

use crate::error::MediaError;

mod image_webp;
mod error;
mod image_qr_code;
mod image_style;


/// 读取JVM 内存的数组.
fn get_vec(env: &JNIEnv, from: JByteArray) -> Result<Option<Vec<u8>>, MediaError> {
    if from.is_null() {
        return Ok(None);
    }
    let array_length = if from.is_null() { 0 } else { env.get_array_length(&from)? };
    if array_length > 0 {
        let mut buffer: Vec<i8> = vec![0; array_length as usize];
        env.get_byte_array_region(from, 0, &mut buffer)?;
        Ok(Some(buffer.iter().map(|&i| i as u8).collect()))
    } else {
        Ok(None)
    }
}

/// 读取JVM 内存的字符串.
fn get_string(env: &mut JNIEnv, from: JString) -> Result<String, MediaError> {
    Ok(env.get_string(&from)?.into())
}

/// 编码通过路径.
#[no_mangle]
pub extern "C" fn Java_com_media_image_WebP_encoded(mut env: JNIEnv, _class: JClass,
                                                    from: JString, to: JString, quality: jfloat) {
    match encoded(&mut env, from, to, quality) {
        Err(message) => {
            let message = message.to_string();
            env.throw(&*message).expect("webp encoded system error");
        }
        _ => {}
    }
}

fn encoded(mut env: &mut JNIEnv, from: JString, to: JString, quality: jfloat) -> Result<(), MediaError> {
    // 读取参数
    let from: String = get_string(&mut env, from)?;
    let to: String = get_string(&mut env, to)?;

    // 文件信息不能大于 20MB
    let metadata = fs::metadata(&from)?;
    if metadata.len() > 1 {
        return Err(MediaError::Error("File exceeds 20MB".to_owned()));
    }

    // 读取源文件
    let image = image::open(&from)?;

    // 执行压缩
    let result = image_webp::encoded_webp(image, quality)?.to_owned();

    // 输出
    fs::write(&to, &result)?;

    Ok(())
}


/// 编码通过二进制.
#[no_mangle]
pub extern "C" fn Java_com_media_image_WebP_encodedByBytes(mut env: JNIEnv, _class: JClass,
                                                           from: JByteArray, quality: jfloat) -> jbyteArray {
    match encoded_by_bytes(&mut env, from, quality) {
        Ok(value) => {
            value
        }
        Err(message) => {
            let message = message.to_string();
            env.throw(&*message).expect("webp encoded system error");
            null_mut()
        }
    }
}

fn encoded_by_bytes(env: &JNIEnv, from: JByteArray, quality: jfloat) -> Result<jbyteArray, MediaError> {
    // 读取图片
    let from_buffer = get_vec(&env, from)?
        .ok_or(MediaError::Error("from not found".to_string()))?;

    // 读取源文件
    let image = image::load_from_memory(&from_buffer)?;

    // 执行压缩
    let result = image_webp::encoded_webp(image, quality)?.to_owned();

    // 返回结果
    let output_array = env.byte_array_from_slice(&result)?;
    return Ok(output_array.into_raw());
}


/// 解码通过路径.
#[no_mangle]
pub extern "C" fn Java_com_media_image_WebP_decode(mut env: JNIEnv, _class: JClass,
                                                   from: JString, to: JString) {
    match decode(&mut env, from, to) {
        Err(message) => {
            let message = message.to_string();
            env.throw(&*message).expect("webp decode system error");
        }
        _ => {}
    }
}

fn decode(mut env: &mut JNIEnv, from: JString, to: JString) -> Result<(), MediaError> {
    // 读取参数
    let from: String = get_string(&mut env, from)?;
    let to: String = get_string(&mut env, to)?;

    // 文件信息不能大于 20MB
    let metadata = fs::metadata(&from)?;
    if metadata.len() > 1 {
        return Err(MediaError::Error("File exceeds 20MB".to_owned()));
    }

    // 读取源文件
    let image = fs::read(from)?;

    // 执行转码
    let result = image_webp::decode_webp(image)?.to_owned();

    // 输出图片
    result.save(to)?;

    Ok(())
}


/// 解码通过流.
#[no_mangle]
pub extern "C" fn Java_com_media_image_WebP_decodeByBytes(mut env: JNIEnv, _class: JClass,
                                                          from: JByteArray) -> jbyteArray {
    match decode_by_bytes(&mut env, from) {
        Ok(value) => {
            value
        }
        Err(message) => {
            let message = message.to_string();
            env.throw(&*message).expect("webp decode system error");
            null_mut()
        }
    }
}

fn decode_by_bytes(env: &mut JNIEnv, from: JByteArray) -> Result<jbyteArray, MediaError> {
    // 读取图片
    let from_buffer = get_vec(&env, from)?
        .ok_or(MediaError::Error("from not found".to_string()))?;

    // 执行转码
    let result = image_webp::decode_webp(from_buffer)?.to_owned();

    // 返回结果
    let mut buffer = Cursor::new(Vec::new());
    result.write_to(&mut buffer, image::ImageOutputFormat::Png)?;
    let output_array = env.byte_array_from_slice(buffer.get_ref())?;
    return Ok(output_array.into_raw());
}


/// 二维码生成.
#[no_mangle]
pub extern "C" fn Java_com_media_image_QrCode_generate(mut env: JNIEnv, _class: JClass,
                                                       query_params: JString, logo: JByteArray,
                                                       poster: JByteArray) -> jbyteArray {
    match qr_code_generate(&mut env, query_params, logo, poster) {
        Ok(value) => {
            value
        }
        Err(message) => {
            let message = message.to_string();
            env.throw(&*message).expect("qrCode system error");
            null_mut()
        }
    }
}

fn qr_code_generate(mut env: &mut JNIEnv, query_params: JString, logo: JByteArray, poster: JByteArray) -> Result<jbyteArray, MediaError> {
    let query_params = get_string(&mut env, query_params)?;
    let logo = get_vec(&env, logo)?;
    let poster = get_vec(&env, poster)?;
    let qr_code = image_qr_code::new(query_params, logo, poster)?;

    // 生成图片
    let result = qr_code.generate()?;

    // 返回结果
    let mut buffer = Cursor::new(Vec::new());
    result.write_to(&mut buffer, image::ImageOutputFormat::Png)?;
    let output_array = env.byte_array_from_slice(buffer.get_ref())?;
    return Ok(output_array.into_raw());
}


#[no_mangle]
pub extern "C" fn Java_com_media_image_ImageStyle_handle(mut env: JNIEnv, _class: JClass,
                                                         image: JByteArray, query_params: JString) -> jbyteArray {
    match image_style_handle(&mut env, image, query_params) {
        Ok(value) => {
            value
        }
        Err(message) => {
            let message = message.to_string();
            env.throw(&*message).expect("image style system error");
            null_mut()
        }
    }
}

fn image_style_handle(mut env: &mut JNIEnv, image: JByteArray, query_params: JString) -> Result<jbyteArray, MediaError> {
    let query_params = get_string(&mut env, query_params)?;
    let image = get_vec(&env, image)?;

    // 生成图片
    let result = image_style::edit(image, &query_params)?;

    // 返回结果
    match result {
        None => {
            Ok(null_mut())
        }
        Some(value) => {
            let output_array = env.byte_array_from_slice(&value)?;
            Ok(output_array.into_raw())
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_com_media_image_ImageStyle_imageMetadata(mut env: JNIEnv, _class: JClass, path: JString) -> jstring {
    match image_metadata(&mut env, path) {
        Ok(value) => {
            value
        }
        Err(message) => {
            let message = message.to_string();
            env.throw(&*message).expect("image style system error");
            null_mut()
        }
    }
}

fn image_metadata(mut env: &mut JNIEnv, path: JString) -> Result<jstring, MediaError> {
    let path = get_string(&mut env, path)?;

    // 读取元信息
    let image = image::open(path)?;
    let image_info = image_style::metadata(image);
    let mut result = String::new();
    for item in image_info {
        result.push_str(item.0);
        result.push_str("=");
        result.push_str(&*item.1);
        result.push_str("&");
    }

    return Ok(env.new_string(result)?.into_raw());
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        println!("{:?}", "");
    }
}