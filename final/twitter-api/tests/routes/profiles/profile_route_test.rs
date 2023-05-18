use twitter_clone_api::routes::profiles::model::{ProfileResponder, ProfileCreateJson};
use actix_web::test;
use twitter_clone_api::common_tests::actix_fixture::get_app;

#[actix_web::test]
async fn test_create_profile_and_get_profile() {
    let app = get_app().await;
    const USER_NAME: &str = "tester";
    const FULL_NAME: &str = "John Donson";
    const DESCRIPTION: &str = "desc";
    const REGION: Option<&str> = Some("usa");
    const MAIN_URL: Option<&str> = Some("http://test.com");
    const AVATAR: Vec<u8> = Vec::new();

    let create_req = test::TestRequest::post().uri("/v1/profile").set_json(ProfileCreateJson {
        user_name: USER_NAME.to_string(),
        full_name: FULL_NAME.to_string(), 
        description: DESCRIPTION.to_string(), 
        region: Some(REGION.unwrap().to_string()),
        main_url: Some(MAIN_URL.unwrap().to_string()), 
        avatar: AVATAR
    }).to_request();
    let id = test::call_and_read_body_json::<_, _, i64>(&app, create_req).await;
    assert!(id > 0);

    let get_req = test::TestRequest::get().uri(&format!("/v1/profile/{}", id)).to_request();
    let get_res = test::call_and_read_body_json::<_, _, Option<ProfileResponder>>(&app, get_req).await;
    let profile = get_res.unwrap();
    assert!(&profile.user_name.eq(USER_NAME));
    assert!(&profile.full_name.eq(FULL_NAME));
    assert!(&profile.description.eq(DESCRIPTION));
    assert!(&profile.region.unwrap() == REGION.unwrap());
    assert!(&profile.main_url.unwrap() == MAIN_URL.unwrap());
    assert!(*&profile.avatar.to_ascii_lowercase() == *&AVATAR.to_ascii_lowercase());
}