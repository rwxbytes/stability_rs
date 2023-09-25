use super::*;
use crate::error::*;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

const TEXT_TO_IMAGE_PATH: &str = "/text-to-image";

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

#[derive(Debug, Serialize)]
pub struct TextToImage {
    height: u32,
    width: u32,
    text_prompts: Vec<TextPrompt>,
    cfg_scale: u32,
    clip_guidance_preset: ClipGuidancePreset,
    #[serde(skip_serializing_if = "Sampler::is_none")]
    sampler: Sampler,
    samples: u32,
    seed: u32,
    steps: u32,
    style_preset: StylePreset,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    extras: HashMap<String, String>,
}

impl TextToImage {
    fn to_json(self) -> Result<String> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }

    /// Generate an image from the text-to-image endpoint
    /// with accept header set to application/json
    ///
    /// # Example
    ///
    /// ```no_run
    /// use stability_rs::{text_to_img::*, Result, ClipGuidancePreset, Sampler, StylePreset};
    ///
    ///#[tokio::main]
    ///async fn main() -> Result<()> {
    ///    let image = TextToImageBuilder::new()
    ///        .height(1024)?
    ///        .width(1024)?
    ///        .cfg_scale(27)?
    ///        .clip_guidance_preset(ClipGuidancePreset::FastBlue)?
    ///        .sampler(Sampler::KDpmpp2sAncestral)?
    ///        .samples(2)?
    ///        .seed(0)?
    ///        .steps(75)?
    ///        .style_preset(StylePreset::DigitalArt)?
    ///        .text_prompt("A scholar tired at his desk, a raven on a bust", 1.0)?
    ///        .build()?;
    ///
    ///    let resp = image.generate("stable-diffusion-xl-1024-v1-0").await?;
    ///
    ///    for (i, image) in resp.artifacts.iter().enumerate() {
    ///        let _ = image.save(&format!("image_{}.png", i)).await?;
    ///    }
    ///
    ///    Ok(())
    ///}
    /// ```
    pub async fn generate(self, engine: &str) -> Result<ImageResponse> {
        let cb = ClientBuilder::new()?;
        let c = cb
            .method(POST)?
            .path(&format!(
                "{}/{}{}",
                GENERATION_PATH,
                engine.to_lowercase(),
                TEXT_TO_IMAGE_PATH,
            ))?
            .header(ACCEPT, APPLICATION_JSON)?
            .header(CONTENT_TYPE, APPLICATION_JSON)?
            .build()?;

        let resp = c
            .send_request(Full::<Bytes>::new(self.to_json()?.into()))
            .await?;

        let text_to_img = serde_json::from_slice::<ImageResponse>(&resp.as_ref())?;

        Ok(text_to_img)
    }

    /// Generate an image from the text-to-image endpoint
    /// with accept header set to image/png
    ///
    /// # Example
    ///
    /// ```no_run
    ///use stability_rs::{text_to_img::*, Result, ClipGuidancePreset, Sampler, StylePreset};
    ///use tokio::{fs::File, io::AsyncWriteExt};
    ///
    ///#[tokio::main]
    ///async fn main() -> Result<()> {
    ///    let image = TextToImageBuilder::new()
    ///        .height(1024)?
    ///        .width(1024)?
    ///        .cfg_scale(33)?
    ///        .clip_guidance_preset(ClipGuidancePreset::FastGreen)?
    ///        .sampler(Sampler::KLms)?
    ///        .samples(1)?
    ///        .seed(0)?
    ///        .steps(75)?
    ///        .style_preset(StylePreset::Photographic)?
    ///        .text_prompt("A crab on the moon surrounded by many stars", 1.0)?
    ///        .build()?;
    ///
    ///    let bytes = image.generate_once("stable-diffusion-xl-1024-v1-0").await?;
    ///
    ///    let mut f = File::create("crab.png").await?;
    ///    f.write_all(&bytes).await?;
    ///
    ///    Ok(())
    ///}
    /// ```

    pub async fn generate_once(self, engine: &str) -> Result<Bytes> {
        let cb = ClientBuilder::new()?;
        let c = cb
            .method(POST)?
            .path(&format!(
                "{}/{}{}",
                GENERATION_PATH,
                engine.to_lowercase(),
                TEXT_TO_IMAGE_PATH,
            ))?
            .header(ACCEPT, IMAGE_PNG)?
            .header(CONTENT_TYPE, APPLICATION_JSON)?
            .build()?;

        let resp = c
            .send_request(Full::<Bytes>::new(self.to_json()?.into()))
            .await?;

        Ok(resp)
    }
}

