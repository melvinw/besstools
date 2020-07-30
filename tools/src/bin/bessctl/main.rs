extern crate bessagent;
extern crate clap;
extern crate futures;
extern crate glob;
extern crate libbess;
extern crate serde;
extern crate serde_json;

use std::fs;
use std::io::prelude::*;

use clap::Clap;
use futures::executor;
use grpc::ClientStubExt;
use protobuf::Message;
use serde_protobuf::descriptor::Descriptors;

use bessagent::{json_pb, pb};
use libbess::service_grpc::BESSControlClient;
use libbess::{bess_msg, port_msg};

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Melvin Walls <mwalls67@gmail.com>")]
struct Opts {
    /// The <address>:<port> of the BESS daemon to control.
    #[clap(short = "b", long = "bess", default_value = "127.0.0.1:10514")]
    bess_addr: String,

    /// Path to a directory containing extra compiled protobuf definitions (produced with `protoc -o foo.pb foo.proto`)
    #[clap(short = "p", long = "extra-protos", default_value = "./extra_protos")]
    extra_proto_dir: std::path::PathBuf,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    ListPorts(ListPorts),
    CreatePort(CreatePort),
}

/// List the currently configured ports
#[derive(Clap)]
struct ListPorts {
    /// Glob filter to select ports with
    #[clap(name = "FILTER")]
    filter: Option<String>,
}

/// Create a new port
#[derive(Clap)]
struct CreatePort {
    /// The driver the port should use
    #[clap(name = "DRIVER")]
    driver: String,

    /// Name to give the new port
    #[clap(name = "NAME")]
    name: String,

    /// Path to driver-specific configuration. Must be in canonical protobuf JSON format
    #[clap(name = "CONFIG", parse(from_os_str))]
    config: std::path::PathBuf,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut descriptors = Descriptors::new();
    if let Ok(protos) = glob::glob(&format!("{}/*.pb", opts.extra_proto_dir.display())) {
        for pb in protos {
            if let Ok(path) = pb {
                let mut f = fs::File::open(path).unwrap();
                let proto = protobuf::parse_from_reader(&mut f).unwrap();
                descriptors.add_file_set_proto(&proto);
            }
        }
    }
    descriptors.resolve_refs();

    let parts: Vec<&str> = opts.bess_addr.split(":").collect();
    assert!(parts.len() == 2);
    let client =
        BESSControlClient::new_plain(parts[0], parts[1].parse().unwrap(), Default::default())
            .unwrap();

    match opts.subcmd {
        SubCommand::ListPorts(_) => {
            let resp = client
                .list_ports(grpc::RequestOptions::new(), bess_msg::EmptyRequest::new())
                .drop_metadata();
            let resp = &executor::block_on(resp).unwrap();
            println!("{}", serde_json::to_string(resp).unwrap());
        }
        SubCommand::CreatePort(args) => {
            let mut f = fs::File::open(args.config).unwrap();
            let mut jstr = String::new();
            f.read_to_string(&mut jstr).unwrap();
            let (arg_type, conf) = json_pb::from_str(&descriptors, &jstr).unwrap();

            let mut req = libbess::bess_msg::CreatePortRequest::new();
            req.set_driver(args.driver);
            req.set_name(args.name);
            req.set_arg(pb::make_any(&arg_type, &conf).unwrap());
            let resp = client
                .create_port(grpc::RequestOptions::new(), req)
                .drop_metadata();
            let resp = &executor::block_on(resp).unwrap();
            println!("{}", serde_json::to_string(resp).unwrap());
        }
    }
}
