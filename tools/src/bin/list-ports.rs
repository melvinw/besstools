extern crate futures;
extern crate libbess;
extern crate serde;
extern crate serde_json;

use futures::executor;
use grpc::ClientStubExt;
use protobuf::Message;

use libbess::service_grpc::BESSControlClient;
use libbess::{bess_msg, port_msg};

fn main() {
    let bessctl = BESSControlClient::new_plain("127.0.0.1", 10514, Default::default()).unwrap();
    let resp = bessctl
        .list_ports(grpc::RequestOptions::new(), bess_msg::EmptyRequest::new())
        .drop_metadata();
    let resp = &executor::block_on(resp).unwrap();
    println!("{}", serde_json::to_string(resp).unwrap());
    for p in resp.get_ports() {
        let arg: port_msg::VPortArg = p.get_driver_arg().unpack().unwrap().unwrap();
        println!("{}", serde_json::to_string(&arg).unwrap());
    }
}
