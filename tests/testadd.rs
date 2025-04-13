#[cfg(test)]

mod tests {
    use listagram::handlers::{add::addtext, *};
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_index_get() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(None)))
                .service(addtext),
        ).await;

        let req = test::TestRequest::post().uri("/addtext").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

    }
}