// rustfmt-edition: 2018
#![feature(uniform_paths)]
use ::log::{error, info, log};
use futures::prelude::*;
use http_03::cli::Cli;
use hyper::{Body, Response, Server, service::service_fn_ok};
use structopt::StructOpt;
