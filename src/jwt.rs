use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomClaims {
    roles: Role,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Role {
    Basic,
    Subscribed,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Basic => write!(f, "Basic"),
            Role::Subscribed => write!(f, "Subscribed"),
        }
    }
}

pub fn generate_token(private_key_pem: String, issuer: String) -> Result<String, String> {
    let custom_claims = CustomClaims { roles: Role::Basic };
    let subject = "example_user_id";
    let claims = Claims::with_custom_claims(custom_claims, Duration::from_hours(1))
        .with_issuer(issuer)
        .with_subject(subject);
    match ES256KeyPair::from_pem(&private_key_pem).and_then(|kp| kp.sign(claims)) {
        Ok(token) => Ok(token),
        Err(e) => Err(format!("failed to generate token: {}", e)),
    }
}

pub fn verify_token(
    public_key_pem: String,
    issuer: String,
    token: String,
) -> Result<JWTClaims<CustomClaims>, String> {
    let mut options = VerificationOptions::default();
    options.accept_future = true;
    options.allowed_issuers = Some(HashSet::from_strings(&[issuer]));
    let res = ES256PublicKey::from_pem(&public_key_pem)
        .and_then(|pk| pk.verify_token::<CustomClaims>(&token, Some(options)));
    match res {
        Ok(claims) => Ok(claims),
        Err(e) => Err(format!("failed to verify token: {}", e)),
    }
}

pub fn strip_bearer_token(authorization: String) -> Option<String> {
    match authorization.strip_prefix("bearer ") {
        Some(token) => Some(token.into()),
        None => match authorization.strip_prefix("Bearer ") {
            Some(token) => Some(token.into()),
            None => None,
        },
    }
}
