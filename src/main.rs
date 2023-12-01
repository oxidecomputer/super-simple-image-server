// Copyright 2023 Oxide Computer Company

//! HTTP server to serve an "image" to an Oxide system

use clap::Parser;
use dropshot::endpoint;
use dropshot::ApiDescription;
use dropshot::ConfigDropshot;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::HttpError;
use dropshot::HttpServerStarter;
use dropshot::RequestContext;
use hyper::body::Body;
use hyper::Response;
use hyper::StatusCode;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    listen_addr: std::net::SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    let config_dropshot =
        ConfigDropshot { bind_address: args.listen_addr, ..Default::default() };
    let config_logging =
        ConfigLogging::StderrTerminal { level: ConfigLoggingLevel::Info };
    let log = config_logging
        .to_logger("simple-server")
        .map_err(|error| format!("failed to create logger: {}", error))?;

    let mut api = ApiDescription::new();
    api.register(do_head).unwrap();
    api.register(do_get).unwrap();

    let api_context = ();

    let server =
        HttpServerStarter::new(&config_dropshot, api, api_context, &log)
            .map_err(|error| format!("failed to create server: {}", error))?
            .start();
    server.await
}

const IMAGE_SIZE: usize = 512;

/// Fetch the contents of a made-up image
#[endpoint {
    method = GET,
    path = "/image",
}]
async fn do_get(_: RequestContext<()>) -> Result<Response<Body>, HttpError> {
    let image_content = b"secret data\n";
    let mut body_bytes =
        image_content.repeat(1 + (IMAGE_SIZE / image_content.len()));
    assert!(body_bytes.len() >= IMAGE_SIZE);
    body_bytes.truncate(IMAGE_SIZE);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body_bytes))?)
}

#[endpoint {
    method = HEAD,
    path = "/image",
}]
async fn do_head(_: RequestContext<()>) -> Result<Response<Body>, HttpError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_LENGTH, IMAGE_SIZE.to_string())
        .body(Body::empty())?)
}
