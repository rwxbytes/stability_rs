use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid API response: {0}")]
    InvalidApiResponse(String),
    #[error("Client build error: {0}")]
    ClientBuildError(String),
    #[error("ClientSendRequestError: {0}")]
    ClientSendRequestError(Value),
    #[error("TextToImageBuildError: {0}")]
    TextToImageBuildError(String),
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
    #[error("a style preset must be set")]
    StylePresetNotSet,
    #[error("a text prompt must not be empty")]
    TextPromptEmpty,
    #[error("failed to read init image: {0}")]
    InitImageReadError(String),
    #[error("init image path must be set")]
    InitImagePathNotSet,
}
