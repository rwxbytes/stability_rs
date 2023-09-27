//! # Stability RS
//!
//! An unofficial Stability API client.
//!
//! Stability-AI's API documentation: <https://platform.stability.ai/docs/api-reference>.
//!
//! # Example
//!
//! ## Text to Image
//!
//! ```no_run
//! use stability_rs::{text_to_img::*, Result, ClipGuidancePreset, Sampler, StylePreset};
//!
//!    #[tokio::main]
//!    async fn main() -> Result<()> {
//!        let image = TextToImageBuilder::new()
//!            .height(1024)?
//!            .width(1024)?
//!            .cfg_scale(27)?
//!            .clip_guidance_preset(ClipGuidancePreset::FastBlue)?
//!            .sampler(Sampler::KDpmpp2sAncestral)?
//!            .samples(2)?
//!            .seed(0)?
//!            .steps(33)?
//!            .style_preset(StylePreset::DigitalArt)?
//!            .text_prompt("A scholar tired at his desk, a raven on a bust", 1.0)?
//!            .build()?;
//!
//!        let resp = image.generate("stable-diffusion-xl-1024-v1-0").await?;
//!
//!        for (i, image) in resp.artifacts.iter().enumerate() {
//!            let _ = image.save(&format!("image_{}.png", i)).await?;
//!        }
//!
//!        Ok(())
//!    }
//!  ```
//! ### Image to Image
//!
//! ```no_run
//! use stability_rs::{img_to_img::*, Result, ClipGuidancePreset, Sampler, StylePreset,};
//!
//!    #[tokio::main]
//!    async fn main() -> Result<()> {
//!         let image = ImageToImageBuilder::new()
//!            .init_image_path("init_image.png")?
//!            .init_image_mode(ImageMode::ImageStrength)?
//!            .image_strength(0.35)?
//!            .cfg_scale(7)?
//!            .clip_guidance_preset(ClipGuidancePreset::FastBlue)?
//!            .sampler(Sampler::KDpm2Ancestral)?
//!            .samples(3)?
//!            .seed(0)?
//!            .steps(20)?
//!            .style_preset(StylePreset::FantasyArt)?
//!            .text_prompt("A crab relaxing on a beach", 0.5)?
//!            .text_prompt("stones", -0.9)?
//!            .build()?;
//!
//!         let resp = image.generate("stable-diffusion-xl-1024-v1-0").await?;
//!
//!         for (i, img) in resp.artifacts.iter().enumerate() {
//!             let _ = img.save(&format!("new_image_{}.png", i)).await?;
//!         }
//!
//!         Ok(())
//! }
//! ```
//!
//! ### Image Upscaling
//!
//! ```no_run
//! use stability_rs::{upscale::*, Result,};
//!
//!      #[tokio::main]
//!      async fn main() -> Result<()> {
//!         let image = UpscalerBuilder::new()
//!            .image("1024_image.png")?
//!            .height(2048)?
//!            .build()?;
//!
//!         let resp = image.generate(UpscaleEngine::EsrganV1X2Plus).await?;
//!
//!         resp.artifacts.first().unwrap().save("2048_image.png").await?;
//!
//!        Ok(())
//!      }
//! ```
//!
//! ### Image Masking
//!
//! ```no_run
//! use stability_rs::{masking::*, Result, StylePreset, ClipGuidancePreset};
//!
//!      #[tokio::main]
//!      async fn main() -> Result<()> {
//!         let engine = "stable-inpainting-512-v2-0";
//!
//!         let image = MaskerBuilder::new()
//!           .init_image_path("init_image.png")?
//!           .mask_source(MaskSource::MaskImageBlack)?
//!           .mask_image("black_mask_image.png")?
//!           .text_prompt("a crab dancing", 1.0)?
//!           .style_preset(StylePreset::FantasyArt)?
//!           .clip_guidance_preset(ClipGuidancePreset::FastBlue)?
//!           .build()?;
//!
//!         let resp = image.generate(engine).await?;
//!
//!         resp.artifacts.first().unwrap().save("masked_image.png").await?;
//!
//!         Ok(())
//!      }
// ```

pub use crate::api::rest::generation::*;
pub use crate::api::rest::generation;
pub use crate::api::rest::generation::text_to_img;
pub use crate::api::rest::generation::img_to_img;
pub use crate::prelude::Result;

pub mod api;
pub mod error;
pub mod prelude;
pub mod support;
