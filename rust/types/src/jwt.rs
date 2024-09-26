use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[cfg(not(target_arch = "wasm32"))]
use tracing::info as log; // the alias is need to reuse the same code for WASM and Lambda

/// Logs output into browser console.
/// TODO: move it to a separate module and share it with WASM
#[cfg(target_arch = "wasm32")]
macro_rules!  log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

// these N and E values are exracted manually from JWK found at https://bookwormfood.us.auth0.com/.well-known/jwks.json
// match the key by the kid field in the header of the JWT token
const JWK_N: &str ="05Vq1snoxherLDEchQYK071vXZW3k-O5hPRcrXkQjoF1R9DFGqbFNIVwFXCrSKGveIuxWIYP3X1GqdsWlsznF8yoYXrKVM4UYutbe_FLr7S1tdLBHSK9JTvpgO-c-3qV-xgAQ6T4j-6Ob_SID7XPG2mSHQwHoH9WFTxU3Bu5my-C-dknB3JNRRn-YhLGI03EBpUiwdO9l7zybx3nQUFXCpKM3kkFcP1XFgnSVASgNsEq4TxyAMISqYN_eZtHbT-b0UtBxdlZ6BeFkykEl0g9osYu1JPALDPbIWDZUNnTyc0MYOABU7f4I-av0-60B41RjpwzkmxKQao12VQDzOqLlQ";
const JWK_E: &str = "AQAB";
// this value is ClientID in the application config on Auth0
const AUDIENCE: &str = "nqzjY0VWUu8GoDVbqyfy2yOdgkydrEaf";

#[derive(Debug, Deserialize)]
struct Claims {
    email: String,
    email_verified: bool,
}

/// Details derived from the token
pub struct User {
    /// User email extracted from the token and converted to lower case.
    pub email: String,
    /// A user ID generated from the email by hashing it with a salt value.
    /// E.g. 8cbf509d254774a13ede02ce246d39434950c93aa328407e7fef657d2bb6f737
    pub id: String,
}

/// Returns the user email in lower case from the given JWT token, if the token is valid,
/// otherwise returns None.
/// All errors are logged inside the function.
pub fn get_user_details(token: &Option<crate::IdToken>) -> Option<User> {
    let token = match token {
        Some(v) => v,
        None => {
            log!("No token provided: None");
            return None;
        }
    };

    // do we have a token?
    if token.is_empty() {
        log!("No token provided: empty");
        return None;
    }

    // try decoding the token
    let header = match decode_header(token) {
        Ok(v) => v,
        Err(e) => {
            log!("Token: {token}");
            log!("Error decoding header: {:?}", e);
            return None;
        }
    };

    // try creating a decoding key
    let decoding_key = match DecodingKey::from_rsa_components(JWK_N, JWK_E) {
        Ok(v) => v,
        Err(e) => {
            log!("Token: {token}");
            log!("Error creating decoding key: {:?}. It's a bug.", e);
            return None;
        }
    };

    // prepare the validation struct for validation as part of the decode
    let validation = {
        let mut validation = Validation::new(header.alg);
        validation.set_audience(&[AUDIENCE]);
        validation.validate_exp = true;
        validation
    };

    // decode and validate in one step
    let claims = match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(v) => v.claims,
        Err(e) => {
            log!("Token: {token}");
            log!("Error decoding token: {:?}", e);
            return None;
        }
    };

    // log!("{:#?}", decoded_token);

    // check if we have an email
    if claims.email.is_empty() {
        log!("No email found in token");
        return None;
    }
    if !claims.email_verified {
        log!("Token: {token}");
        log!("Unverified email: {}", claims.email);
        return None;
    }

    // emails must be normalized to lower case
    let email = claims.email.to_ascii_lowercase();

    // id hashing with the salt
    // do not change the salt ever without converting the existing user IDs
    // the salt is not secret
    // it is used to prevent use of rainbow tables to discover emails
    let mut hasher = Sha256::new();
    hasher.update("bookwormfood");
    hasher.update(email.clone());
    let id = hex::encode(hasher.finalize());

    Some(User { email, id })
}
