use failure::format_err;
use failure::Error;
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
    pub fn new(addr: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = addr.split(":").collect();
        if parts.len() != 2 {
            return Err(format_err!("address must be <dns_name>:<port>"));
        }
        let grpc_handle =
            BESSControlClient::new_plain(parts[0], parts[1].parse()?, Default::default()).unwrap();
        let mut descriptors = Descriptors::new();

        let resp = executor::block_on(
            grpc_handle
                .dump_descriptors(
                    grpc::RequestOptions::new(),
                    libbess::bess_msg::EmptyRequest::new(),
                )
                .drop_metadata(),
        )?;

        for (_mclass, desc) in &resp.modules {
            descriptors.add_message_proto("", &desc);
        }
        for (_mclass, cmds) in &resp.module_commands {
            for (_cmd, desc) in &cmds.commands {
                descriptors.add_message_proto("", &desc);
            }
        }
        for (_driver, desc) in &resp.ports {
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
            Some(desc) => self
                .descriptors
                .message_by_name(&format!(".{}", desc.get_name())),
            None => None,
        }
    }

    pub fn get_module_command_descriptor(
        &self,
        mclass: &str,
        cmd: &str,
    ) -> Option<&MessageDescriptor> {
        match self.remote_descriptors.module_commands.get(mclass) {
            Some(cmds) => match cmds.commands.get(cmd) {
                Some(desc) => self
                    .descriptors
                    .message_by_name(&format!(".{}", desc.get_name())),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_port_init_descriptor(&self, driver: &str) -> Option<&MessageDescriptor> {
        match self.remote_descriptors.ports.get(driver) {
            Some(desc) => self
                .descriptors
                .message_by_name(&format!(".{}", desc.get_name())),
            None => None,
        }
    }
}
