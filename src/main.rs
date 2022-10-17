use std::{path::{PathBuf, Path}, ffi::OsString};

use openssl::{pkey::PKey, nid::Nid};
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

    /// Directory for private key storage
    #[clap(long, env, value_parser, default_value="/data/")]
    output_path: PathBuf,


    #[clap(long, env, value_parser, default_value="admin@samply.beam.dkfz.de")]
    admin_email: String,

    #[clap(long, env, value_parser, default_value="false")]
    overwrite: bool,

    // Broker Domain
    //#[clap(long, env, value_parser)]
    //broker_url: String,
}

fn main() {
    let args = Args::parse();
    let id = beam_id::ProxyId::new(&args.proxy_id).unwrap();
    println!("Welcome to the Samply.Beam enrollment companion app.");
    println!("This application generates");
    println!("\ta) a secret key. This file is automatically saved and must not be shared," );
    println!("\tb) a certificate sign request. This is output sent to the administrator of the central broker via email to: {}.", args.admin_email);
    let (priv_key, csr) = generate_priv_key_and_csr(&id).unwrap();
    write_priv_key(priv_key, id, &args.output_path, args.overwrite).unwrap();
    println!("Please send the following text block to {}:", args.admin_email);
    println!("{}", String::from_utf8(csr).unwrap());

}

fn generate_priv_key_and_csr(proxy_id: &beam_id::ProxyId) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let rsa = PKey::from_rsa(openssl::rsa::Rsa::generate(2048)?)?;
    let private_key = &rsa.private_key_to_pem_pkcs8()?;
    let mut name_builder = openssl::x509::X509Name::builder()?;
    name_builder.append_entry_by_nid(Nid::COMMONNAME, proxy_id.value())?;
    name_builder.append_entry_by_nid(Nid::COUNTRYNAME, "DE")?;
    let mut csr_builder = openssl::x509::X509Req::builder()?;
    csr_builder.set_pubkey(rsa.as_ref())?;
    csr_builder.set_subject_name(&name_builder.build())?;
    csr_builder.sign(rsa.as_ref(), openssl::hash::MessageDigest::sha256())?;
    let csr = csr_builder.build().to_pem()?;

    Ok((private_key.clone(),csr))
}
 fn write_priv_key(priv_key: Vec<u8>, proxy_id: ProxyId, path: &Path, overwrite: bool) -> anyhow::Result<()>{
    let proxy = proxy_id.value().split('.').map(|v| String::from(v)).collect::<Vec<String>>();
    let filename = path.clone().with_file_name(&proxy[0]).with_extension("priv.pem");
    if filename.exists() && !overwrite{
        eprintln!("File {} already exists. For overwriting set --overwrite flag.", filename.into_os_string().to_string_lossy());
        std::process::exit(2);
    }
    std::fs::write(filename.into_os_string(), priv_key)?;
    Ok(())
 }
