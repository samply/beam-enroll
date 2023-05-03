use std::path::{Path, PathBuf};

use clap::Parser;
use openssl::{
    nid::Nid,
    pkey::{PKey, Private},
};
mod beam_id;
mod errors;
use beam_id::*;
/// Settings for Samply.Beam (Shared)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// ProxyId of the option to enroll
    #[clap(long, env, value_parser)]
    proxy_id: String,

    /// File to store private key (.pem format)
    #[clap(long, env, value_parser, default_value = "./pki/myprivatekey.pem")]
    output_file: PathBuf,

    #[clap(long, env, value_parser)]
    admin_email: Option<String>,

    #[clap(long, env, value_parser, default_value = "false")]
    overwrite: bool,
    // Broker Domain
    //#[clap(long, env, value_parser)]
    //broker_url: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let id = beam_id::ProxyId::new(&args.proxy_id).unwrap();
    println!("Welcome to the Samply.Beam enrollment companion app.");
    println!("This application generates");
    println!("\ta) a secret key. This file is automatically saved and must not be shared,");
    println!("\tb) a certificate sign request. This output is sent to the administrator of the central broker via email{}.", match args.admin_email{Some(ref addr)=> " to: ".to_owned() + addr, None => "".to_string()});

    let (priv_key, mut csr) = generate_priv_key_and_csr(&id)?;

    if args.output_file.exists() && !args.overwrite {
        eprintln!(
            "File {} already exists. For overwriting set --overwrite flag. Displaying the CSR of the existing private key instead.",
            args.output_file.to_string_lossy()
        );
        let privkey = std::fs::read_to_string(args.output_file)?;
        let privkey = PKey::from_rsa(openssl::rsa::Rsa::private_key_from_pem(privkey.as_bytes())?)?;
        csr = generate_csr(&privkey, &id)?;
    } else {
        write_priv_key(priv_key, &args.output_file)?;
    }
    println!(
        "Please send the following text block to {}:",
        match args.admin_email {
            Some(ref addr) => addr,
            None => "the central administrator",
        }
    );
    println!("{}", String::from_utf8(csr).unwrap());
    Ok(())
}

fn generate_priv_key_and_csr(proxy_id: &ProxyId) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let rsa = PKey::from_rsa(openssl::rsa::Rsa::generate(2048)?)?;
    let private_key = rsa.private_key_to_pem_pkcs8()?;
    let csr = generate_csr(&rsa, proxy_id)?;
    Ok((private_key, csr))
}

fn generate_csr(priv_key: &PKey<Private>, proxy_id: &ProxyId) -> anyhow::Result<Vec<u8>> {
    let mut name_builder = openssl::x509::X509Name::builder()?;
    name_builder.append_entry_by_nid(Nid::COMMONNAME, proxy_id.value())?;
    name_builder.append_entry_by_nid(Nid::COUNTRYNAME, "DE")?;
    let mut csr_builder = openssl::x509::X509Req::builder()?;
    csr_builder.set_pubkey(priv_key.as_ref())?;
    csr_builder.set_subject_name(&name_builder.build())?;
    csr_builder.sign(priv_key.as_ref(), openssl::hash::MessageDigest::sha256())?;
    let csr = csr_builder.build().to_pem()?;
    Ok(csr)
}

fn write_priv_key(priv_key: Vec<u8>, filename: &Path) -> anyhow::Result<()> {
    std::fs::write(filename, priv_key)?;
    Ok(())
}
