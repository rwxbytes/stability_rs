#![allow(dead_code)]

use super::client::*;
use crate::prelude::*;

const LIST_PATH: &str = "/engines/list";

/// List all engines available to your organization/user
pub async fn get_engines() -> Result<Vec<Engine>> {
    let cb = ClientBuilder::new()?;
    let c = cb
        .method(GET)?
        .path(LIST_PATH)?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let engines = serde_json::from_slice::<Vec<Engine>>(&resp.as_ref())?;

    Ok(engines)
}

#[derive(Debug, Deserialize)]
pub struct Engine {
    description: String,
    id: String,
    name: String,
    r#type: String,
}
