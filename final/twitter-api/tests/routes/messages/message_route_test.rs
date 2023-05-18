use twitter_clone_api::routes::profiles::model::ProfileCreateJson;
use twitter_clone_api::{common_tests::actix_fixture::get_app, routes::messages::model::MessageResponder};
use twitter_clone_api::routes::messages::model::{MessagePostJson, GroupTypes};
use actix_web::{
    test,
    web::Json
};

#[tokio::test]
pub async fn test_create_and_get_message() {
    let app = get_app().await;

    println!("one");
    let create_profile_req = test::TestRequest::post().uri("/v1/profile").set_json(Json(ProfileCreateJson {
        user_name: "tester".to_string(),
        full_name: "fullName".to_string(),
        description: "description".to_string(),
        region: Some("region".to_string()),
        main_url: Some("mainUrl".to_string()),
        avatar: Vec::new()
    })).to_request();
    let profile_id = test::call_and_read_body_json::<_, _, i64>(&app, create_profile_req).await;

    println!("two");
    const MSG_BODY_STR: &str = "Testing 123";
    let create_msg_req = test::TestRequest::post().uri("/v1/msg").set_json(Json(MessagePostJson {
        user_id: profile_id,
        body: MSG_BODY_STR.clone().to_string(),
        group_type: GroupTypes::Public,
        broadcasting_msg_id: None
    })).to_request();
    let msg_id = test::call_and_read_body_json::<_, _, i64>(&app, create_msg_req).await;

    // 3. get the new message
    let get_msg_req = test::TestRequest::get().uri(&format!("/v1/msg?id={}", msg_id)).to_request();
    let get_msg_body = test::call_and_read_body_json::<_, _, Option<MessageResponder>>(&app, get_msg_req).await;

    assert!(get_msg_body.unwrap().body.unwrap().eq(MSG_BODY_STR));
}