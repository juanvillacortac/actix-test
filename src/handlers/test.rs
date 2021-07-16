use actix_web::{get, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
  pub msg: String,
}

#[get("/test")]
pub async fn get_test() -> Result<HttpResponse, Error> {
  let msg = Message {
    msg: "Test route".to_string(),
  };

  let response = HttpResponse::Ok()
    .json(msg);

  Ok(response)
}
