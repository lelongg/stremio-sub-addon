#![allow(dead_code)]

mod handlers;
mod manifest;

use handlers::build;
use manifest::get_manifest;
use now_lambda::{error::NowError, lambda, IntoResponse, Request};
use std::error::Error;
use stremio_addon_sdk::server::{serve_serverless, ServerOptions};

fn handler(req: Request) -> Result<impl IntoResponse, NowError> {
    let manifest = get_manifest();
    let build = build(manifest);
    let options = ServerOptions::default();
    serve_serverless(req, build, options)
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);
    Ok(())
}
