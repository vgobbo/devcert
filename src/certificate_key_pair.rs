use std::fs;
use rcgen::{Certificate, KeyPair};

pub struct CertificateKeyPair {
    certificate: Certificate,
    key_pair: KeyPair,
}

impl CertificateKeyPair {
    pub fn new(certificate: Certificate, key_pair: KeyPair) -> CertificateKeyPair {
        Self {
            certificate,
            key_pair,
        }
    }

    pub fn write(&self, prefix: &str) -> std::io::Result<()> {
        fs::write(format!("{prefix}.pem"), self.certificate.pem())?;
        fs::write(format!("{prefix}.cert"), self.key_pair.serialize_pem())?;
        Ok(())
    }
}