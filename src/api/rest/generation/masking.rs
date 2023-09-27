use super::*;
use crate::error::*;
use crate::img_to_img::IMAGE_TO_IMAGE_PATH;

const MASKING_PATH: &str = "/masking";

#[derive(Debug, Serialize)]
pub struct Masker {
    text_prompts: Vec<TextPrompt>,
    init_image: String,
    mask_source: MaskSource,
    mask_image: String,
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

impl Masker {

    /// Selectively modify portions of an image using a mask
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use stability_rs::{masking::*, Result, StylePreset, ClipGuidancePreset};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///    let engine = "stable-inpainting-512-v2-0";
    ///
    ///    let image = MaskerBuilder::new()
    ///      .init_image_path("init_image.png")?
    ///      .mask_source(MaskSource::MaskImageBlack)?
    ///      .mask_image("black_mask_image.png")?
    ///      .text_prompt("a crab dancing", 1.0)?
    ///      .style_preset(StylePreset::FantasyArt)?
    ///      .clip_guidance_preset(ClipGuidancePreset::FastBlue)?
    ///      .build()?;
    ///
    ///    let resp = image.generate(engine).await?;
    ///
    ///    resp.artifacts.first().unwrap().save("masked_image.png").await?;
    ///
    ///    Ok(())
    /// }
    /// ```
    pub async fn generate(&self, engine: &str) -> Result<ImageResponse> {
        let data = self.to_multipart_form_data()?;

        let cb = ClientBuilder::new()?;

        let c = cb
            .method(POST)?
            .path(&format!(
                "{}/{}{}{}",
                GENERATION_PATH,
                engine,
                IMAGE_TO_IMAGE_PATH,
                MASKING_PATH,
            ))?
            .header(ACCEPT, APPLICATION_JSON)?
            .header(CONTENT_TYPE, &format!("{}{}", MULTIPART_FORM_DATA_BOUNDARY, data.boundary))?
            .build()?;


        let resp = c.send_request(Full::<Bytes>::new(data.body.into())).await?;

        let masked_img = serde_json::from_slice::<ImageResponse>(&resp.as_ref())?;

        Ok(masked_img)

    }
    fn to_multipart_form_data(
        &self,
    ) -> Result<MultipartFormData> {

        let mut multipart_form_data = MultipartFormData::new();

        multipart_form_data.add_text("mask_source", &self.mask_source.to_string().to_ascii_uppercase())?;
        multipart_form_data.add_text("cfg_scale", &self.cfg_scale.to_string())?;
        multipart_form_data.add_text("samples", &self.samples.to_string())?;
        multipart_form_data.add_text("seed", &self.seed.to_string())?;
        multipart_form_data.add_text("steps", &self.steps.to_string())?;
        multipart_form_data.add_text("style_preset", &self.style_preset.to_string())?;
        multipart_form_data.add_text(
            "clip_guidance_preset",
            &self.clip_guidance_preset.to_string().to_ascii_uppercase(),
        )?;
        if self.sampler != Sampler::None {
            multipart_form_data.add_text("sampler", &self.sampler.to_string())?;
        }

        for (key, value) in &self.extras {
            multipart_form_data.add_text(key, value)?;
        }

        for (i, text_prompt) in self.text_prompts.iter().enumerate() {
            multipart_form_data.add_text(
                &format!("text_prompts[{}][text]", i),
                &text_prompt.text,
            )?;
            multipart_form_data.add_text(
                &format!("text_prompts[{}][weight]", i),
                &text_prompt.weight.to_string(),
            )?;
        }

        multipart_form_data.add_file("init_image", &self.init_image)?;

        if self.mask_source != MaskSource::InitImageAlpha {
            multipart_form_data.add_file("mask_image", &self.mask_image)?;
        }

        multipart_form_data.end_body()?;

        Ok(multipart_form_data)
    }
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MaskSource {
    MaskImageBlack,
    MaskImageWhite,
    InitImageAlpha,
}

impl fmt::Display for MaskSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskSource::MaskImageBlack => write!(f, "mask_image_black"),
            MaskSource::MaskImageWhite => write!(f, "mask_image_white"),
            MaskSource::InitImageAlpha => write!(f, "init_image_alpha"),
        }
    }
}

