    use super::*;
    use std::fs::File;
    use std::{fmt, io};
    use std::io::{Read, Write};
    use rand::Rng;

    const IMAGE_TO_IMAGE_PATH: &str = "/image-to-image";

    #[derive(Debug, Serialize)]
    pub struct ImageToImage {
        text_prompts: Vec<TextPrompt>,
        init_image: String,
        init_image_mode: ImageMode,
        image_strength: f32,
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

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    pub enum ImageMode {
        #[serde(rename = "image_strength")]
        ImageStrength,
        #[serde(rename = "step_schedule_*")]
        StepSchedule,
    }

    impl fmt::Display for ImageMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ImageMode::ImageStrength => write!(f, "image_strength"),
                ImageMode::StepSchedule => write!(f, "step_schedule"),
            }
        }
    }

    impl ImageToImage {

        /// Generate an image from another image
        ///
        /// # Example
        ///
        /// ```no_run
        ///use stability_rs::{img_to_img::*, Result, ClipGuidancePreset, Sampler, StylePreset,};
        ///
        ///#[tokio::main]
        ///async fn main() -> Result<()> {
        ///    let image = ImageToImageBuilder::new()
        ///        .init_image_path("init_image.png")?
        ///        .init_image_mode(ImageMode::ImageStrength)?
        ///        .image_strength(0.35)?
        ///        .cfg_scale(7)?
        ///        .clip_guidance_preset(ClipGuidancePreset::FastBlue)?
        ///        .sampler(Sampler::KDpm2Ancestral)?
        ///        .samples(3)?
        ///        .seed(0)?
        ///        .steps(20)?
        ///        .style_preset(StylePreset::FantasyArt)?
        ///        .text_prompt("A crab relaxing on a beach", 0.5)?
        ///        .text_prompt("stones", -0.9)?
        ///        .build()?;
        ///
        ///    let resp = image.generate("stable-diffusion-xl-1024-v1-0").await?;
        ///
        ///    for (i, img) in resp.artifacts.iter().enumerate() {
        ///        let _ = img.save(&format!("new_init_image_{}.png", i)).await?;
        ///    }
        ///
        ///    Ok(())
        ///}
        /// ```

        pub async fn generate(self, engine: &str) -> Result<ImageResponse> {
            let boundary = format!(
                "-----------------------------{}",
                rand::thread_rng().gen::<u64>()
            );

            let data = self.to_multipart_form_data(&boundary)?;


            let cb = ClientBuilder::new()?;
            let c = cb
                .method(POST)?
                .path(&format!(
                    "{}/{}{}",
                    GENERATION_PATH,
                    engine.to_lowercase(),
                    IMAGE_TO_IMAGE_PATH
                ))?
                .header(ACCEPT, APPLICATION_JSON)?
                .header(CONTENT_TYPE, &format!("{}{}", MULTIPART_FORM_DATA_BOUNDARY, boundary))?
                .build()?;

            let resp = c
                .send_request(Full::<Bytes>::new(data.into()))
                .await?;

            let img_to_img = serde_json::from_slice::<ImageResponse>(&resp.as_ref())?;

            Ok(img_to_img)
        }


        fn to_multipart_form_data(&self, boundary: &str) -> io::Result<Vec<u8>> {
            let mut data = Vec::new();

            for (i, prompts) in self.text_prompts.iter().enumerate() {
                write!(data, "--{}\r\n", boundary)?;
                write!(
                    data,
                    "Content-Disposition: form-data; name=\"text_prompts[{}][text]\"\r\n\r\n{}\r\n",
                    i, prompts.text
                )?;

                write!(data, "--{}\r\n", boundary)?;
                write!(
                    data,
                    "Content-Disposition: form-data; name=\"text_prompts[{}][weight]\"\r\n\r\n{}\r\n",
                    i, prompts.weight
                )?;
            }

            write!(data, "--{}\r\n", boundary)?;
            write!(
                data,
                "Content-Disposition: form-data; name=\"init_image_mode\"\r\n\r\n{}\r\n",
                &self.init_image_mode.to_string().to_ascii_uppercase()
            )?;

            if self.init_image_mode == ImageMode::ImageStrength {
                write!(data, "--{}\r\n", boundary)?;
                write!(
                    data,
                    "Content-Disposition: form-data; name=\"image_strength\"\r\n\r\n{}\r\n",
                    &self.image_strength
                )?;
            }

            write!(data, "--{}\r\n", boundary)?;
            write!(
                data,
                "Content-Disposition: form-data; name=\"cfg_scale\"\r\n\r\n{}\r\n",
                &self.cfg_scale
            )?;

            write!(data, "--{}\r\n", boundary)?;
            write!(
                data,
                "Content-Disposition: form-data; name=\"samples\"\r\n\r\n{}\r\n",
                &self.samples
            )?;

            write!(data, "--{}\r\n", boundary)?;
            write!(
                data,
                "Content-Disposition: form-data; name=\"steps\"\r\n\r\n{}\r\n",
                &self.steps
            )?;

            if self.sampler != Sampler::None {
                write!(data, "--{}\r\n", boundary)?;
                write!(
                    data,
                    "Content-Disposition: form-data; name=\"sampler\"\r\n\r\n{}\r\n",
                    &self.sampler.to_string().to_ascii_uppercase()
                )?;
            }


            write!(data, "--{}\r\n", boundary)?;
            write!(
                data,
                "Content-Disposition: form-data; name=\"clip_guidance_preset\"\r\n\r\n{}\r\n",
                &self.clip_guidance_preset.to_string().to_ascii_uppercase()
            )?;

            write!(data, "--{}\r\n", boundary)?;
            write!(
                data,
                "Content-Disposition: form-data; name=\"style_preset\"\r\n\r\n{}\r\n",
                &self.style_preset.to_string()
            )?;

            write!(data, "--{}\r\n", boundary)?;
            write!(
                data,
                "Content-Disposition: form-data; name=\"seed\"\r\n\r\n{}\r\n",
                &self.seed
            )?;

            write!(data, "--{}\r\n", boundary)?;
            write!(
                data,
                "Content-Disposition: form-data; name=\"init_image\"; filename=\"{}\"\r\n",
                self.init_image
            )?;
            write!(data, "Content-Type: image/png\r\n\r\n")?;

            let mut f = File::open(&self.init_image)?;
            f.read_to_end(&mut data)?;

            write!(data, "\r\n")?;


            write!(data, "--{}--\r\n", boundary)?;

            Ok(data)
        }
    }

    #[derive(Debug, Default)]
    pub struct ImageToImageBuilder {
        init_image: Option<String>,
        init_image_mode: Option<ImageMode>,
        image_strength: Option<f32>,
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

    impl ImageToImageBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn init_image_path(mut self, init_image_path: &str) -> Result<Self> {
            self.init_image = Some(init_image_path.to_string());
            Ok(self)
        }

        pub fn init_image_mode(mut self, init_image_mode: ImageMode) -> Result<Self> {
            self.init_image_mode = Some(init_image_mode);
            Ok(self)
        }

        pub fn image_strength(mut self, image_strength: f32) -> Result<Self> {
            self.image_strength = Some(image_strength);
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

        pub fn build(self) -> Result<ImageToImage> {
            if self.init_image.is_none() {
                return Err(Box::new(ImageBuilderError::InitImagePathNotSet));
            }
            if self.style_preset.is_none() {
                return Err(Box::new(ImageBuilderError::StylePresetNotSet));
            }

            if self.text_prompts.is_empty() || self.text_prompts[0].text.is_empty() {
                return Err(Box::new(ImageBuilderError::TextPromptEmpty));
            }

            Ok(ImageToImage {
                text_prompts: self.text_prompts,
                init_image: self.init_image.unwrap(),
                init_image_mode: self.init_image_mode.unwrap_or(ImageMode::ImageStrength),
                image_strength: self.image_strength.unwrap_or(0.0),
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