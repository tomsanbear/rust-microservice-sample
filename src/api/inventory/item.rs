use crate::authentication::auth;

use rocket::http::RawStr;

#[get("/<id>")]
pub fn get_item(_key: auth::ApiKey, id: &RawStr) -> String {
    format!("Hello, the ID you provided is: {}", id.as_str())
}