#[derive(Debug, Default)]
pub struct TextToImageBuilder {
    height: Option<u32>,
    width: Option<u32>,
    text_prompts: Vec<TextPrompt>,
    cfg_scale: Option<u32>,
    clip_guidance_preset: Option<ClipGuidancePreset>,
    sampler: Option<Sampler>,
    samples: Option<u32>,
    seed: Option<u32>,
    steps: Option<u32>,
    style_preset: Option<StylePreset>,
    extras: Option<HashMap<String, String>>,
}

impl TextToImageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn height(mut self, height: u32) -> Result<Self> {
        if height % 64 != 0 {
            return Err(Box::new(ImageBuilderError::HeightNotMultipleOf64(
                height,
            )));
        }

        if height < 128 {
            return Err(Box::new(ImageBuilderError::HeightLessThan128(height)));
        }

        self.height = Some(height);

        Ok(self)
    }

    pub fn width(mut self, width: u32) -> Result<Self> {
        if width % 64 != 0 {
            return Err(Box::new(ImageBuilderError::WidthNotMultipleOf64(
                width,
            )));
        }

        if width < 128 {
            return Err(Box::new(ImageBuilderError::WidthLessThan128(width)));
        }

        self.width = Some(width);

        Ok(self)
    }

    pub fn text_prompt(mut self, text_prompt: &str, weight: f32) -> Result<Self> {
        self.text_prompts.push(TextPrompt {
            text: text_prompt.to_string(),
            weight,
        });
        Ok(self)
    }

    /// How strictly the diffusion process adheres to the prompt text
    /// (higher values keep your image closer to your prompt)
    pub fn cfg_scale(mut self, cfg_scale: u32) -> Result<Self> {
        if cfg_scale > 35 {
            return Err(Box::new(ImageBuilderError::CfgScaleGreaterThan35(
                cfg_scale,
            )));
        }

        self.cfg_scale = Some(cfg_scale);

        Ok(self)
    }

    pub fn clip_guidance_preset(
        mut self,
        clip_guidance_preset: ClipGuidancePreset,
    ) -> Result<Self> {
        self.clip_guidance_preset = Some(clip_guidance_preset);
        Ok(self)
    }

    pub fn sampler(mut self, sampler: Sampler) -> Result<Self> {
        self.sampler = Some(sampler);
        Ok(self)
    }

    pub fn samples(mut self, samples: u32) -> Result<Self> {
        if samples > 10 {
            return Err(Box::new(ImageBuilderError::SamplesGreaterThan10(
                samples,
            )));
        }

        self.samples = Some(samples);

        Ok(self)
    }

    pub fn seed(mut self, seed: u32) -> Result<Self> {
        self.seed = Some(seed);
        Ok(self)
    }

    pub fn steps(mut self, steps: u32) -> Result<Self> {
        if steps > 150 {
            return Err(Box::new(ImageBuilderError::StepsGreaterThan150(
                steps,
            )));
        }

        if steps < 10 {
            return Err(Box::new(ImageBuilderError::StepsLessThan10(
                steps,
            )));
        }

        self.steps = Some(steps);

        Ok(self)
    }

    pub fn style_preset(mut self, style_preset: StylePreset) -> Result<Self> {
        self.style_preset = Some(style_preset);
        Ok(self)
    }

    pub fn extras(mut self, extras: HashMap<String, String>) -> Result<Self> {
        self.extras = Some(extras);
        Ok(self)
    }

    pub fn build(self) -> Result<TextToImage> {
        if self.style_preset.is_none() {
            return Err(Box::new(ImageBuilderError::StylePresetNotSet));
        }

        if self.text_prompts.is_empty() || self.text_prompts[0].text.is_empty() {
            return Err(Box::new(ImageBuilderError::TextPromptEmpty));
        }

        Ok(TextToImage {
            height: self.height.unwrap_or(1024),
            width: self.width.unwrap_or(1024),
            cfg_scale: self.cfg_scale.unwrap_or(7),
            clip_guidance_preset: self
                .clip_guidance_preset
                .unwrap_or(ClipGuidancePreset::None),
            sampler: self.sampler.unwrap_or(Sampler::None),
            samples: self.samples.unwrap_or(1),
            seed: self.seed.unwrap_or(0),
            steps: self.steps.unwrap_or(50),
            style_preset: self.style_preset.unwrap(),
            text_prompts: self.text_prompts,
            extras: self.extras.unwrap_or(HashMap::new()),
        })
    }
}
