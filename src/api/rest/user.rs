#![allow(dead_code)]

use super::client::*;
use crate::prelude::*;

const ACCOUNT_PATH: &str = "/user/account";
const BALANCE_PATH: &str = "/user/balance";

/// Get information about the account associated with the provided API key
pub async fn get_user_account() -> Result<User> {
    let cb = ClientBuilder::new()?;
    let c = cb
        .method(GET)?
        .path(ACCOUNT_PATH)?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let user = serde_json::from_slice::<User>(&resp.as_ref())?;

    Ok(user)
}

#[derive(Debug, Deserialize)]
pub struct User {
    email: String,
    id: String,
    organizations: Vec<Organization>,
    profile_picture: String,
}

#[derive(Debug, Deserialize)]
struct Organization {
    id: String,
    is_default: bool,
    name: String,
    role: String,
}

/// Get the credit balance of the account/organizations associated with the API key
pub async fn get_user_balance() -> Result<Balance> {
    let cb = ClientBuilder::new()?;
    let c = cb
        .method(GET)?
        .path(BALANCE_PATH)?
        .header(CONTENT_TYPE, APPLICATION_JSON)?
        .build()?;

    let resp = c.send_request(Empty::<Bytes>::new()).await?;

    let balance = serde_json::from_slice::<Balance>(&resp.as_ref())?;

    Ok(balance)
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    credits: f64,
}
