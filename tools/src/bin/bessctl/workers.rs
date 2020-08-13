use futures::executor;

use besstools::bess_client::BessClient;
use libbess::bess_msg;

use clap::Clap;

/// List the currently configured workers
#[derive(Clap)]
pub struct ListWorkers {
    /// Print ports as JSON objects
    #[clap(short = "j", long = "json")]
    pub json: bool,
}

/// Create a new worker
#[derive(Clap)]
pub struct CreateWorker {
    /// The worker's ID
    #[clap(name = "WID")]
    pub wid: i64,

    /// Core to assign the worker to
    #[clap(name = "CORE")]
    pub core: i64,

    /// Optional scheduler to use on the worker
    #[clap(short = "s", long = "scheduler", name = "SCHEDULER")]
    pub scheduler: Option<String>,
}

/// Destroy an existing worker
#[derive(Clap)]
pub struct DestroyWorkers {
    /// ID of the worker(s) to destroy
    #[clap(name = "WID")]
    pub wids: Vec<i64>,
}

/// Pause workers
#[derive(Clap)]
pub struct PauseWorkers {
    /// ID of the worker(s) to pause
    #[clap(name = "WID")]
    pub wids: Vec<i64>,
}

/// Resume workers
#[derive(Clap)]
pub struct ResumeWorkers {
    /// ID of the worker(s) to resume
    #[clap(name = "WID")]
    pub wids: Vec<i64>,
}

pub fn list_workers(client: &BessClient, args: ListWorkers) {
    let resp = client
        .grpc_handle
        .list_workers(grpc::RequestOptions::new(), bess_msg::EmptyRequest::new())
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();

    if resp.has_error() {
        let ec = resp.get_error().get_code();
        if ec != 0 {
            println!("{}", resp.get_error().get_errmsg());
            std::process::exit(ec);
        }
    }

    for worker in resp.workers_status.iter() {
        if args.json {
            println!("{}", serde_json::to_string(worker).unwrap());
        } else {
            println!("{} {} {}", worker.wid, worker.core, worker.running);
        }
    }
}

pub fn create_worker(client: &BessClient, args: CreateWorker) {
    let mut req = bess_msg::AddWorkerRequest::new();
    req.set_wid(args.wid);
    req.set_core(args.core);
    req.set_scheduler(args.scheduler.unwrap_or("".to_string()));

    let resp = client
        .grpc_handle
        .add_worker(grpc::RequestOptions::new(), req)
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

pub fn destroy_workers(client: &BessClient, args: DestroyWorkers) {
    for wid in args.wids.iter() {
        let mut req = bess_msg::DestroyWorkerRequest::new();
        req.set_wid(*wid);
        let resp = client
            .grpc_handle
            .destroy_worker(grpc::RequestOptions::new(), req)
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
}

pub fn pause_workers(client: &BessClient, args: PauseWorkers) {
    for wid in args.wids.iter() {
        let mut req = bess_msg::PauseWorkerRequest::new();
        req.set_wid(*wid);
        let resp = client
            .grpc_handle
            .pause_worker(grpc::RequestOptions::new(), req)
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
}

pub fn resume_workers(client: &BessClient, args: ResumeWorkers) {
    for wid in args.wids.iter() {
        let mut req = bess_msg::ResumeWorkerRequest::new();
        req.set_wid(*wid);
        let resp = client
            .grpc_handle
            .resume_worker(grpc::RequestOptions::new(), req)
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
}
