use google_drive3::{DriveHub, hyper, hyper_rustls, oauth2};

pub async fn init_drive_client() -> DriveHub<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    let secret = oauth2::read_application_secret("credentials.json")
        .await
        .expect("Failed to read Google API credentials");

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .expect("Failed to build authenticator");

    // FIXED: unwrap the Result from `with_native_roots()`
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .expect("Failed to load native cert roots")
        .https_or_http()
        .enable_http1()
        .build();

    let client = hyper::Client::builder().build(https);

    DriveHub::new(client, auth)
}
