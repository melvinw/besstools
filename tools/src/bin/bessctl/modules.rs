use std::fs;
use std::io::prelude::*;

use futures::executor;

use besstools::bess_client::BessClient;
use besstools::{json_pb, pb};
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

    /// JSON formated representation of the module's configuration
    #[clap(short = "j", long = "json", name = "CONFGI")]
    pub json_args: Option<String>,

    /// Path to a file containing the JSON formated representation of the module's configuration
    #[clap(short = "f", long = "file", name = "CONFIG_FILE")]
    pub args_file: Option<String>,
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

/// Connect two modules
#[derive(Clap)]
pub struct ConnectModules {
    /// Name of the source module
    #[clap(name = "SRC")]
    pub src: String,

    /// Name of the destination module
    #[clap(name = "DSST")]
    pub dest: String,

    /// The output gate to be used on the source module
    #[clap(long = "ogate", default_value = "0")]
    pub ogate: u16,

    /// The input gate to be used on the destination module
    #[clap(long = "igate", default_value = "0")]
    pub igate: u16,
}

/// Disconnect two modules
#[derive(Clap)]
pub struct DisconnectModules {
    /// Name of the source module
    #[clap(name = "SRC")]
    pub src: String,

    /// The output gate to be used on the source module
    #[clap(name = "OGATE")]
    pub ogate: u16,
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
    if args.json_args.is_some() && args.args_file.is_some() {
        println!("at most one of -j or -f can be provided");
        std::process::exit(1);
    }

    let mut jstr = "{}".to_string();
    if let Some(json_args) = args.json_args {
        jstr = json_args;
    }
    if let Some(file_args) = args.args_file {
        let mut f = fs::File::open(file_args).unwrap();
        f.read_to_string(&mut jstr).unwrap();
    }
    let arg_desc = client.get_module_init_descriptor(&args.mclass).unwrap();
    let conf = json_pb::from_str(&arg_desc, &client.descriptors, &jstr).unwrap();

    let mut req = bess_msg::CreateModuleRequest::new();
    req.set_mclass(args.mclass);
    req.set_name(args.name);
    req.set_arg(pb::make_any(arg_desc.name(), &conf).unwrap());
    let resp = client
        .grpc_handle
        .create_module(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();

    if resp.has_error() {
        let ec = resp.get_error().get_code();
        if ec != 0 {
            println!("{}", resp.get_error().get_errmsg());
            std::process::exit(ec);
        }
    }
}

pub fn destroy_module(client: &BessClient, args: DestroyModule) {
    let mut req = bess_msg::DestroyModuleRequest::new();
    req.set_name(args.name);
    let resp = client
        .grpc_handle
        .destroy_module(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();

    if resp.has_error() {
        let ec = resp.get_error().get_code();
        if ec != 0 {
            println!("{}", resp.get_error().get_errmsg());
            std::process::exit(ec);
        }
    }
}

pub fn run_module_command(client: &BessClient, args: RunModuleCommand) {
    if args.json_args.is_some() && args.args_file.is_some() {
        println!("at most one of -j or -f can be provided");
        std::process::exit(1);
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
        std::process::exit(1);
    }
    let arg_desc = client
        .get_module_command_descriptor(mclass.unwrap(), &args.cmd)
        .unwrap();
    let conf = json_pb::from_str(&arg_desc, &client.descriptors, &jstr).unwrap();

    let mut req = bess_msg::CommandRequest::new();
    req.set_name(args.name);
    req.set_cmd(args.cmd);
    req.set_arg(pb::make_any(arg_desc.name(), &conf).unwrap());
    let resp = client
        .grpc_handle
        .module_command(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();

    if resp.has_error() {
        let ec = resp.get_error().get_code();
        if ec != 0 {
            println!("{}", resp.get_error().get_errmsg());
            std::process::exit(ec);
        }
    }

    if resp.has_data() {
        let (subdesc, inner_resp) = pb::unpack_any(resp.get_data(), &client.descriptors).unwrap();
        println!(
            "{}",
            json_pb::to_str(subdesc, &client.descriptors, &inner_resp).unwrap()
        );
    }
}

pub fn connect_modules(client: &BessClient, args: ConnectModules) {
    let mut req = bess_msg::ConnectModulesRequest::new();
    req.set_m1(args.src);
    req.set_m2(args.dest);
    req.set_ogate(args.ogate.into());
    req.set_igate(args.igate.into());
    let resp = client
        .grpc_handle
        .connect_modules(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();

    if resp.has_error() {
        let ec = resp.get_error().get_code();
        if ec != 0 {
            println!("{}", resp.get_error().get_errmsg());
            std::process::exit(ec);
        }
    }
}

pub fn disconnect_modules(client: &BessClient, args: DisconnectModules) {
    let mut req = bess_msg::DisconnectModulesRequest::new();
    req.set_name(args.src);
    req.set_ogate(args.ogate.into());
    let resp = client
        .grpc_handle
        .disconnect_modules(grpc::RequestOptions::new(), req)
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();

    if resp.has_error() {
        let ec = resp.get_error().get_code();
        if ec != 0 {
            println!("{}", resp.get_error().get_errmsg());
            std::process::exit(ec);
        }
    }
}
