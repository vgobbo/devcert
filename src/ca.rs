use chrono::{Datelike, Duration, Utc};
use rcgen::{BasicConstraints, CertificateParams, DistinguishedName, DnType, IsCa, KeyPair, KeyUsagePurpose};
use rsa::pkcs8::EncodePrivateKey;
use rsa::RsaPrivateKey;
use crate::certificate_key_pair::CertificateKeyPair;

pub struct CaParameters {
    pub ttl: Duration,
    pub on: String,
    pub cn: String,
}

pub fn generate_ca<P>(params: P) -> Result<CertificateKeyPair, rcgen::Error> where P: Into<CaParameters> {
    let params = params.into();

    let mut cert_params = CertificateParams::default();

    cert_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);

    let mut distinguished_name = DistinguishedName::new();
    distinguished_name.push(DnType::OrganizationName, params.on.clone());
    distinguished_name.push(DnType::CommonName, params.cn.clone());
    cert_params.distinguished_name = distinguished_name;

    let not_before = Utc::now();
    let not_after = not_before + params.ttl;
    cert_params.not_before = rcgen::date_time_ymd(not_before.year(), not_before.month() as u8, not_before.day() as u8);
    cert_params.not_after = rcgen::date_time_ymd(not_after.year(), not_after.month() as u8, not_after.day() as u8);

    cert_params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];

    let ca_cert_key = generate_key_pair().unwrap();

    let ca_cert = cert_params.self_signed(&ca_cert_key)?;

    Ok(CertificateKeyPair::new(ca_cert, ca_cert_key))
}

fn generate_key_pair() -> rsa::Result<KeyPair> {
    let mut rng = rand::rngs::OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
    let private_key_der = private_key.to_pkcs8_der()?;
    Ok(KeyPair::try_from(private_key_der.as_bytes()).unwrap())
}
