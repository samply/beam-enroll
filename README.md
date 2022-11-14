![Logo](./doc/Logo.svg) <!-- TODO: New Logo -->

The tool in this repository is a helper tool for enrolling new endpoints in a Samply.Beam network.

Samply.Beam is a distributed task broker designed for efficient communication across strict network environments. It provides most commonly used communication patterns across strict network boundaries, end-to-end encryption and signatures, as well as certificate management and validation on top of an easy to use REST API.
To provide authentication, end-to-end encryption, and ensure data integrity, Samply.Beam requires so called "public key cryptography". As it is implemented in Samply.Beam, this requires each site to have a secret "private key" and a publicly known "public certificate". This companion tool generates the private key and generates a "Certificate Sign Request" (CSR), a predecessor to the public certificate. This CSR can be used by the central CA operator to generate and sign the certificate.

## Usage

There are two ways to run the companion tool: First using Docker or Second compiling and running it locally. In both cases you need to provide the intended ProxyID (see [Beam Documentation]() for details). Therefore, the following commands expect the ProxyId in the environment varable `$PROXY_ID` and an output keyfile name in `$PRIVATEKEXFILENAME`. Per default, the beam proxy expects `$PROXY_ID_SHOR.priv.pem`.

### Docker

You can use the pre-compiled docker images in the [Docker registry](https://hub.docker.com/r/samply/beam-enroll):

```bash
docker run --rm -ti -v <path to beam proxy>/pki:/etc/bridgehead/pki samply/beam-enroll:latest --output-file $PRIVATEKEYFILENAME --proxy-id $PROXY_ID
chmod 600 $PRIVATEKEYFILENAME
```

### Manual

With an installed rust toolchain (at least Rust 1.65), first, clone the repository:

```bash
git clone https://github.com/samply/beam-enroll.git
```

Compile the application using cargo:

```bash
cd beam-enroll && cargo build --release
```

Last, execute the application:

```bash
./target/release/beam_enroll --proxy-id=$PROXY_ID
```

### Command Line Options

The Beam Certificate Enrollment Companion app supports the following command line parameters:

* `proxy-id`: The fully qualified proxy id, i.e. proxy1.broker.example.org
* `output-file`: Optional: Filename of the generated private key file. Defaults to `./pki/myprivatekey.pem`
* `admin-email`: Optional: Provide your central Beam admin's email address for better on screen directions
* `overwrite`: Optional: Allow the companion tool to overwrite an existing private key file. Defaults to `false`

## Cryptography Notice

This distribution includes cryptographic software. The country in which you currently reside may have restrictions on the import, possession, use, and/or re-export to another country, of encryption software. BEFORE using any encryption software, please check your country's laws, regulations and policies concerning the import, possession, or use, and re-export of encryption software, to see if this is permitted. See [http://www.wassenaar.org/](http://www.wassenaar.org) for more information.
