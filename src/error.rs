use std::fmt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponseError {
    pub id: String,
    pub name: String,
    pub message: String,

}

impl fmt::Display for ApiResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}, name: {}, message: {}", self.id, self.name, self.message)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Client build error: {0}")]
    ClientBuildError(String),
    #[error("{:?}", .0)]
    ClientSendRequestError(ApiResponseError),
}

#[derive(thiserror::Error, Debug)]
pub enum ImageBuilderError {
    #[error("height must be a multiple of 64, but was {0}")]
    HeightNotMultipleOf64(u32),
    #[error("height must not be less than 128, but was {0}")]
    HeightLessThan128(u32),
    #[error("width must be a multiple of 64, but was {0}")]
    WidthNotMultipleOf64(u32),
    #[error("width must not be less than 128, but was {0}")]
    WidthLessThan128(u32),
    #[error("cfg_scale must be no greater than 35, but was {0}")]
    CfgScaleGreaterThan35(u32),
    #[error("samples must be no greater than 10, but was {0}")]
    SamplesGreaterThan10(u32),
    #[error("steps must be no greater than 150, but was {0}")]
    StepsGreaterThan150(u32),
    #[error("steps must be no less than 10, but was {0}")]
    StepsLessThan10(u32),
    #[error("a style preset must be set")]
    StylePresetNotSet,
    #[error("a text prompt must not be empty")]
    TextPromptEmpty,
    #[error("failed to read init image: {0}")]
    InitImageReadError(String),
    #[error("init image path must be set")]
    InitImagePathNotSet,
    #[error("upscale height must be greater or equal to 512, but was {0}")]
    UpscaleHeightLessThan512(u32),
    #[error("upscale width must be greater or equal to 512, but was {0}")]
    UpscaleWidthLessThan512(u32),
    #[error("upscale image path must be set")]
    UpscaleImagePathNotSet,
    #[error("only one of width or height may be specified")]
    UpscaleWidthHeightConflict,
    #[error("a mask source must be set")]
    MaskSourceNotSet,
    #[error("mask image path must be set when using a black or white mask source")]
    MaskImagePathNotSet,
}
