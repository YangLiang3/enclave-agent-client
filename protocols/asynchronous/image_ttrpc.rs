// This file is generated by ttrpc-compiler 0.4.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clipto_camel_casepy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Clone)]
pub struct ImageClient {
    client: ::ttrpc::r#async::Client,
}

impl ImageClient {
    pub fn new(client: ::ttrpc::r#async::Client) -> Self {
        ImageClient {
            client: client,
        }
    }

    pub async fn pull_image(&mut self, ctx: ttrpc::context::Context, req: &super::image::PullImageRequest) -> ::ttrpc::Result<super::image::PullImageResponse> {
        let mut cres = super::image::PullImageResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "grpc.Image", "PullImage", cres);
    }
}

struct PullImageMethod {
    service: Arc<std::boxed::Box<dyn Image + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for PullImageMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, image, PullImageRequest, pull_image);
    }
}

#[async_trait]
pub trait Image: Sync {
    async fn pull_image(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::image::PullImageRequest) -> ::ttrpc::Result<super::image::PullImageResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/grpc.Image/PullImage is not supported".to_string())))
    }
}

pub fn create_image(service: Arc<std::boxed::Box<dyn Image + Send + Sync>>) -> HashMap <String, Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>> {
    let mut methods = HashMap::new();

    methods.insert("/grpc.Image/PullImage".to_string(),
                    std::boxed::Box::new(PullImageMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods
}