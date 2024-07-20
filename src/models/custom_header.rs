use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::serde::Serialize;

// ヘッダ関連
pub struct CustomHeaders {
    _authorization: String,
    pub(crate) pharmacy_id: u32,
}

#[derive(Debug, Serialize)]
pub enum CustomHeaderError {
    MissingAuthorization,
    MissingPharmacyId,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CustomHeaders {
    type Error = CustomHeaderError;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let authorization = match request.headers().get_one("Authorization") {
            Some(authorization) => authorization.to_string(),
            None => {
                return rocket::request::Outcome::Error((
                    Status::Unauthorized,
                    CustomHeaderError::MissingAuthorization,
                ))
            }
        };

        let pharmacy_id = match request.headers().get_one("pharmacy-id") {
            Some(pharmacy_id) => pharmacy_id.parse().expect("pharmacy-id must be an integer"),
            None => {
                return rocket::request::Outcome::Error((
                    Status::BadRequest,
                    CustomHeaderError::MissingPharmacyId,
                ))
            }
        };

        rocket::request::Outcome::Success(CustomHeaders {
            _authorization: authorization,
            pharmacy_id,
        })
    }
}
