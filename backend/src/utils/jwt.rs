use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub device_id: String,
}

const SECRET_KEY: &str = "my_secret_key";

pub fn generate_token(user_id: i32, device_id: String) -> String {
    let expiration = 3600;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        device_id,
    };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(SECRET_KEY.as_ref());

    encode(&header, &claims, &encoding_key).unwrap()
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(SECRET_KEY.as_ref());
    let mut validation = Validation::default();
    validation.leeway = 0;
    validation.validate_exp = true;

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
