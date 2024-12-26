use std::time::Duration;
use chrono::{Datelike, Utc};
use clap::Args;
use duration_flex::DurationFlex;
use exitcode::ExitCode;
use gethostname::gethostname;
use rcgen::{
    BasicConstraints, Certificate, CertificateParams, DistinguishedName, DnType, IsCa, KeyPair, KeyUsagePurpose,
};
use rsa::pkcs8::EncodePrivateKey;
use rsa::RsaPrivateKey;

#[derive(Args)]
pub struct Arguments {
    /// How long in the future should the certificate expire.
    #[arg(long, default_value_t = Arguments::default().ttl)]
    pub ttl: DurationFlex,

    /// ON to be used on the CA.
    #[arg(long, default_value_t = Arguments::default().on)]
    pub on: String,

    /// CN to be used on the CA.
    #[arg(long, default_value_t = Arguments::default().cn)]
    pub cn: String,

    /// Output file name.
    #[arg(long, default_value_t = Arguments::default().name)]
    pub name: String,
}

pub fn main(args: Arguments) -> Result<(), ExitCode> {
    match generate_ca(args) {
        Ok(_) => Ok(()),
        Err(rcgen::Error::PemError(_)) => Err(exitcode::IOERR),
        Err(_) => Err(exitcode::SOFTWARE),
    }
}

fn generate_ca(args: Arguments) -> Result<(Certificate, KeyPair), rcgen::Error> {
    let mut cert_params = CertificateParams::default();

    cert_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);

    let mut distinguished_name = DistinguishedName::new();
    distinguished_name.push(DnType::OrganizationName, args.on.clone());
    distinguished_name.push(DnType::CommonName, args.cn.clone());
    cert_params.distinguished_name = distinguished_name;

    let not_before = Utc::now();
    let not_after = not_before + args.ttl;
    cert_params.not_before = rcgen::date_time_ymd(not_before.year(), not_before.month() as u8, not_before.day() as u8);
    cert_params.not_after = rcgen::date_time_ymd(not_after.year(), not_after.month() as u8, not_after.day() as u8);

    cert_params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign];

    let ca_cert_key = generate_key_pair().unwrap();

    let ca_cert = cert_params.self_signed(&ca_cert_key)?;

    Ok((ca_cert, ca_cert_key))
}

fn generate_key_pair() -> rsa::Result<KeyPair> {
    let mut rng = rand::rngs::OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
    let private_key_der = private_key.to_pkcs8_der()?;
    Ok(KeyPair::try_from(private_key_der.as_bytes()).unwrap())
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(365 * 24 * 60 * 60).into(),
            on: gethostname().to_string_lossy().into_owned(),
            cn: gethostname().to_string_lossy().into_owned(),
            name: "ca_cert".to_owned(),
        }
    }
}