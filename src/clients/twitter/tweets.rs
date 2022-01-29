use crate::clients;
use crate::data;

pub async fn recent(id: &str) -> Result<Vec<data::twitter::Tweet>, reqwest::Error> {
    let full_url = format!("{}/users/{}/tweets", *data::twitter::URL, id);

    let client = clients::twitter::build_client().unwrap();
    let resp = client.get(full_url)
        .send()
        .await?
        .json::<data::twitter::tweet::Response>()
        .await?;

    Ok(resp.data)
}

