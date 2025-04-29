#[cfg(test)]

mod tests {
    use listagram::handlers::{feed::feed, *};
    use actix_web::{test, web, App};
    use listagram::utils::progs::TimePolicy;

    #[actix_web::test] 
    async fn test_feed_success_klo_2310() {

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
    async fn test_feed_success_klo_0010() {

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

    #[actix_web::test] 
    async fn test_feed_klo_2310_contains() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::FixedTime(2025, 4, 28, 23, 10))))
                .service(feed),
            ).await;

        {
            let req = test::TestRequest::get().uri("/feed").to_request();
            let bytes = test::call_and_read_body(&app, req).await;        
            let str = std::str::from_utf8(&bytes).unwrap();
            assert!(str.contains("<title>Ohjelmat: 2025-04-28 klo 21–01</title>"));
            //println!("now = '{}'", str);
        }
    }

    #[actix_web::test] 
    async fn test_feed_klo_0010_contains() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::FixedTime(2025, 4, 29, 0, 10))))
                .service(feed),
            ).await;

        {
            let req = test::TestRequest::get().uri("/feed").to_request();
            let bytes = test::call_and_read_body(&app, req).await;        
            let str = std::str::from_utf8(&bytes).unwrap();
            println!("feed = '{}'", str);
            assert!(str.contains("<title>Ohjelmat: 2025-04-28 klo 21–01</title>"));
            
        }
    }
}