use std::fs;
use std::io::prelude::*;

use futures::executor;

use bessagent::bess_client::BessClient;
use bessagent::{json_pb, pb};
use libbess::bess_msg;

use clap::Clap;

/// List the currently configured modules
#[derive(Clap)]
pub struct ListModules {
    /// Glob filter to select modules with
    #[clap(name = "FILTER")]
    pub filter: Option<String>,

    /// Print modules as JSON objects
    #[clap(short = "j", long = "json")]
    pub json: bool,
}

/// Create a new module
#[derive(Clap)]
pub struct CreateModule {
    /// The mclass of the new module
    #[clap(name = "MCLASS")]
    pub mclass: String,

    /// Name to give the new module
    #[clap(name = "NAME")]
    pub name: String,

    /// Path to driver-specific configuration. Must be in canonical protobuf JSON format
    #[clap(name = "CONFIG", parse(from_os_str))]
    pub config: std::path::PathBuf,
}

/// Destroy an existing module
#[derive(Clap)]
pub struct DestroyModule {
    /// Name of the module to destroy
    #[clap(name = "NAME")]
    pub name: String,
}

pub fn list_modules(client: &BessClient, args: ListModules) {
    let resp = client
        .grpc_handle
        .list_modules(grpc::RequestOptions::new(), bess_msg::EmptyRequest::new())
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();
    for m in resp.modules.iter() {
        if args.json {
            println!("{}", serde_json::to_string(m).unwrap());
        } else {
            println!("{} {} {}", m.name, m.mclass, m.desc);
        }
    }
}

pub fn create_module(client: &BessClient, args: CreateModule) {
    let mut f = fs::File::open(args.config).unwrap();
    let mut jstr = String::new();
    f.read_to_string(&mut jstr).unwrap();
    let arg_desc = client.get_module_init_descriptor(&args.mclass).unwrap();
    let conf = json_pb::from_str(&arg_desc, &client.descriptors, &jstr).unwrap();

    let mut req = bess_msg::CreateModuleRequest::new();
    req.set_mclass(args.mclass);
    req.set_name(args.name);
    req.set_arg(pb::make_any(arg_desc.name(), "bess.pb", &conf).unwrap());
    let resp = client
        .grpc_handle
        .create_module(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();
    println!("{}", serde_json::to_string(resp).unwrap());
}

pub fn destroy_module(client: &BessClient, args: DestroyModule) {
    let mut req = bess_msg::DestroyModuleRequest::new();
    req.set_name(args.name);
    let resp = client
        .grpc_handle
        .destroy_module(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();
    println!("{}", serde_json::to_string(resp).unwrap());
}
