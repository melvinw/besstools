extern crate futures;
extern crate libbess;
extern crate serde;
extern crate serde_protobuf;

use futures::executor;
use grpc::ClientStubExt;

use serde_protobuf::descriptor::{Descriptors, MessageDescriptor};

use libbess::service_grpc::BESSControlClient;

pub struct BessClient {
    pub grpc_handle: BESSControlClient,

    pub descriptors: Descriptors,
    remote_descriptors: libbess::bess_msg::DumpDescriptorsResponse,
}

impl BessClient {
    pub fn new(addr: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = addr.split(":").collect();
        assert!(parts.len() == 2);
        let grpc_handle =
            BESSControlClient::new_plain(parts[0], parts[1].parse().unwrap(), Default::default())
                .unwrap();
        let mut descriptors = Descriptors::new();

        let f_resp = grpc_handle
            .dump_descriptors(
                grpc::RequestOptions::new(),
                libbess::bess_msg::EmptyRequest::new(),
            )
            .drop_metadata();
        let resp = executor::block_on(f_resp).unwrap();
        for (_, desc) in &resp.modules {
            descriptors.add_message_proto("", &desc);
        }
        for (_, cmds) in &resp.module_commands {
            for (_, desc) in &cmds.commands {
                descriptors.add_message_proto("", &desc);
            }
        }
        for (_, desc) in &resp.ports {
            descriptors.add_message_proto("", &desc);
        }

        Ok(BessClient {
            grpc_handle: grpc_handle,
            descriptors: descriptors,
            remote_descriptors: resp,
        })
    }

    pub fn get_module_init_descriptor(&self, mclass: &str) -> Option<&MessageDescriptor> {
        match self.remote_descriptors.modules.get(mclass) {
            Some(desc) => self.descriptors.message_by_name(desc.get_name()),
            None => None,
        }
    }

    pub fn get_port_init_descriptor(&self, driver: &str) -> Option<&MessageDescriptor> {
        match self.remote_descriptors.ports.get(driver) {
            Some(desc) => self.descriptors.message_by_name(desc.get_name()),
            None => None,
        }
    }
}
