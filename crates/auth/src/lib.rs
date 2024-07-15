use config::CONFIG;
use ct_codecs::{Base64UrlSafeNoPadding, Decoder, Encoder, Hex};
use ring::{digest, hmac};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[must_use]
pub fn test_password(pass: &[u8]) -> bool {
    let hashed_pass = digest::digest(&digest::SHA512, pass);
    let expected_pass = Hex::decode_to_vec(&CONFIG.hash, None).unwrap();

    if hashed_pass.as_ref() == expected_pass {
        return true;
    }
    false
}

#[must_use]
pub fn create_token() -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, CONFIG.secret.as_bytes());

    let current_time = SystemTime::now();
    let expiry_time = current_time + Duration::from_secs(CONFIG.expiry.into());

    let current_timestamp = current_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let expiry_timestamp = expiry_time.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let mut claims = [0_u8; 16];
    claims[0..8].copy_from_slice(&current_timestamp.to_be_bytes());
    claims[8..16].copy_from_slice(&expiry_timestamp.to_be_bytes());

    let sig = hmac::sign(&key, &claims);

    let encoded_claims = Base64UrlSafeNoPadding::encode_to_string(claims).unwrap();
    let encoded_sig = Base64UrlSafeNoPadding::encode_to_string(sig).unwrap();

    format!("dpdashboard.{encoded_claims}.{encoded_sig}")
}

#[must_use]
pub fn verify_token(token: &str) -> bool {
    let key = hmac::Key::new(hmac::HMAC_SHA256, CONFIG.secret.as_bytes());

    // Until key is verified, anything that can fail indicates a possible bad/malformed key
    let token_parts = token.split('.').collect::<Vec<_>>();
    let [ident, encoded_claims, encoded_sig] = token_parts[..] else {
        return false;
    };

    if ident != "dpdashboard" {
        return false;
    }

    let Ok(claims) = Base64UrlSafeNoPadding::decode_to_vec(encoded_claims, None) else {
        return false;
    };
    let Ok(sig) = Base64UrlSafeNoPadding::decode_to_vec(encoded_sig, None) else {
        return false;
    };

    if hmac::verify(&key, &claims, &sig).is_err() {
        return false;
    }

    let Some(claims_nbf) = claims.first_chunk::<8>().map(|&x| u64::from_be_bytes(x)) else {
        return false;
    };

    let Some(claims_exp) = claims.last_chunk::<8>().map(|&x| u64::from_be_bytes(x)) else {
        return false;
    };

    let current_time = SystemTime::now();
    let nbf_time = UNIX_EPOCH + Duration::from_secs(claims_nbf);

    if current_time < nbf_time {
        return false;
    }

    let expiry_time = UNIX_EPOCH + Duration::from_secs(claims_exp);
    if current_time > expiry_time {
        return false;
    }

    true
}
