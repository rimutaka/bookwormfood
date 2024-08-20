use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use lambda_runtime::Error;
use serde::Deserialize;
use tracing::info;

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

/// Returns the user email from the given JWT token, if the token is valid,
/// otherwise returns an error.
/// All errors are logged inside the function.
pub(crate) fn get_email(token: &str) -> Result<String, Error> {
    // do we have a token?
    if token.is_empty() {
        return Err(Error::from("No token provided"));
    }

    // try decoding the token
    let header = match decode_header(token) {
        Ok(v) => v,
        Err(e) => {
            info!("Token: {token}");
            info!("Error decoding header: {:?}", e);
            return Err(Error::from("Error decoding header"));
        }
    };

    // try creating a decoding key
    let decoding_key = match DecodingKey::from_rsa_components(JWK_N, JWK_E) {
        Ok(v) => v,
        Err(e) => {
            info!("Token: {token}");
            info!("Error creating decoding key: {:?}. It's a bug.", e);
            return Err(Error::from("Error (a bug) creating decoding key"));
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
            info!("Token: {token}");
            info!("Error decoding token: {:?}", e);
            return Err(Error::from("Error decoding token"));
        }
    };

    // info!("{:#?}", decoded_token);

    // check if we have an email
    if claims.email.is_empty() {
        return Err(Error::from("No email found in token"));
    }
    if !claims.email_verified {
        info!("Token: {token}");
        info!("Unverified email: {}", claims.email);
        return Err(Error::from("Unverified email"));
    }

    Ok(claims.email)
}
