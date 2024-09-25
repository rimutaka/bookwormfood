use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use serde::Deserialize;
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

/// Returns the user email in lower case from the given JWT token, if the token is valid,
/// otherwise returns None.
/// All errors are logged inside the function.
pub fn get_email(token: &str) -> Option<String> {
    // do we have a token?
    if token.is_empty() {
        log!("No token provided");
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

    Some(claims.email.to_ascii_lowercase())
}
