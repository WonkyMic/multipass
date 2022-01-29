use std::io::Error;
use crate::clients::airtable;
use crate::clients::twitter;

pub async fn create_profile(name: &str) -> Result<String, Error> {     
    let user = twitter::user::find(name)
        .await.expect("Failed find_user in service.");
    println!("-- user {:?}", user); 

    let followers = twitter::user::followers(&user.id)
        .await.expect(format!("Failed to get_followers for username={}", &user.username).as_str());
     
    let following = twitter::user::following(&user.id)
        .await.expect(format!("Failed to get_following for username={}", &user.username).as_str());
     
    let recent = twitter::tweets::recent(&user.id)
        .await.expect(format!("Failed to get_recent_tweets for username={}", &user.username).as_str()); 
     
    let twitter_grid_id = airtable::twitter::create(user).await.expect("Failed to create_user");
     
    // let profile = TwitterProfile { user, followers, following, recent }; 

    Ok(twitter_grid_id)
}
