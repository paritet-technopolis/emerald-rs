//! # CLI wrapper for Ethereum Classic web3 like connector

#![cfg(feature = "cli")]

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate docopt;
extern crate env_logger;
extern crate emerald_core as emerald;
extern crate regex;

use docopt::Docopt;
use emerald::keystore::KdfDepthLevel;
use env_logger::LogBuilder;
use log::LogLevel;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::*;
use std::str::FromStr;

const USAGE: &'static str = include_str!("../usage.txt");

#[derive(Debug, Deserialize)]
struct Args {
    flag_version: bool,
    flag_verbose: usize,
    flag_quiet: bool,
    flag_host: String,
    flag_port: String,
    flag_base_path: String,
    flag_security_level: String,
    cmd_server: bool,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let verbosity: String = match args.flag_verbose {
        1 => "info".into(),
        2 => "warn".into(),
        3 => "error".into(),
        4 => "debug".into(),
        5 => "trace".into(),
        _ => "off".into(),
    };
    env::set_var("RUST_LOG", verbosity);
    let mut log_builder = LogBuilder::new();
    if env::var("RUST_LOG").is_ok() {
        log_builder.parse(&env::var("RUST_LOG").unwrap());
    }

    log_builder.init().expect("Expect to initialize logger");
    if args.flag_version {
        println!("v{}", emerald::version());
        exit(0);
    }

    if log_enabled!(LogLevel::Info) {
        info!("Starting Emerald Connector - v{}", emerald::version());
    }

    let sec_level: &str = &args.flag_security_level.parse::<String>().expect(
        "Expect to parse \
         security level",
    );
    let sec_level = match KdfDepthLevel::from_str(sec_level) {
        Ok(sec) => sec,
        Err(e) => {
            error!("{}", e.to_string());
            KdfDepthLevel::default()
        }
    };
    info!("security level set to '{}'", sec_level);

    if args.cmd_server {
        let addr = format!("{}:{}", args.flag_host, args.flag_port)
            .parse::<SocketAddr>()
            .expect("Expect to parse address");

        let base_path_str = args.flag_base_path.parse::<String>().expect(
            "Expect to parse base \
             path",
        );

        let base_path = if !base_path_str.is_empty() {
            Some(PathBuf::from(&base_path_str))
        } else {
            None
        };
        emerald::rpc::start(&addr, base_path, Some(sec_level));
    }

}