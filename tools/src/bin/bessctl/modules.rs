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

/// Run a command on an existing module
#[derive(Clap)]
pub struct RunModuleCommand {
    /// Name of the module to run the command on
    #[clap(name = "NAME")]
    pub name: String,

    /// Name of the command to run
    #[clap(name = "CMD")]
    pub cmd: String,

    /// JSON formated representation of the command's arguments
    #[clap(short = "j", long = "json", name = "ARGS")]
    pub json_args: Option<String>,

    /// Path to a file containing the JSON formated representation of the command's arguments
    #[clap(short = "f", long = "file", name = "ARGS_FILE")]
    pub args_file: Option<String>,
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

pub fn run_module_command(client: &BessClient, args: RunModuleCommand) {
    if args.json_args.is_some() && args.args_file.is_some() {
        println!("at most one of -j or -f can be provided");
        std::process::exit(1); // XXX: EINVAL
    }

    let mut jstr = "{}".to_string();
    if let Some(json_args) = args.json_args {
        jstr = json_args;
    }
    if let Some(file_args) = args.args_file {
        let mut f = fs::File::open(file_args).unwrap();
        f.read_to_string(&mut jstr).unwrap();
    }

    let mut mclass: Option<&str> = None;
    let mc_resp = client
        .grpc_handle
        .list_modules(grpc::RequestOptions::new(), bess_msg::EmptyRequest::new())
        .drop_metadata();
    let mc_resp = &executor::block_on(mc_resp).unwrap();
    for m in mc_resp.modules.iter() {
        if m.name == args.name {
            mclass = Some(&m.mclass);
            break;
        }
    }
    if mclass.is_none() {
        println!("module {} doesn't exist", args.name);
        std::process::exit(1); // XXX: ENOENT
    }
    let arg_desc = client
        .get_module_command_descriptor(mclass.unwrap(), &args.cmd)
        .unwrap();
    let conf = json_pb::from_str(&arg_desc, &client.descriptors, &jstr).unwrap();

    let mut req = bess_msg::CommandRequest::new();
    req.set_name(args.name);
    req.set_cmd(args.cmd);
    req.set_arg(pb::make_any(arg_desc.name(), "bess.pb", &conf).unwrap());
    let resp = client
        .grpc_handle
        .module_command(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();

    let (subdesc, inner_resp) = pb::unpack_any(resp.get_data(), &client.descriptors).unwrap();
    println!(
        "{}",
        json_pb::to_str(subdesc, &client.descriptors, &inner_resp).unwrap()
    );
}
