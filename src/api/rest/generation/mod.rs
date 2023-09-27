pub mod text_to_img;
pub mod img_to_img;
pub mod upscale;
pub mod masking;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use base64::{engine::general_purpose, Engine as _};
use crate::prelude::*;
use crate::error::*;
use crate::api::rest::client::*;
use rand::Rng;
use std::io::{Read, Write};
use std::fs::File;
use std::{fmt, io};


const GENERATION_PATH: &str = "/generation";
pub const MULTIPART_FORM_DATA_BOUNDARY: &str = "multipart/form-data; boundary=";


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

pub struct MultipartFormData {
    pub boundary: String,
    pub body: Vec<u8>,
}

impl MultipartFormData {
    pub fn new() -> Self {
        Self {
            boundary: format!(
                "-----------------------------{}", rand::thread_rng().gen::<u64>()),
            body: Vec::new(),
        }
    }

    pub fn add_text(&mut self, name: &str, value: &str) -> io::Result<()> {
        write!(self.body, "--{}\r\n", self.boundary)?;
        write!(self.body, "Content-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n", name, value)?;
        Ok(())
    }

    pub fn add_file(&mut self, name: &str, path: &str) -> io::Result<()> {
        if !path.contains(".") {
            return Err(io::Error::new(io::ErrorKind::Other, "Invalid file path"));
        }
        write!(self.body, "--{}\r\n", self.boundary)?;
        write!(self.body, "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n", name, path)?;
        write!(self.body, "Content-Type: image/{}\r\n\r\n", path.split_once(".").unwrap().1)?;
        let mut file = File::open(path)?;
        file.read_to_end(&mut self.body)?;
        write!(self.body, "\r\n")?;


        //write!(self.body, "--{}--\r\n", self.boundary)?;
        Ok(())
    }

    pub fn end_body(&mut self) -> io::Result<()> {
        write!(self.body, "--{}--\r\n", self.boundary)?;
        Ok(())
    }

}


