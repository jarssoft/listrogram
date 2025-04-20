#[cfg(test)]

mod tests {
    use listagram::handlers::{get::list, add::addtext, *};
    use actix_web::{test, web, App};
    use chrono::NaiveTime;

    #[actix_web::test] 
    async fn test_get_correct() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(None)))
                .service(list),
            ).await;

        let reqget = test::TestRequest::get().uri("/list").to_request();
        let resp = test::call_service(&app, reqget).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test] 
    async fn test_get_exact() {

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(build_appdata(None)))
                .service(list)
                .service(addtext),
            ).await;

        let payload ="15:20\nEfter Nio\n16:20\nElossa 24h\n";
        let req = test::TestRequest::post().uri("/addtext").set_payload(payload).to_request();
        test::call_service(&app, req).await;        

        let reqget = test::TestRequest::get().uri("/list").to_request();
        let bytes = test::call_and_read_body(&app, reqget).await;        
        let str = std::str::from_utf8(&bytes).unwrap();
        assert!(str.eq("[[\"15:20:00\",\"Efter Nio\"],[\"16:20:00\",\"Elossa 24h\"]]"));
        print!("resp = '{}'", str);
    }
    
}