use crate::clients;
use crate::data;

pub async fn find(name: &str) -> Result<data::twitter::User, reqwest::Error> {    
    let full_url = format!("{}/users/by/username/{}", *data::twitter::URL, name);

    let client = clients::twitter::build_client().unwrap();
    let resp = client
        .get(full_url) 
        .send()
        .await?
        .json::<data::twitter::user::Response>()
        .await?;

    Ok(resp.data)
}

pub async fn followers(id: &str) -> Result<Vec<data::twitter::User>, reqwest::Error> {
    let full_url = format!("{}/users/{}/followers", *data::twitter::URL, &id);

    let client = clients::twitter::build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<data::twitter::user::FollowResponse>()
        .await?;

    Ok(resp.data)
}

pub async fn following(id: &str) -> Result<Vec<data::twitter::User>, reqwest::Error> {
    let full_url = format!("{}/users/{}/following", *data::twitter::URL, id);

    let client = clients::twitter::build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<data::twitter::user::FollowResponse>()
        .await?;

    Ok(resp.data)
}

