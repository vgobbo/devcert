use rcgen::KeyPair;
use rsa::pkcs8::EncodePrivateKey;
use rsa::RsaPrivateKey;

pub fn generate() -> rsa::Result<KeyPair> {
    let mut rng = rand::rngs::OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
    let private_key_der = private_key.to_pkcs8_der()?;
    Ok(KeyPair::try_from(private_key_der.as_bytes()).unwrap())
}
