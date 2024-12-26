use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use exitcode::ExitCode;
use rcgen::{Certificate, CertificateParams, KeyPair};

pub struct Error {
    pub file: PathBuf,
    pub kind: ErrorKind,
}

pub enum ErrorKind {
    IO(std::io::Error),
    Certificate(rcgen::Error),
}

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
        fs::write(format!("{prefix}.key"), self.key_pair.serialize_pem())?;
        Ok(())
    }

    pub fn certificate(&self) -> &Certificate {
        &self.certificate
    }

    pub fn key_pair(&self) -> &KeyPair {
        &self.key_pair
    }
}

impl TryFrom<PathBuf> for CertificateKeyPair {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let ca_key_path = path.with_extension("key");
        let ca_key_pem = fs::read_to_string(&ca_key_path)
            .map_err(|e| Error::new(ca_key_path.clone(), ErrorKind::IO(e)))?;
        let ca_key = KeyPair::from_pem(ca_key_pem.as_str())
            .map_err(|e| Error::new(ca_key_path, ErrorKind::Certificate(e)))?;

        let ca_cert_path = path.with_extension("pem");
        let ca_cert_pem = fs::read_to_string(&ca_cert_path)
            .map_err(|e| Error::new(ca_cert_path.clone(), ErrorKind::IO(e)))?;
        let ca_cert_params = CertificateParams::from_ca_cert_pem(ca_cert_pem.as_str())
            .map_err(|e| Error::new(ca_cert_path.clone(), ErrorKind::Certificate(e)))?;

        // rcgen doesn't offer a way of loading the CA, so we create a fake temporary certificate.
        let ca_cert = ca_cert_params
            .self_signed(&ca_key)
            .map_err(|e| Error::new(ca_cert_path, ErrorKind::Certificate(e)))?;

        Ok(CertificateKeyPair::new(ca_cert, ca_key))
    }
}

impl Error {
    pub fn new(file: PathBuf, kind: ErrorKind) -> Self {
        Self { file, kind }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing file {}: ", self.file.as_os_str().to_string_lossy())?;
        match &self.kind {
            ErrorKind::IO(e) => write!(f, "I/O error: {}", e),
            ErrorKind::Certificate(e) => write!(f, "Certificate error: {}", e),
        }
    }
}

impl Into<ExitCode> for Error {
    fn into(self) -> ExitCode {
        match self.kind {
            ErrorKind::IO(_) => exitcode::IOERR,
            ErrorKind::Certificate(_) => exitcode::DATAERR,
        }
    }
}