#[derive(Debug, Default)]
pub struct MaskerBuilder {
    text_prompts: Vec<TextPrompt>,
    init_image: Option<String>,
    mask_source: Option<MaskSource>,
    mask_image: Option<String>,
    cfg_scale: Option<u32>,
    clip_guidance_preset: Option<ClipGuidancePreset>,
    sampler: Option<Sampler>,
    samples: Option<u32>,
    seed: Option<u32>,
    steps: Option<u32>,
    style_preset: Option<StylePreset>,
    extras: Option<HashMap<String, String>>,
}


impl MaskerBuilder {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_image_path(mut self, init_image_path: &str) -> Result<Self> {
        self.init_image = Some(init_image_path.to_string());
        Ok(self)
    }

    pub fn mask_source(mut self, mask_src: MaskSource) -> Result<Self> {
        self.mask_source = Some(mask_src);
        Ok(self)
    }

    pub fn mask_image(mut self, mask_img: &str) -> Result<Self> {
        self.mask_image = Some(mask_img.to_string());
        Ok(self)
    }

    pub fn cfg_scale(mut self, cfg_scale: u32) -> Result<Self> {
        if cfg_scale > 35 {
            return Err(Box::new(ImageBuilderError::CfgScaleGreaterThan35(
                cfg_scale,
            )));
        }

        self.cfg_scale = Some(cfg_scale);

        Ok(self)
    }

    pub fn clip_guidance_preset(mut self, clip_guidance_preset: ClipGuidancePreset) -> Result<Self> {
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

    fn extras(mut self, extras: HashMap<String, String>) -> Result<Self> {
        self.extras = Some(extras);
        Ok(self)
    }

    pub fn text_prompt(mut self, text_prompt: &str, weight: f32) -> Result<Self> {
        self.text_prompts.push(TextPrompt {
            text: text_prompt.to_string(),
            weight,
        });
        Ok(self)
    }

    pub fn build(self) -> Result<Masker> {
        if self.init_image.is_none() {
            return Err(Box::new(ImageBuilderError::InitImagePathNotSet));
        }
        if self.style_preset.is_none() {
            return Err(Box::new(ImageBuilderError::StylePresetNotSet));
        }

        if self.text_prompts.is_empty() || self.text_prompts[0].text.is_empty() {
            return Err(Box::new(ImageBuilderError::TextPromptEmpty));
        }

        if self.mask_source.is_none() {
            return Err(Box::new(ImageBuilderError::MaskSourceNotSet));
        }

        if self.mask_source == Some(MaskSource::MaskImageBlack) || self.mask_source == Some(MaskSource::MaskImageWhite) {
            if self.mask_image.is_none() {
                return Err(Box::new(ImageBuilderError::MaskImagePathNotSet));
            }
        }

        Ok(Masker {
            text_prompts: self.text_prompts,
            init_image: self.init_image.unwrap(),
            mask_source: self.mask_source.unwrap(),
            mask_image: self.mask_image.unwrap_or_default(),
            cfg_scale: self.cfg_scale.unwrap_or(7),
            clip_guidance_preset: self
                .clip_guidance_preset
                .unwrap_or(ClipGuidancePreset::None),
            sampler: self.sampler.unwrap_or(Sampler::None),
            samples: self.samples.unwrap_or(1),
            seed: self.seed.unwrap_or(0),
            steps: self.steps.unwrap_or(50),
            style_preset: self.style_preset.unwrap(),
            extras: self.extras.unwrap_or(HashMap::new()),
        })
    }

}

