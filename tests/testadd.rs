#[cfg(test)]

mod tests {
    use listagram::handlers::{add::addtext, add::addtextdate, *};
    use actix_web::{test, web, App};
    use chrono::NaiveDateTime;
    use listagram::utils::progs::TimePolicy;

    #[actix_web::test]
    async fn test_addtext_post_without_payload() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
        ).await;

        let req = test::TestRequest::post().uri("/addtext").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test] 
    async fn test_addtext_post_wrong_timestring() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
            ).await;

        let payload ="99:20\nEfter Nio\n16:20\nElossa 24h";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
    
    #[actix_web::test] 
    async fn test_addtext_post_without_last_title() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
            ).await;

        let payload ="15:20\nEfter Nio\n16:20";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test] 
    async fn test_addtext_post_wrong_order() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
            ).await;

        let payload ="15:20\nEfter Nio\n15:00\nElossa 24h";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test] 
    async fn test_addtext_post_too_short_title() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
            ).await;

        let payload ="15:20\n\n15:00\nElossa 24h";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test] 
    async fn test_addtext_post_correct() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
            ).await;

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test] 
    async fn test_addtext_response_json_is_correct() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
            ).await;

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();
 
        let resp: Vec<(NaiveDateTime, String)> = test::call_and_read_body_json(&app, req).await;
        println!("resp = '{:?}'", resp);
        assert!(resp.len()==2);
    }

    #[actix_web::test] 
    async fn test_addtext_response_is_exact() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::FixedTime(2025, 4, 28, 14, 10))))
                .service(addtext),
            ).await;

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        /*
        let resp = test::call_service(&app, req).await;        
        let bytes = actix_web::body::to_bytes(resp.into_body()).await.unwrap();
        */

        let bytes = test::call_and_read_body(&app, req).await;        
        let str = std::str::from_utf8(&bytes).unwrap();
        assert!(str.eq("[[\"2025-04-28T15:20:00\",\"Efter Nio\"],[\"2025-04-28T16:20:00\",\"Elossa 24h\"]]"));
        println!("resp = '{}'", str);
    }

    #[actix_web::test] 
    async fn test_addtextdate_response_is_exact() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtextdate),
            ).await;

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtextdate/2025-04-29").set_payload(payload).to_request();

        /*
        let resp = test::call_service(&app, req).await;        
        let bytes = actix_web::body::to_bytes(resp.into_body()).await.unwrap();
        */

        let bytes = test::call_and_read_body(&app, req).await;        
        let str = std::str::from_utf8(&bytes).unwrap();
        assert!(str.eq("[[\"2025-04-29T15:20:00\",\"Efter Nio\"],[\"2025-04-29T16:20:00\",\"Elossa 24h\"]]"));
        println!("resp = '{}'", str);
    }
    
    #[actix_web::test] 
    async fn test_addtextdate_days_must_be_added_by_order() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtextdate),
            ).await;

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtextdate/2025-04-29").set_payload(payload).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());    

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtextdate/2025-04-30").set_payload(payload).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());  

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtextdate/2025-04-28").set_payload(payload).to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }


    // adding with date and empty lines

    #[actix_web::test] 
    async fn test_addtext_date_post_correct() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
            ).await;

        let payload ="2025-05-06\n15:20\nEfter Nio\n16:20\nElossa 24h\n2025-05-07\n11:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test] 
    async fn test_addtext_with_empty_lines_post_correct() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
            ).await;

        let payload ="2025-05-06\n\n15:20\nEfter Nio\n16:20\nElossa 24h\n\n2025-05-07\n\n11:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test] 
    async fn test_addtext_date_response_is_exact() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(TimePolicy::Naive())))
                .service(addtext),
            ).await;

        let payload ="2025-05-05\n\n15:20\nEfter Nio\n16:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let bytes = test::call_and_read_body(&app, req).await;        
        let str = std::str::from_utf8(&bytes).unwrap();
        println!("test_addtext_date_response_is_exact = '{}'", str);
        assert!(str.eq("[[\"2025-05-05T15:20:00\",\"Efter Nio\"],[\"2025-05-05T16:20:00\",\"Elossa 24h\"]]"));
        
    }

}