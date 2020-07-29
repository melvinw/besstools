extern crate bessagent;
extern crate serde_json;

use std::fs;
use std::io::prelude::*;

use bessagent::json_pb;
use protobuf::Message;
use serde_protobuf::descriptor::Descriptors;

fn main() {
    // Read Descriptors
    let mut descriptors = Descriptors::new();
    let mut f = fs::File::open("tools/port_msg.pb").unwrap();
    let proto = protobuf::parse_from_reader(&mut f).unwrap();
    descriptors.add_file_set_proto(&proto);
    let mut f = fs::File::open("tools/bess_msg.pb").unwrap();
    let proto = protobuf::parse_from_reader(&mut f).unwrap();
    descriptors.add_file_set_proto(&proto);
    descriptors.resolve_refs();

    let mut f = fs::File::open("tools/blah2.json").unwrap();
    let mut jstr = String::new();
    f.read_to_string(&mut jstr).unwrap();
    let msg = json_pb::from_str(&descriptors, &jstr).unwrap();

    let mut p = libbess::port_msg::VPortArg::new();
    p.set_ifname("eth_bob".to_string());
    p.set_rxq_cpus(vec![0, 1]);
    p.set_ip_addrs(protobuf::RepeatedField::from_vec(vec![
        "10.255.99.1/24".to_string(),
        "fdd4:955b:82c1:0cb7::2/64".to_string(),
    ]));
    p.set_tx_tci(0);
    p.set_tx_outer_tci(0);
    p.set_loopback(false);

    let v = msg.write_to_bytes().unwrap();
    let pv = p.write_to_bytes().unwrap();
    println!("{:?}", v);
    println!("{:?}", pv);
    assert!(pv == v);

    println!("---");
    let mut f2 = fs::File::open("tools/blah3.json").unwrap();
    let mut jstr2 = String::new();
    f2.read_to_string(&mut jstr2).unwrap();
    let req = json_pb::from_str(&descriptors, &jstr2).unwrap();
    let rv = req.write_to_bytes().unwrap();

    let mut cargs = libbess::bess_msg::CreatePortRequest::new();
    cargs.set_arg(protobuf::well_known_types::Any::pack(&p).unwrap());
    let cv = cargs.write_to_bytes().unwrap();
    println!("{:?}", rv);
    println!("{:?}", cv);
    assert!(rv == cv);
}
