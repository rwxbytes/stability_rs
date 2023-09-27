use super::*;
use crate::error::*;
use crate::img_to_img::IMAGE_TO_IMAGE_PATH;

const UPSCALE_PATH: &str = "/upscale";

#[derive(Debug, Serialize)]
pub struct Upscaler {
    image: String,
    height: u32,
    width: u32,
    text_prompts: Vec<TextPrompt>,
    cfg_scale: u32,
    seed: u32,
    steps: u32,
}

impl Upscaler {
    pub fn builder() -> UpscalerBuilder {
        UpscalerBuilder::new()
    }

    /// Upscales an image using the specified engine.
    ///
    /// # Examples
    ///
    ///
    /// ```no_run
    /// use stability_rs::{upscale::*, Result,};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///    let image = UpscalerBuilder::new()
    ///       .image("1024_image.png")?
    ///       .height(2048)?
    ///       .build()?;
    ///
    ///    let resp = image.generate(UpscaleEngine::EsrganV1X2Plus).await?;
    ///
    ///    resp.artifacts.first().unwrap().save("2048_image.png").await?;
    ///
    ///   Ok(())
    /// }
    /// ```
    pub async fn generate(self, engine: UpscaleEngine) -> Result<ImageResponse> {

        let data = self.to_multipart_form_data(engine.clone())?;


        let cb = ClientBuilder::new()?;
        let c = cb
            .method(POST)?
            .path(&format!(
                "{}/{}{}{}",
                GENERATION_PATH,
                engine.to_string(),
                IMAGE_TO_IMAGE_PATH,
                UPSCALE_PATH
            ))?
            .header(ACCEPT, APPLICATION_JSON)?
            .header(CONTENT_TYPE, &format!("{}{}", MULTIPART_FORM_DATA_BOUNDARY, data.boundary))?
            .build()?;

        let resp = c
            .send_request(Full::<Bytes>::new(data.body.into()))
            .await?;

        let upscaled_img = serde_json::from_slice::<ImageResponse>(&resp.as_ref())?;

        Ok(upscaled_img)
    }

    fn to_multipart_form_data(&self, engine: UpscaleEngine) -> io::Result<MultipartFormData> {
        let mut multipart_form_data = MultipartFormData::new();

        if engine == UpscaleEngine::StableDiffusionX4LatentUpscaler {
            for (i, prompts) in self.text_prompts.iter().enumerate() {
                multipart_form_data.add_text(
                    &format!("text_prompts[{}][text]", i),
                    &prompts.text.clone(),
                );
                multipart_form_data.add_text(
                    &format!("text_prompts[{}][weight]", i),
                    &prompts.weight.to_string(),
                );
            }
        }

        if self.height != 0 {
            multipart_form_data.add_text("height", &self.height.to_string());
        }

        if self.width != 0 {
            multipart_form_data.add_text("width", &self.width.to_string());
        }

        if engine == UpscaleEngine::StableDiffusionX4LatentUpscaler {
            multipart_form_data.add_text("cfg_scale", &self.cfg_scale.to_string());
        }

        if engine == UpscaleEngine::StableDiffusionX4LatentUpscaler {
            multipart_form_data.add_text("steps", &self.steps.to_string());
        }

        if engine == UpscaleEngine::StableDiffusionX4LatentUpscaler {
            multipart_form_data.add_text("seed", &self.seed.to_string());
        }

        multipart_form_data.add_file("image", &self.image)?;

        multipart_form_data.end_body();

        Ok(multipart_form_data)
    }
}

#[derive(Debug, Default,)]
pub struct UpscalerBuilder {
    image: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
    text_prompts: Vec<TextPrompt>,
    cfg_scale: Option<u32>,
    seed: Option<u32>,
    steps: Option<u32>,
}

impl UpscalerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image(mut self, image: &str) -> Result<Self> {
        self.image = Some(image.to_string());
        Ok(self)
    }

    pub fn height(mut self, height: u32) -> Result<Self> {
        if height < 512 {
            return Err(Box::new(ImageBuilderError::UpscaleHeightLessThan512(height)))
        }

        self.height = Some(height);
        Ok(self)
    }

    pub fn width(mut self, width: u32) -> Result<Self> {
        if width < 512 {
            return Err(Box::new(ImageBuilderError::UpscaleWidthLessThan512(width)))
        }

        self.width = Some(width);
        Ok(self)
    }

    pub fn text_prompt(mut self, text: &str, weight: f32) -> Result<Self> {
        self.text_prompts.push(TextPrompt { text: text.to_string(), weight });
        Ok(self)
    }

    pub fn cfg_scale(mut self, cfg_scale: u32) -> Result<Self> {
        if cfg_scale > 35 {
            return Err(Box::new(ImageBuilderError::CfgScaleGreaterThan35(cfg_scale)))
        }
        self.cfg_scale = Some(cfg_scale);
        Ok(self)
    }

    pub fn seed(mut self, seed: u32) -> Result<Self> {
        self.seed = Some(seed);
        Ok(self)
    }

    pub fn steps(mut self, steps: u32) -> Result<Self> {
        if steps > 150 {
            return Err(Box::new(ImageBuilderError::StepsGreaterThan150(steps)))
        }

        if steps < 10 {
            return Err(Box::new(ImageBuilderError::StepsLessThan10(steps)))
        }


        self.steps = Some(steps);
        Ok(self)
    }

    pub fn build(self) -> Result<Upscaler> {
        if self.image.is_none() {
            return Err(Box::new(ImageBuilderError::UpscaleImagePathNotSet))
        }

       if self.width.is_some() && self.height.is_some() {
           return Err(Box::new(ImageBuilderError::UpscaleWidthHeightConflict))
       }

        Ok(Upscaler {
            image: self.image.unwrap(),
            height: self.height.unwrap_or_default(),
            width: self.width.unwrap_or_default(),
            text_prompts: self.text_prompts,
            cfg_scale: self.cfg_scale.unwrap_or(7),
            seed: self.seed.unwrap_or(0),
            steps: self.steps.unwrap_or(50),
        })
    }

}

#[derive(Debug, PartialEq, Clone)]
pub enum UpscaleEngine {
    EsrganV1X2Plus,
    StableDiffusionX4LatentUpscaler,
}

impl fmt::Display for UpscaleEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpscaleEngine::EsrganV1X2Plus => write!(f, "esrgan-v1-x2plus"),
            UpscaleEngine::StableDiffusionX4LatentUpscaler => write!(f, "stable-diffusion-x4-latent-upscaler"),
        }
    }
}