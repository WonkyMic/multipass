use crate::clients::twitter::get;
use crate::data::twitter;
use serde_json;

pub async fn find(name: &str) -> Result<twitter::User, reqwest::Error> {    
    let url = format!("{}/users/by/username/{}", *twitter::URL, name);

    let resp = get(&url)
        .await.unwrap()
        .as_string().unwrap();

    let json_resp: twitter::user::Response = serde_json::from_str(resp.as_str()).unwrap();
 
    Ok(json_resp.data)
}

pub async fn followers(id: &str) -> Result<Vec<twitter::User>, reqwest::Error> {
    let url = format!("{}/users/{}/followers", *twitter::URL, &id);

    let resp = get(&url)
        .await.unwrap()
        .as_string().unwrap();

    let json_resp: twitter::user::FollowResponse = serde_json::from_str(resp.as_str()).unwrap();

    Ok(json_resp.data)
}

pub async fn following(id: &str) -> Result<Vec<twitter::User>, reqwest::Error> {
    let url = format!("{}/users/{}/following", *twitter::URL, id);

    let resp = get(&url)
        .await.unwrap()
        .as_string().unwrap();

    let json_resp: twitter::user::FollowResponse = serde_json::from_str(resp.as_str()).unwrap();

    Ok(json_resp.data)

}

