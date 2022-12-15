use std::fs::File;
use std::io;
use std::io::BufReader;
use rustls::{Certificate, Error, PrivateKey, ServerConfig};
use rustls_pemfile as pem;

pub fn load_cert_chain(path: &str) -> io::Result<Vec<Certificate>> {
  let mut file = BufReader::new(File::open(path)?);
  let certs = pem::certs(&mut file)?.into_iter().map(Certificate).collect();

  Ok(certs)
}

pub fn load_private_key(path: &str) -> io::Result<PrivateKey> {
  let mut file = BufReader::new(File::open(path)?);
  let mut keys = pem::pkcs8_private_keys(&mut file)?;

  Ok(PrivateKey(keys.remove(0)))
}

pub fn build_config(cert_chain: Vec<Certificate>, private_key: PrivateKey) -> Result<ServerConfig, Error> {
  ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth()
    .with_single_cert(cert_chain, private_key)
}