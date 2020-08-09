use std::fs;
use std::io::prelude::*;

use futures::executor;
use serde_protobuf::descriptor::Descriptors;

use bessagent::{json_pb, pb};
use libbess::bess_msg;
use libbess::service_grpc::BESSControlClient;

use clap::Clap;

/// List the currently configured ports
#[derive(Clap)]
pub struct ListPorts {
    /// Glob filter to select ports with
    #[clap(name = "FILTER")]
    pub filter: Option<String>,

    /// Print ports as JSON objects
    #[clap(short = "j", long = "json")]
    pub json: bool,
}

/// Create a new port
#[derive(Clap)]
pub struct CreatePort {
    /// The driver the port should use
    #[clap(name = "DRIVER")]
    pub driver: String,

    /// Name to give the new port
    #[clap(name = "NAME")]
    pub name: String,

    /// Path to driver-specific configuration. Must be in canonical protobuf JSON format
    #[clap(name = "CONFIG", parse(from_os_str))]
    pub config: std::path::PathBuf,
}

/// Destroy an existing port
#[derive(Clap)]
pub struct DestroyPort {
    /// Name of the port to destroy
    #[clap(name = "NAME")]
    pub name: String,
}

pub fn list_ports(client: &BESSControlClient, args: ListPorts) {
    let resp = client
        .list_ports(grpc::RequestOptions::new(), bess_msg::EmptyRequest::new())
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();
    for port in resp.ports.iter() {
        if args.json {
            println!("{}", serde_json::to_string(port).unwrap());
        } else {
            println!("{} {} {}", port.name, port.driver, port.mac_addr);
        }
    }
}

pub fn create_port(client: &BESSControlClient, descriptors: &Descriptors, args: CreatePort) {
    let mut f = fs::File::open(args.config).unwrap();
    let mut jstr = String::new();
    f.read_to_string(&mut jstr).unwrap();
    let (arg_type, conf) = json_pb::from_str(descriptors, &jstr).unwrap();

    let mut req = bess_msg::CreatePortRequest::new();
    req.set_driver(args.driver);
    req.set_name(args.name);
    req.set_arg(pb::make_any(&arg_type, &conf).unwrap());
    let resp = client
        .create_port(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();
    println!("{}", serde_json::to_string(resp).unwrap());
}

pub fn destroy_port(client: &BESSControlClient, args: DestroyPort) {
    let mut req = bess_msg::DestroyPortRequest::new();
    req.set_name(args.name);
    let resp = client
        .destroy_port(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();
    println!("{}", serde_json::to_string(resp).unwrap());
}