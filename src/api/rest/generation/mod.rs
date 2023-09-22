pub mod text_to_img;
pub mod img_to_img;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;
use base64::{engine::general_purpose, Engine as _};
use crate::prelude::*;
use crate::error::*;
use crate::api::rest::client::*;


const GENERATION_PATH: &str = "/generation";
pub const MULTIPART_FORM_DATA_BOUNDARY: &str = "multipart/form-data; boundary=";

#[cfg(test)]
mod tests {
    use super::*;
    use super::text_to_img::*;


    #[test]
    fn height_is_erring_when_not_a_multiple_of_64() {
        let image = TextToImageBuilder::new().height(1023).unwrap_err();
        assert_eq!(
            image.to_string(),
            "height must be a multiple of 64, but was 1023"
        );
    }

    #[test]
    fn height_is_erring_when_less_than_128() {
        let image = TextToImageBuilder::new().height(64).unwrap_err();
        assert_eq!(
            image.to_string(),
            "height must not be less than 128, but was 64"
        );
    }

    #[test]
    fn width_is_erring_when_not_a_multiple_of_64() {
        let image = TextToImageBuilder::new().width(1023).unwrap_err();
        assert_eq!(
            image.to_string(),
            "width must be a multiple of 64, but was 1023"
        );
    }

    #[test]
    fn width_is_erring_when_less_than_128() {
        let image = TextToImageBuilder::new().width(64).unwrap_err();
        assert_eq!(
            image.to_string(),
            "width must not be less than 128, but was 64"
        );
    }

    #[test]
    fn cfg_scale_is_erring_when_greater_than_35() {
        let image = TextToImageBuilder::new().cfg_scale(36).unwrap_err();
        assert_eq!(
            image.to_string(),
            "cfg_scale must be no greater than 35, but was 36"
        );
    }

    #[test]
    fn samples_is_erring_when_greater_than_10() {
        let image = TextToImageBuilder::new().samples(11).unwrap_err();
        assert_eq!(
            image.to_string(),
            "samples must be no greater than 10, but was 11"
        );
    }

    #[test]
    fn steps_is_erring_when_greater_than_150() {
        let image = TextToImageBuilder::new().steps(151).unwrap_err();
        assert_eq!(
            image.to_string(),
            "steps must be no greater than 150, but was 151"
        );
    }

    #[test]
    fn tti_build_is_erring_when_style_preset_is_not_set() {
        let image = TextToImageBuilder::new().build().unwrap_err();
        assert_eq!(image.to_string(), "a style preset must be set");
    }

    #[test]
    fn tti_build_is_erring_when_textprompt_is_empty() {
        let image = TextToImageBuilder::new()
            .style_preset(StylePreset::DigitalArt)
            .unwrap()
            .text_prompt("", 1.0)
            .unwrap()
            .build()
            .unwrap_err();
        assert_eq!(image.to_string(), "a text prompt must not be empty");
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub base64: String,
    #[serde(rename = "finishReason")]
    pub finish_reason: String,
    pub seed: u32,
}

impl Image {
    pub async fn save(&self, path: &str) -> Result<()> {
        let mut png_file = tokio::fs::File::create(path).await?;
        let mut buffer: Vec<u8> = Vec::new();
        let _dec = general_purpose::STANDARD.decode_vec(&self.base64, &mut buffer)?;
        png_file.write_all(buffer.as_mut_slice()).await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResponse {
    pub artifacts: Vec<Image>,
}


    #[derive(Debug, Deserialize, Serialize)]
    struct TextPrompt {
        text: String,
        weight: f32,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum ClipGuidancePreset {
        FastBlue,
        FastGreen,
        Simple,
        Slow,
        Slower,
        Slowest,
        None,
    }

impl fmt::Display for ClipGuidancePreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClipGuidancePreset::FastBlue => write!(f, "fast_blue"),
            ClipGuidancePreset::FastGreen => write!(f, "fast_green"),
            ClipGuidancePreset::Simple => write!(f, "simple"),
            ClipGuidancePreset::Slow => write!(f, "slow"),
            ClipGuidancePreset::Slower => write!(f, "slower"),
            ClipGuidancePreset::Slowest => write!(f, "slowest"),
            ClipGuidancePreset::None => write!(f, "none"),
        }
    }
}

