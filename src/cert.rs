use chrono::{Datelike, Duration, Utc};
use rcgen::{CertificateParams, DistinguishedName, DnType, IsCa, KeyUsagePurpose, SanType};

use crate::certificate_key_pair::CertificateKeyPair;
use crate::kp;

pub struct CertParameters {
	pub ttl: Duration,
	pub on: String,
	pub cn: String,
	pub ca: CertificateKeyPair,
	pub sans: Vec<String>,
}

pub fn generate(params: CertParameters) -> Result<CertificateKeyPair, rcgen::Error> {
	let mut cert_params = CertificateParams::default();
	cert_params.is_ca = IsCa::NoCa;

	let mut distinguished_name = DistinguishedName::new();
	distinguished_name.push(DnType::OrganizationName, params.on.clone());
	distinguished_name.push(DnType::CommonName, params.cn.clone());
	cert_params.distinguished_name = distinguished_name;

	let not_before = Utc::now();
	let not_after = not_before + params.ttl;
	cert_params.not_before = rcgen::date_time_ymd(not_before.year(), not_before.month() as u8, not_before.day() as u8);
	cert_params.not_after = rcgen::date_time_ymd(not_after.year(), not_after.month() as u8, not_after.day() as u8);

	cert_params.key_usages =
		vec![KeyUsagePurpose::KeyEncipherment, KeyUsagePurpose::DigitalSignature, KeyUsagePurpose::ContentCommitment];

	let sans = params.sans.into_iter().map(|san| SanType::DnsName(san.try_into().unwrap())).collect::<Vec<SanType>>();
	cert_params.subject_alt_names = sans;

	let cert_key = kp::generate().unwrap();

	let cert = cert_params.signed_by(&cert_key, params.ca.certificate(), params.ca.key_pair())?;

	Ok(CertificateKeyPair::new(cert, cert_key))
}
