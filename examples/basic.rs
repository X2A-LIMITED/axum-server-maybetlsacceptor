use anyhow::{Context as _, anyhow};
use axum::{Router, routing::get};
use axum_server::{
    accept::DefaultAcceptor,
    tls_rustls::{RustlsAcceptor, RustlsConfig},
};
use axum_server_maybetlsacceptor::MaybeTlsAcceptor;
use std::net::SocketAddr;

struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // 1. configure TLS cert/key file paths (through env or whatever)
    // let tls_config = Some(TlsConfig {
    //     cert_path: "cert.pem".to_owned(),
    //     key_path: "key.pem".to_owned(),
    // });
    let tls_config = None::<TlsConfig>;

    // 2. create acceptor
    let acceptor = if let Some(TlsConfig {
        ref cert_path,
        ref key_path,
    }) = tls_config
    {
        let () = rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .map_err(|_| anyhow!("could not register default crypto provider"))?;
        let tls_config = RustlsConfig::from_pem_file(cert_path, key_path)
            .await
            .with_context(|| "could not load TLS config")?;

        MaybeTlsAcceptor::Rustls(RustlsAcceptor::new(tls_config))
    } else {
        MaybeTlsAcceptor::Default(DefaultAcceptor)
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let router = Router::new().route("/", get(async || "Hello, World!"));

    axum_server::bind(addr)
        .acceptor(acceptor) // 3. set acceptor here
        .serve(router.into_make_service())
        .await
        .with_context(|| "could not start server")?;
    Ok(())
}