    impl ClipGuidancePreset {
        pub fn is_none(&self) -> bool {
            match self {
                ClipGuidancePreset::None => true,
                _ => false,
            }
        }

}

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum StylePreset {
        #[serde(rename = "3d-model")]
        ThreeDModel,
        Anime,
        AnalogFilm,
        Cinematic,
        ComicBook,
        DigitalArt,
        Enhance,
        FantasyArt,
        Isometric,
        LineArt,
        LowPoly,
        ModelingCompound,
        NeonPunk,
        Origami,
        Photographic,
        PixelArt,
        TileTexture,
    }

impl fmt::Display for StylePreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StylePreset::ThreeDModel => write!(f, "3d-model"),
            StylePreset::Anime => write!(f, "anime"),
            StylePreset::AnalogFilm => write!(f, "analog-film"),
            StylePreset::Cinematic => write!(f, "cinematic"),
            StylePreset::ComicBook => write!(f, "comic-book"),
            StylePreset::DigitalArt => write!(f, "digital-art"),
            StylePreset::Enhance => write!(f, "enhance"),
            StylePreset::FantasyArt => write!(f, "fantasy-art"),
            StylePreset::Isometric => write!(f, "isometric"),
            StylePreset::LineArt => write!(f, "line-art"),
            StylePreset::LowPoly => write!(f, "low-poly"),
            StylePreset::ModelingCompound => write!(f, "modeling-compound"),
            StylePreset::NeonPunk => write!(f, "neon-punk"),
            StylePreset::Origami => write!(f, "origami"),
            StylePreset::Photographic => write!(f, "photographic"),
            StylePreset::PixelArt => write!(f, "pixel-art"),
            StylePreset::TileTexture => write!(f, "tile-texture"),
        }
    }

}

    #[derive(Debug,PartialEq,Deserialize, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    // Todo: Add more samplers K_DPMPP_SDE?
    pub enum Sampler {
        Ddim,
        Ddpm,
        #[serde(rename = "K_DPMPP_2M")]
        KDpmpp2m,
        #[serde(rename = "K_DPMPP_2S_ANCESTRAL")]
        KDpmpp2sAncestral,
        #[serde(rename = "K_DPMP_2")]
        KDpm2,
        #[serde(rename = "K_DPMP_2_ANCESTRAL")]
        KDpm2Ancestral,
        #[serde(rename = "K_EULER")]
        KEuler,
        #[serde(rename = "K_EULER_ANCESTRAL")]
        KEAncestral,
        #[serde(rename = "K_HEUN")]
        KHeun,
        #[serde(rename = "K_LMS")]
        KLms,
        None,
    }

impl fmt::Display for Sampler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sampler::Ddim => write!(f, "ddim"),
            Sampler::Ddpm => write!(f, "ddpm"),
            Sampler::KDpmpp2m => write!(f, "k_dpmpp_2m"),
            Sampler::KDpmpp2sAncestral => write!(f, "k_dpmpp_2s_ancestral"),
            Sampler::KDpm2 => write!(f, "k_dpm_2"),
            Sampler::KDpm2Ancestral => write!(f, "k_dpm_2_ancestral"),
            Sampler::KEuler => write!(f, "k_euler"),
            Sampler::KEAncestral => write!(f, "k_euler_ancestral"),
            Sampler::KHeun => write!(f, "k_heun"),
            Sampler::KLms => write!(f, "k_lms"),
            Sampler::None => write!(f, "none"),
        }
    }
}

    impl Sampler {
        pub fn is_none(&self) -> bool {
            match self {
                Sampler::None => true,
                _ => false,
            }
        }
    }




