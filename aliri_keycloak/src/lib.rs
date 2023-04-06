use std::collections::HashMap;

use aliri_extra_reqwest::AuthClient;
use itertools::Itertools;
use serde_json::{Value, json};
use uuid::Uuid;




pub enum FindUserWays {
    Username(String),
    Sub(String),
}

pub async fn get_user(client: &AuthClient, find_user_ways: &FindUserWays) -> anyhow::Result<Value> {
    match find_user_ways {
        FindUserWays::Username(username) => get_user_by_username(client, &username).await,
        FindUserWays::Sub(sub) => get_user_by_sub(client, &sub).await,
    }
}


pub async fn get_user_by_username(client: &AuthClient, username: &str) -> anyhow::Result<Value> {
    let response = client.client
    .get(format!("{}/admin/realms/{}/users/?username={}", client.host_url, client.realm, username))
    .send()
    .await?;

    let users = response.json::<Vec<Value>>().await?;

    let user = users.into_iter().at_most_one()?.unwrap();

    Ok(user)
}

pub async fn get_user_by_sub(client: &AuthClient, sub: &str) -> anyhow::Result<Value> {
    let response = client.client
    .get(format!("{}/admin/realms/{}/users/{}", client.host_url, client.realm, sub))
    .send()
    .await?;

    let user = response.json::<Value>().await?;
    Ok(user)
}


pub struct UserAttributesResult {
    pub sub: Uuid,
    pub attributes: Value
}

pub async fn get_user_attributes(client: &AuthClient, find_user_ways: &FindUserWays) -> anyhow::Result<UserAttributesResult> {
    let mut user = get_user(client, find_user_ways).await?;
    let sub = user["id"].take();
    let sub = sub.as_str().unwrap().parse().unwrap();
    Ok(UserAttributesResult { 
        sub,
        attributes: user["attributes"].take()
    })
}

pub async fn update_user_attributes(
    client: &AuthClient,
    find_user_ways: &FindUserWays,
    updated_attributes: HashMap<String, Vec<String>>
) -> anyhow::Result<()> {
    let mut user = get_user_attributes(client, find_user_ways).await?;

    if let Value::Object(ref mut map) = user.attributes {
        for (key, value) in updated_attributes {
            map.insert(key, Value::Array(value.into_iter().map(Value::String).collect()));
        }
        
    }

    let body = json!({
        "attributes": user.attributes
    });

    let response = client.client
    .put(format!("{}/admin/realms/{}/users/{}", client.host_url, client.realm, user.sub))
    .json(&body)
    .send()
    .await?;

    response.error_for_status()?;
    Ok(())
}
