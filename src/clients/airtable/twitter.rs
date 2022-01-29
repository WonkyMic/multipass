use crate::data;
use crate::clients;

pub async fn create(user: data::twitter::User) -> Result<String, reqwest::Error> {
    let fields = data::airtable::twitter::Fields{ id: user.id, name: user.name, username: user.username };

    let records = vec![ data::airtable::twitter::NewRecord { fields } ];

    let create_records = data::airtable::twitter::CreateRecords { records };

    let formatted_url = format!("{}/Twitter", *data::airtable::URL);

    println!("create_records {:?}", create_records);

    let client = clients::airtable::build_client().unwrap();
    let resp = client.post(formatted_url)
        .json(&create_records)
        .send()
        .await?
        .json::<data::airtable::twitter::Records>()
        .await?;

    let twitter_grid_id = resp.records.get(0).unwrap().id.clone();

    Ok(twitter_grid_id)
}

pub async fn find(id: &str) -> Result<data::airtable::twitter::Record, reqwest::Error> {    
    let formatted_url = format!("{}/users/by/username/{}", *data::airtable::URL, id);

    let client = clients::airtable::build_client().unwrap();
    let resp = client.get(formatted_url)
        .send()
        .await?
        .json::<data::airtable::twitter::Record>()
        .await?;

    Ok(resp)
}
