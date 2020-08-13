extern crate besstools;
extern crate clap;
extern crate failure;
extern crate futures;
extern crate glob;
extern crate libbess;
extern crate serde;
extern crate serde_json;

mod modules;
mod ports;
mod workers;

use std::fs;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Melvin Walls <mwalls67@gmail.com>")]
struct Opts {
    /// The <address>:<port> of the BESS daemon to control.
    #[clap(short = "b", long = "bess", default_value = "127.0.0.1:10514")]
    bess_addr: String,

    /// Path to a directory containing extra compiled protobuf definitions (produced with `protoc -o foo.pb foo.proto`)
    #[clap(short = "p", long = "extra-protos")]
    extra_proto_dir: Option<std::path::PathBuf>,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    ListPorts(ports::ListPorts),
    CreatePort(ports::CreatePort),
    DestroyPort(ports::DestroyPort),
    ListModules(modules::ListModules),
    CreateModule(modules::CreateModule),
    DestroyModule(modules::DestroyModule),
    RunModuleCommand(modules::RunModuleCommand),
    ConnectModules(modules::ConnectModules),
    DisconnectModules(modules::DisconnectModules),
    ListWorkers(workers::ListWorkers),
    CreateWorker(workers::CreateWorker),
    DestroyWorker(workers::DestroyWorker),
    PauseWorkers(workers::PauseWorkers),
    ResumeWorkers(workers::ResumeWorkers),
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut client = besstools::bess_client::BessClient::new(&opts.bess_addr).unwrap();
    if let Some(extra_protos) = opts.extra_proto_dir {
        if let Ok(protos) = glob::glob(&format!("{}/*.pb", extra_protos.display())) {
            for pb in protos {
                if let Ok(path) = pb {
                    let mut f = fs::File::open(path).unwrap();
                    let proto = protobuf::parse_from_reader(&mut f).unwrap();
                    client.descriptors.add_file_set_proto(&proto);
                }
            }
        }
    }
    client.descriptors.resolve_refs();

    match opts.subcmd {
        SubCommand::ListPorts(args) => ports::list_ports(&client, args),
        SubCommand::CreatePort(args) => ports::create_port(&client, args),
        SubCommand::DestroyPort(args) => ports::destroy_port(&client, args),
        SubCommand::ListModules(args) => modules::list_modules(&client, args),
        SubCommand::CreateModule(args) => modules::create_module(&client, args),
        SubCommand::DestroyModule(args) => modules::destroy_module(&client, args),
        SubCommand::RunModuleCommand(args) => modules::run_module_command(&client, args),
        SubCommand::ConnectModules(args) => modules::connect_modules(&client, args),
        SubCommand::DisconnectModules(args) => modules::disconnect_modules(&client, args),
        SubCommand::ListWorkers(args) => workers::list_workers(&client, args),
        SubCommand::CreateWorker(args) => workers::create_worker(&client, args),
        SubCommand::DestroyWorker(args) => workers::destroy_worker(&client, args),
        SubCommand::PauseWorkers(args) => workers::pause_workers(&client, args),
        SubCommand::ResumeWorkers(args) => workers::resume_workers(&client, args),
    }
}
