use openssl::{pkey::PKey};
use clap::Parser;
mod beam_id;
mod errors;
use beam_id::*;
/// Settings for Samply.Beam (Shared)
#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// ProxyId of the option to enroll
    #[clap(long, env, value_parser)]
    proxy_id: String,

    // Broker Domain
    //#[clap(long, env, value_parser)]
    //broker_url: String,
}

#[allow(dead_code)]
pub(crate) struct Config {
    proxy_id: beam_id::ProxyId,
}
fn main() {
    let args = Args::parse();
    let id = beam_id::ProxyId::new("proxy.id").unwrap();
    generate_priv_key_and_csr(&id).unwrap();
}

fn generate_priv_key_and_csr(proxy_id: &beam_id::ProxyId) -> anyhow::Result<()> {
    println!("Generate new private key");
    let rsa = PKey::from_rsa(openssl::rsa::Rsa::generate(2048)?)?;
    let private_key = &rsa.private_key_to_pem_pkcs8()?;
    let public_key = &rsa.public_key_to_pem()?;
    println!("Build name");
    let mut name_builder = openssl::x509::X509Name::builder()?;
    name_builder.append_entry_by_text("CN", proxy_id.value())?;
    name_builder.append_entry_by_text("C", "DE")?;
    println!("Build CSR");
    let mut csr_builder = openssl::x509::X509Req::builder()?;
    csr_builder.set_pubkey(rsa.as_ref())?;
    csr_builder.set_subject_name(&name_builder.build())?;
    let csr = csr_builder.build().to_pem()?;

    println!("CSR: {}", String::from_utf8(csr).unwrap());
    println!("Private: {}", String::from_utf8_lossy(private_key));
    println!("Public: {}", String::from_utf8_lossy(public_key));

    println!("Please execute  openssl req -nodes -new -newkey rsa:2048 -sha256 -keyout secret_key.pem -out <ProxyId>_csr.pem -subj '/CN=<ProxyId>/C=DE'");
    //Err(SamplyBeamError::SignEncryptError(String::from("Cannot generate CSR and private Key")))
    Ok(())
}