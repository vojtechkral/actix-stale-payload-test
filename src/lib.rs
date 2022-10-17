use std::{io, ptr};

use actix_web::{
    error, post,
    web::{Json, Payload},
    App, Error, FromRequest, HttpRequest, HttpServer,
};
use serde::{Deserialize, Serialize};

static QUITE_A_LOT: u32 = 9001;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Request {
    foo: String,
    bar: String,
}

#[post("/hello")]
async fn hello(req: HttpRequest, payload: Payload) -> Result<String, Error> {
    // Don't want to just return an error here, the payload might get
    // optimized out. Make it dependant on a volatile read:
    let quite_a_lot = unsafe { ptr::read_volatile(&QUITE_A_LOT) };
    if quite_a_lot == 9001 {
        Err(error::ErrorForbidden("pk"))
    } else {
        let res = Json::<Request>::from_request(&req, &mut payload.into_inner()).await;
        Ok(format!("{res:?}"))
    }
}

#[post("/another")]
async fn another(_req: HttpRequest, _payload: Payload) -> Result<String, Error> {
    Ok(format!("reply"))
}

pub async fn server(port: u16) -> io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(another))
        .bind(("::", port))?
        .run()
        .await
}
