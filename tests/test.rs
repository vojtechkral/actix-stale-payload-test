use std::{collections::HashMap, time::Duration};

use actix_web::rt;

#[actix_web::test]
async fn test() {
    rt::spawn(actix_stale_payload::server(8080));
    rt::time::sleep(Duration::from_millis(250)).await;

    // with Hurl:
    let hurl_script = r#"
POST http://localhost:8080/another
{ "foo": "bar", "baz": "qux" }
HTTP/* 200

POST http://localhost:8080/hello
{ "foo": "bar", "baz": "qux" }
HTTP/* 403

POST http://localhost:8080/another
{ "foo": "bar", "baz": "qux" }
HTTP/* 200
    "#;
    let hurl_file = hurl_core::parser::parse_hurl_file(hurl_script).unwrap();

    for _ in 0..10 {
        hurl::runner::run(
            &hurl_file,
            "test.hurl",
            &mut hurl::http::Client::new(None),
            &hurl::runner::RunnerOptions::default(),
            &HashMap::new(),
            &hurl::util::logger::Logger::new(false, true, "test.hurl", hurl_script),
        );
    }

    // // with Reqwest:
    // let client = reqwest::Client::builder().tcp_keepalive(Duration::from_secs(9001)).build().unwrap();

    // for _ in 1..10 {
    //     let res = client
    //         .post("http://localhost:8080/another")
    //         .body(r#"{ "foo": "baz", "bar": "qux" }"#)
    //         .send()
    //         .await
    //         .unwrap();
    //     dbg!(res);

    //     let res = client
    //         .post("http://localhost:8080/hello")
    //         .body(r#"{ "foo": "baz", "bar": "qux" }"#)
    //         .send()
    //         .await
    //         .unwrap();
    //     dbg!(res);

    //     let res = client
    //         .post("http://localhost:8080/another")
    //         .body(r#"{ "foo": "baz", "bar": "qux" }"#)
    //         .send()
    //         .await
    //         .unwrap();
    //     dbg!(res);
    // }
}
