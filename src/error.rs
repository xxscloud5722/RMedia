#[derive(thiserror::Error, Debug)]
pub enum MediaError {
    #[error("An error occurred: {0}")]
    Error(String),

    #[error("An IO error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("An image error occurred: {0}")]
    JniError(#[from] jni::errors::Error),

    #[error("QRCodeError: {0}")]
    QRCodeError(#[from] fast_qr::qr::QRCodeError),

    #[error("ImageError: {0}")]
    QRCodeImageError(#[from] image::ImageError),

    #[error("ParseError: {0}")]
    ParseError(#[from] url::ParseError),

    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

}

impl From<fast_qr::convert::image::ImageError> for MediaError {
    fn from(image_error: fast_qr::convert::image::ImageError) -> Self {
        MediaError::Error(format!("{:?}", image_error))
    }
}

impl From<&str> for MediaError {
    fn from(value: &str) -> Self {
        MediaError::Error(format!("{:?}", value))
    }
}