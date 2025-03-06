use axum::{
    routing::get,
    Router,
};
use samael::metadata::EntityDescriptor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();

    let resp = reqwest::get("http://keycloak:8080/realms/master/protocol/saml/descriptor")
        .await?
        .text()
        .await?;
    let idp_metadata: EntityDescriptor = samael::metadata::de::from_str(&resp)?;

    let pub_key = openssl::x509::X509::from_pem(&std::fs::read("./publickey.cer")?)?;
    let private_key = openssl::rsa::Rsa::private_key_from_pem(&std::fs::read("./privatekey.pem")?)?;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}


