#[cfg(test)]

mod tests {
    use listagram::handlers::{add::addtext, *};
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_addtext_post_without_payload() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(None)))
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
                .app_data(web::Data::new(build_appdata(None)))
                .service(addtext),
            ).await;

        let payload ="99:20\nEfter Nio\n16:20\nElossa 24h\n16:50\nNovosti Yle";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
    
    #[actix_web::test] 
    async fn test_addtext_post_without_last_title() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(None)))
                .service(addtext),
            ).await;

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n16:50";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test] 
    async fn test_addtext_post_wrong_order() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(None)))
                .service(addtext),
            ).await;

        let payload ="15:20\nEfter Nio\n15:00\nElossa 24h\n16:50";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test] 
    async fn test_addtext_post_too_short_title() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(None)))
                .service(addtext),
            ).await;

        let payload ="15:20\n\n15:00\nElossa 24h\n16:50";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test] 
    async fn test_addtext_post_correct() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(None)))
                .service(addtext),
            ).await;

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n16:50\nNovosti Yle";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }


            //let resp = test::call_service(& app, req).await;        
        //let resp: AppState = test::call_and_read_body_json(&app, req).await;
        
        //print!("resp = '{resp:?}'");
        ////let resp = test::call_and_read_body(&app, req).await;
        ////print!("resp = '{resp:?}'");

        /*let body = resp.take_body();
        let body = body.as_ref().unwrap();
        
        //call_and_read_body_json(&app, req).await;
        assert!(resp.status().is_success());
        */



}