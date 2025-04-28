#[cfg(test)]

mod tests {
    use listagram::handlers::{feed::feed, *};
    use actix_web::{test, web, App};
    use listagram::utils::progs::TimePolicy;

    #[actix_web::test] 
    async fn test_feed_correct_klo_2310() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::FixedTime(2025, 4, 28, 23, 10))))
                .service(feed),
            ).await;

        {
            let req = test::TestRequest::get().uri("/feed").to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }
    }
    
    #[actix_web::test] 
    async fn test_feed_correct_klo_0010() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::FixedTime(2025, 4, 28, 0, 10))))
                .service(feed),
            ).await;

        {
            let req = test::TestRequest::get().uri("/feed").to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }
    }
}