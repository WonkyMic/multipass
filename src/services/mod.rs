use crate::data::twitter::TwitterProfile;
use std::io::Error;

mod twitter;

pub async fn create(name: &str, social: &str) -> Result<String, Error>{
    let twitter_grid_id = twitter::create_profile(name)
        .await.expect("Error attempting to create Twitter Profile.");

    Ok(twitter_grid_id)
}
