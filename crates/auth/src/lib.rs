use config::CONFIG;
use ct_codecs::{Base64UrlSafeNoPadding, Decoder, Encoder, Hex};
use ed25519_compact::{Noise, PublicKey, SecretKey, Signature};
use ring::digest::{digest, SHA512};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[must_use]
pub fn test_password(pass: &str) -> bool {
    let hashed_pass = digest(&SHA512, pass.as_bytes());
    let expected_pass = Hex::decode_to_vec(&CONFIG.auth.hash, None).unwrap();

    if hashed_pass.as_ref() == expected_pass {
        return true;
    }
    false
}

#[must_use]
pub fn create_token() -> String {
    let key_file =
        std::fs::read(&CONFIG.auth.privkey_path).expect("failed to read secret key file");
    let key = SecretKey::from_der(&key_file).expect("failed to parse secret key");

    let current_time = SystemTime::now();
    let expiry_time = current_time + Duration::from_secs(CONFIG.auth.expiry);

    let current_timestamp = current_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let expiry_timestamp = expiry_time.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let mut claims = [0_u8; 16];
    claims[0..8].copy_from_slice(&current_timestamp.to_be_bytes());
    claims[8..16].copy_from_slice(&expiry_timestamp.to_be_bytes());

    let sig = key.sign(claims, Some(Noise::generate()));

    let encoded_claims = Base64UrlSafeNoPadding::encode_to_string(claims).unwrap();
    let encoded_sig = Base64UrlSafeNoPadding::encode_to_string(sig).unwrap();

    format!("dpdashboard.{encoded_claims}.{encoded_sig}")
}

#[must_use]
pub fn verify_token(token: &str) -> bool {
    let key_file = std::fs::read(&CONFIG.auth.pubkey_path).expect("failed to read public key file");
    let key = PublicKey::from_der(&key_file).expect("failed to parse public key");

    // Until key is verified, anything that can fail indicates a possible bad/malformed key
    let token_parts = token.split('.').collect::<Vec<_>>();
    let [ident, encoded_claims, encoded_sig] = *token_parts.as_slice() else {
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

    let Ok(sig) = Signature::from_slice(&sig) else {
        return false;
    };

    if key.verify(&claims, &sig).is_err() {
        return false;
    }

    // Unwrap is fine here because, assuming it exists, we know the slice is 8 bytes long
    let Some(claims_nbf) = claims
        .get(0..8)
        .map(|x| u64::from_be_bytes(x.try_into().unwrap()))
    else {
        return false;
    };

    let Some(claims_exp) = claims
        .get(8..16)
        .map(|x| u64::from_be_bytes(x.try_into().unwrap()))
    else {
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
