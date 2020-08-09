// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// server interface

pub trait BESSControl {
    fn get_version(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::VersionResponse>,
    ) -> ::grpc::Result<()>;

    fn reset_all(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn kill_bess(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn import_plugin(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::ImportPluginRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn unload_plugin(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::UnloadPluginRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn list_plugins(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ListPluginsResponse>,
    ) -> ::grpc::Result<()>;

    fn pause_all(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn pause_worker(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::PauseWorkerRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn resume_worker(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::ResumeWorkerRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn resume_all(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn reset_workers(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn list_workers(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ListWorkersResponse>,
    ) -> ::grpc::Result<()>;

    fn add_worker(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::AddWorkerRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn destroy_worker(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::DestroyWorkerRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn reset_tcs(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn list_tcs(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::ListTcsRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ListTcsResponse>,
    ) -> ::grpc::Result<()>;

    fn check_scheduling_constraints(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::CheckSchedulingConstraintsResponse>,
    ) -> ::grpc::Result<()>;

    fn add_tc(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::AddTcRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn update_tc_params(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::UpdateTcParamsRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn update_tc_parent(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::UpdateTcParentRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn get_tc_stats(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::GetTcStatsRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::GetTcStatsResponse>,
    ) -> ::grpc::Result<()>;

    fn list_drivers(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ListDriversResponse>,
    ) -> ::grpc::Result<()>;

    fn get_driver_info(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::GetDriverInfoRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::GetDriverInfoResponse>,
    ) -> ::grpc::Result<()>;

    fn reset_ports(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn list_ports(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ListPortsResponse>,
    ) -> ::grpc::Result<()>;

    fn create_port(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::CreatePortRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::CreatePortResponse>,
    ) -> ::grpc::Result<()>;

    fn destroy_port(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::DestroyPortRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn set_port_conf(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::SetPortConfRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::CommandResponse>,
    ) -> ::grpc::Result<()>;

    fn get_port_conf(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::GetPortConfRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::GetPortConfResponse>,
    ) -> ::grpc::Result<()>;

    fn get_port_stats(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::GetPortStatsRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::GetPortStatsResponse>,
    ) -> ::grpc::Result<()>;

    fn get_link_status(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::GetLinkStatusRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::GetLinkStatusResponse>,
    ) -> ::grpc::Result<()>;

    fn list_mclass(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ListMclassResponse>,
    ) -> ::grpc::Result<()>;

    fn get_mclass_info(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::GetMclassInfoRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::GetMclassInfoResponse>,
    ) -> ::grpc::Result<()>;

    fn reset_modules(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn list_modules(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ListModulesResponse>,
    ) -> ::grpc::Result<()>;

    fn create_module(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::CreateModuleRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::CreateModuleResponse>,
    ) -> ::grpc::Result<()>;

    fn destroy_module(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::DestroyModuleRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn get_module_info(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::GetModuleInfoRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::GetModuleInfoResponse>,
    ) -> ::grpc::Result<()>;

    fn connect_modules(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::ConnectModulesRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn disconnect_modules(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::DisconnectModulesRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::EmptyResponse>,
    ) -> ::grpc::Result<()>;

    fn dump_mempool(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::DumpMempoolRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::DumpMempoolResponse>,
    ) -> ::grpc::Result<()>;

    fn module_command(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::CommandRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::CommandResponse>,
    ) -> ::grpc::Result<()>;

    fn list_gate_hook_class(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ListGateHookClassResponse>,
    ) -> ::grpc::Result<()>;

    fn get_gate_hook_class_info(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::GetGateHookClassInfoRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::GetGateHookClassInfoResponse>,
    ) -> ::grpc::Result<()>;

    fn configure_gate_hook(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::ConfigureGateHookRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ConfigureGateHookResponse>,
    ) -> ::grpc::Result<()>;

    fn list_gate_hooks(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::ListGateHooksResponse>,
    ) -> ::grpc::Result<()>;

    fn gate_hook_command(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::GateHookCommandRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::CommandResponse>,
    ) -> ::grpc::Result<()>;

    fn configure_resume_hook(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::ConfigureResumeHookRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::CommandResponse>,
    ) -> ::grpc::Result<()>;

    fn dump_descriptors(
        &self,
        o: ::grpc::ServerHandlerContext,
        req: ::grpc::ServerRequestSingle<super::bess_msg::EmptyRequest>,
        resp: ::grpc::ServerResponseUnarySink<super::bess_msg::DumpDescriptorsResponse>,
    ) -> ::grpc::Result<()>;
}

// client

pub struct BESSControlClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for BESSControlClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        BESSControlClient {
            grpc_client: grpc_client,
        }
    }
}

impl BESSControlClient {
    pub fn get_version(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::VersionResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetVersion"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn reset_all(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResetAll"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn kill_bess(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/KillBess"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn import_plugin(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::ImportPluginRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ImportPlugin"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn unload_plugin(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::UnloadPluginRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/UnloadPlugin"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn list_plugins(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ListPluginsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListPlugins"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn pause_all(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/PauseAll"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn pause_worker(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::PauseWorkerRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/PauseWorker"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn resume_worker(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::ResumeWorkerRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResumeWorker"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn resume_all(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResumeAll"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn reset_workers(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResetWorkers"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn list_workers(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ListWorkersResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListWorkers"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn add_worker(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::AddWorkerRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/AddWorker"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn destroy_worker(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::DestroyWorkerRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/DestroyWorker"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn reset_tcs(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResetTcs"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn list_tcs(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::ListTcsRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ListTcsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListTcs"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn check_scheduling_constraints(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::CheckSchedulingConstraintsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static(
                "/bess.pb.BESSControl/CheckSchedulingConstraints",
            ),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn add_tc(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::AddTcRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/AddTc"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn update_tc_params(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::UpdateTcParamsRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/UpdateTcParams"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn update_tc_parent(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::UpdateTcParentRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/UpdateTcParent"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_tc_stats(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::GetTcStatsRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::GetTcStatsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetTcStats"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn list_drivers(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ListDriversResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListDrivers"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_driver_info(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::GetDriverInfoRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::GetDriverInfoResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetDriverInfo"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn reset_ports(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResetPorts"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn list_ports(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ListPortsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListPorts"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn create_port(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::CreatePortRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::CreatePortResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/CreatePort"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn destroy_port(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::DestroyPortRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/DestroyPort"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn set_port_conf(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::SetPortConfRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::CommandResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/SetPortConf"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_port_conf(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::GetPortConfRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::GetPortConfResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetPortConf"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_port_stats(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::GetPortStatsRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::GetPortStatsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetPortStats"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_link_status(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::GetLinkStatusRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::GetLinkStatusResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetLinkStatus"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn list_mclass(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ListMclassResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListMclass"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_mclass_info(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::GetMclassInfoRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::GetMclassInfoResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetMclassInfo"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn reset_modules(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResetModules"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn list_modules(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ListModulesResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListModules"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn create_module(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::CreateModuleRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::CreateModuleResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/CreateModule"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn destroy_module(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::DestroyModuleRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/DestroyModule"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_module_info(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::GetModuleInfoRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::GetModuleInfoResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetModuleInfo"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn connect_modules(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::ConnectModulesRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ConnectModules"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn disconnect_modules(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::DisconnectModulesRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::EmptyResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/DisconnectModules"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn dump_mempool(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::DumpMempoolRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::DumpMempoolResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/DumpMempool"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn module_command(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::CommandRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::CommandResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ModuleCommand"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn list_gate_hook_class(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ListGateHookClassResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListGateHookClass"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_gate_hook_class_info(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::GetGateHookClassInfoRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::GetGateHookClassInfoResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetGateHookClassInfo"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn configure_gate_hook(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::ConfigureGateHookRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ConfigureGateHookResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ConfigureGateHook"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn list_gate_hooks(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::ListGateHooksResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListGateHooks"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn gate_hook_command(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::GateHookCommandRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::CommandResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GateHookCommand"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn configure_resume_hook(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::ConfigureResumeHookRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::CommandResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ConfigureResumeHook"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn dump_descriptors(
        &self,
        o: ::grpc::RequestOptions,
        req: super::bess_msg::EmptyRequest,
    ) -> ::grpc::SingleResponse<super::bess_msg::DumpDescriptorsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/DumpDescriptors"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct BESSControlServer;

impl BESSControlServer {
    pub fn new_service_def<H: BESSControl + 'static + Sync + Send + 'static>(
        handler: H,
    ) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new(
            "/bess.pb.BESSControl",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetVersion"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).get_version(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResetAll"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).reset_all(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/KillBess"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).kill_bess(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ImportPlugin",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).import_plugin(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/UnloadPlugin",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).unload_plugin(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ListPlugins",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).list_plugins(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/PauseAll"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).pause_all(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/PauseWorker",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).pause_worker(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ResumeWorker",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).resume_worker(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResumeAll"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).resume_all(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ResetWorkers",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).reset_workers(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ListWorkers",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).list_workers(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/AddWorker"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).add_worker(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/DestroyWorker",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).destroy_worker(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResetTcs"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).reset_tcs(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListTcs"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).list_tcs(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/CheckSchedulingConstraints",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).check_scheduling_constraints(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/AddTc"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).add_tc(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/UpdateTcParams",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).update_tc_params(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/UpdateTcParent",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).update_tc_parent(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/GetTcStats"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).get_tc_stats(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ListDrivers",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).list_drivers(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/GetDriverInfo",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).get_driver_info(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ResetPorts"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).reset_ports(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListPorts"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).list_ports(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/CreatePort"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).create_port(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/DestroyPort",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).destroy_port(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/SetPortConf",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).set_port_conf(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/GetPortConf",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).get_port_conf(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/GetPortStats",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).get_port_stats(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/GetLinkStatus",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).get_link_status(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/bess.pb.BESSControl/ListMclass"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).list_mclass(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/GetMclassInfo",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).get_mclass_info(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ResetModules",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).reset_modules(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ListModules",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).list_modules(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/CreateModule",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).create_module(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/DestroyModule",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).destroy_module(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/GetModuleInfo",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).get_module_info(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ConnectModules",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).connect_modules(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/DisconnectModules",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).disconnect_modules(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/DumpMempool",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).dump_mempool(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ModuleCommand",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).module_command(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ListGateHookClass",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).list_gate_hook_class(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/GetGateHookClassInfo",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).get_gate_hook_class_info(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ConfigureGateHook",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).configure_gate_hook(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ListGateHooks",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).list_gate_hooks(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/GateHookCommand",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).gate_hook_command(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/ConfigureResumeHook",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).configure_resume_hook(ctx, req, resp)
                        })
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static(
                            "/bess.pb.BESSControl/DumpDescriptors",
                        ),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(
                            &::grpc_protobuf::MarshallerProtobuf,
                        ),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| {
                            (*handler_copy).dump_descriptors(ctx, req, resp)
                        })
                    },
                ),
            ],
        )
    }
}
