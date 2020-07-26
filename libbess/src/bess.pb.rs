#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyResponse {
    //// Contains a non-zero error code and a non-empty message if and only if
    //// there has been an error
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// Version of bessd
    #[prost(string, tag = "2")]
    pub version: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportPluginRequest {
    //// Path to the module library (*.so file)
    #[prost(string, tag = "1")]
    pub path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnloadPluginRequest {
    //// Path to the module library (*.so file)
    #[prost(string, tag = "1")]
    pub path: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPluginsResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// Paths to the module library (*.so file)
    #[prost(string, repeated, tag = "2")]
    pub paths: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkersResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// List of all existing workers
    #[prost(message, repeated, tag = "2")]
    pub workers_status: ::std::vec::Vec<list_workers_response::WorkerStatus>,
}
pub mod list_workers_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WorkerStatus {
        //// Worker ID, starting from 0
        #[prost(int64, tag = "1")]
        pub wid: i64,
        //// CPU core ID on which the worker is pinned
        #[prost(int64, tag = "2")]
        pub core: i64,
        //// True if running, otherwise False.
        #[prost(bool, tag = "3")]
        pub running: bool,
        //// Number of traffic classes running on the worker
        #[prost(int64, tag = "4")]
        pub num_tcs: i64,
        //// Total number of packets that have been silently dropped on the worker.
        //// Silent drops happen when a module transmit packets via disconnected
        //// output gates.
        #[prost(int64, tag = "5")]
        pub silent_drops: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddWorkerRequest {
    //// Worker ID to be added
    #[prost(int64, tag = "1")]
    pub wid: i64,
    //// CPU core ID on which the worker would run
    #[prost(int64, tag = "2")]
    pub core: i64,
    //// Empty string denotes default scheduler.
    #[prost(string, tag = "3")]
    pub scheduler: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyWorkerRequest {
    //// Worker ID
    #[prost(int64, tag = "1")]
    pub wid: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficClass {
    //// Name of parent TC
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    //// Name of TC
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    //// Is it running or ready to run at the moment?
    #[prost(bool, tag = "3")]
    pub blocked: bool,
    //// One of "priority", "weighted_fair", "round_robin", "rate_limit", "leaf"
    #[prost(string, tag = "4")]
    pub policy: std::string::String,
    //// Type of resource to regulate. Only used for traffic classes of
    //// weighted_fair and rate_limit types.
    //// Should be one of resource types: "count", "cycle", "packet", "bit"
    #[prost(string, tag = "5")]
    pub resource: std::string::String,
    //// Worker ID that this TC belongs to. If -1, the TC will be assigned
    //// to an arbitrary worker.
    #[prost(int64, tag = "8")]
    pub wid: i64,
    //  FIXME: If one can specify limit for only one resource type per TC,
    //         these two fields shouldn't be a map.

    //// Long-term average of resource limit, in cycles/s, packets/s, ...
    #[prost(map = "string, int64", tag = "9")]
    pub limit: ::std::collections::HashMap<std::string::String, i64>,
    //// Burst allowance of resource limit, in cycles, packets, bits, ...
    //// If set to 0, no extra tokens will be saved.
    #[prost(map = "string, int64", tag = "10")]
    pub max_burst: ::std::collections::HashMap<std::string::String, i64>,
    //// Only for "leaf": the task executed by this class.
    #[prost(string, tag = "11")]
    pub leaf_module_name: std::string::String,
    #[prost(uint64, tag = "12")]
    pub leaf_module_taskid: u64,
    #[prost(oneof = "traffic_class::Arg", tags = "6, 7")]
    pub arg: ::std::option::Option<traffic_class::Arg>,
}
pub mod traffic_class {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Arg {
        //// Used by "priority". Lower number == high priority.
        ///  FIXME: should be higher number == higher priority, to be consistent
        ///         other uses of "priority" in BESS
        #[prost(int64, tag = "6")]
        Priority(i64),
        //// Relative weight (share), used by "weighted_fair".
        //// 1 <= share <= 1024 is recommended. Higher number will result in
        //// lower scheduling accuracy.
        #[prost(int64, tag = "7")]
        Share(i64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTcsRequest {
    //// Specify a worker thread to fetch traffic classes.
    //// To include all traffic classes on every worker, specify -1.
    #[prost(int64, tag = "1")]
    pub wid: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTcsResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    #[prost(message, repeated, tag = "2")]
    pub classes_status: ::std::vec::Vec<list_tcs_response::TrafficClassStatus>,
}
pub mod list_tcs_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrafficClassStatus {
        #[prost(message, optional, tag = "1")]
        pub class: ::std::option::Option<super::TrafficClass>,
        //// Name of its parent TC
        #[prost(string, tag = "2")]
        pub parent: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckSchedulingConstraintsResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    #[prost(bool, tag = "2")]
    pub fatal: bool,
    #[prost(message, repeated, tag = "3")]
    pub violations: ::std::vec::Vec<check_scheduling_constraints_response::ViolatingClass>,
    #[prost(message, repeated, tag = "4")]
    pub modules: ::std::vec::Vec<check_scheduling_constraints_response::ViolatingModule>,
}
pub mod check_scheduling_constraints_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ViolatingClass {
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        #[prost(int32, tag = "2")]
        pub constraint: i32,
        #[prost(int32, tag = "3")]
        pub assigned_node: i32,
        #[prost(int32, tag = "4")]
        pub assigned_core: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ViolatingModule {
        #[prost(string, tag = "1")]
        pub name: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTcRequest {
    #[prost(message, optional, tag = "1")]
    pub class: ::std::option::Option<TrafficClass>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTcParamsRequest {
    #[prost(message, optional, tag = "1")]
    pub class: ::std::option::Option<TrafficClass>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTcParentRequest {
    #[prost(message, optional, tag = "1")]
    pub class: ::std::option::Option<TrafficClass>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTcStatsRequest {
    //// Name of TC
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTcStatsResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// The time that stat counters were read
    #[prost(double, tag = "2")]
    pub timestamp: f64,
    //// THe following counters represent the total amount of accumulated resource
    //// usage of a module since its creation.
    ///
    //// # of scheduled times
    #[prost(uint64, tag = "3")]
    pub count: u64,
    //// CPU cycles
    #[prost(uint64, tag = "4")]
    pub cycles: u64,
    //// # of packets
    #[prost(uint64, tag = "5")]
    pub packets: u64,
    //// # of bits
    #[prost(uint64, tag = "6")]
    pub bits: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDriversResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// List of availabe port drivers
    #[prost(string, repeated, tag = "2")]
    pub driver_names: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDriverInfoRequest {
    //// Name of port driver
    #[prost(string, tag = "1")]
    pub driver_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDriverInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// Name of port driver
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    //// 1-line description of the driver
    #[prost(string, tag = "3")]
    pub help: std::string::String,
    //// List of supported commands (TODO)
    #[prost(string, repeated, tag = "4")]
    pub commands: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPortsResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// List of all existing ports
    #[prost(message, repeated, tag = "2")]
    pub ports: ::std::vec::Vec<list_ports_response::Port>,
}
pub mod list_ports_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Port {
        //// Name of port
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        //// Name of port driver.
        #[prost(string, tag = "2")]
        pub driver: std::string::String,
        //// MAC address of the port
        #[prost(string, tag = "3")]
        pub mac_addr: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePortRequest {
    //// Name of the port to create. Every port must have a unique name.
    //// If not specified, a default name will be assigned
    //// (returned via CreatePortResponse for future reference).
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    //// Name of port driver. Must be specified.
    #[prost(string, tag = "2")]
    pub driver: std::string::String,
    //// Number of incoming/RX queues (Ext -> BESS). Default is 1
    #[prost(uint64, tag = "3")]
    pub num_inc_q: u64,
    //// Number of outgoind/TX queues (BESS -> Ext). Default is 1
    #[prost(uint64, tag = "4")]
    pub num_out_q: u64,
    //// Size of each incoming queue (# of packets).
    //// If not set (0), a driver-specific default value will be used.
    #[prost(uint64, tag = "5")]
    pub size_inc_q: u64,
    //// Size of each incoming queue (# of packets).
    //// If not set (0), a driver-specific default value will be used.
    #[prost(uint64, tag = "6")]
    pub size_out_q: u64,
    //// Driver specific argument for port initialization. See port_msg.proto
    #[prost(message, optional, tag = "7")]
    pub arg: ::std::option::Option<::prost_types::Any>,
}
//// All configuration parameters updatable at runtime
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortConf {
    //// MAC address for the new port. Should be "xx:xx:xx:xx:xx:xx" format.
    //// Set to '00:00:00:00:00:00' to use the default/current MAC address
    #[prost(string, tag = "1")]
    pub mac_addr: std::string::String,
    //// Port MTU. Set to 0 to accept the default port MTU
    #[prost(uint32, tag = "2")]
    pub mtu: u32,
    //// Enable or disable the port.
    //// Both admin and link (operational) states must be up to function
    #[prost(bool, tag = "3")]
    pub admin_up: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPortConfRequest {
    //// Name of port
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub conf: ::std::option::Option<PortConf>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPortConfRequest {
    //// Name of port
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPortConfResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    #[prost(message, optional, tag = "2")]
    pub conf: ::std::option::Option<PortConf>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePortResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// Name of the created port (specified or default one)
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    //// Port MAC address (specified or default one)
    #[prost(string, tag = "3")]
    pub mac_addr: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyPortRequest {
    //// Name of port
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPortStatsRequest {
    //// Name of port
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPortStatsResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// Port stats for incoming (Ext -> BESS) direction.
    #[prost(message, optional, tag = "2")]
    pub inc: ::std::option::Option<get_port_stats_response::Stat>,
    //// Port stats for outgoing (BESS -> Ext) direction.
    #[prost(message, optional, tag = "3")]
    pub out: ::std::option::Option<get_port_stats_response::Stat>,
    //// Time that stat counters were read.
    #[prost(double, tag = "4")]
    pub timestamp: f64,
}
pub mod get_port_stats_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Stat {
        //// Number of objects that have been successfully sent/received.
        //// All counters shows the accumulated value since port initialization.
        #[prost(uint64, tag = "1")]
        pub packets: u64,
        //// Number of dropped packets.
        //// For incoming direction, it implies BESS is not picking up fast enough.
        //// For outgoing direction, non-zero drop counter indicates that the "peer"
        //// of this port is the performance bottleneck: namely, VMs/containers/apps
        //// for virtual ports, or PCIe/NIC/link for physical port.
        #[prost(uint64, tag = "2")]
        pub dropped: u64,
        //// Total number of bytes, not including Frame CRC or Ethernet overheads
        #[prost(uint64, tag = "3")]
        pub bytes: u64,
        /// Histogram of how many times a given number of packets in a batch was
        /// requested.
        #[prost(uint64, repeated, tag = "4")]
        pub requested_hist: ::std::vec::Vec<u64>,
        /// Histogram of how many times a given number of packets in a batch were
        /// actually processed.
        #[prost(uint64, repeated, tag = "5")]
        pub actual_hist: ::std::vec::Vec<u64>,
        /// Histogram of the difference between the requested batch size and the
        /// actual number of packets processed in that batch.
        #[prost(uint64, repeated, tag = "6")]
        pub diff_hist: ::std::vec::Vec<u64>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLinkStatusRequest {
    //// name of the port to query
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLinkStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// speed in mbps: 1000, 40000, etc. 0 for vports
    #[prost(uint32, tag = "2")]
    pub speed: u32,
    //// full-duplex enabled?
    #[prost(bool, tag = "3")]
    pub full_duplex: bool,
    //// auto-negotiated speed and duplex?
    #[prost(bool, tag = "4")]
    pub autoneg: bool,
    //// link up?
    #[prost(bool, tag = "5")]
    pub link_up: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMclassResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// List of module types
    #[prost(string, repeated, tag = "2")]
    pub names: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMclassInfoRequest {
    //// Name of module type
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMclassInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// Name of module type
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    //// 1=line description of the module type
    #[prost(string, tag = "3")]
    pub help: std::string::String,
    //// List of commands supported by the module
    #[prost(string, repeated, tag = "4")]
    pub cmds: ::std::vec::Vec<std::string::String>,
    //// Corresponding Protobuf message types
    #[prost(string, repeated, tag = "5")]
    pub cmd_args: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModulesResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// List of all existing modules
    #[prost(message, repeated, tag = "2")]
    pub modules: ::std::vec::Vec<list_modules_response::Module>,
}
pub mod list_modules_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Module {
        //// Name of module
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        //// Module type
        #[prost(string, tag = "2")]
        pub mclass: std::string::String,
        //// Current status of module as a short, 1-line string
        #[prost(string, tag = "3")]
        pub desc: std::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModuleRequest {
    //// Name of the module to create. Every module must have a unique name.
    //// If not specified, a default name will be assigned
    //// (returned via CreateModuleResponse for future reference).
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    //// Name of module type. Must be specified.
    #[prost(string, tag = "2")]
    pub mclass: std::string::String,
    //// Protobuf message to be used for module initialization.
    //// See module_msg.proto for the argument message types.
    #[prost(message, optional, tag = "3")]
    pub arg: ::std::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModuleResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// Name of the created module (specified or default one)
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyModuleRequest {
    //// Name of module to remove
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModuleInfoRequest {
    //// Name of module to query
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModuleInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// Name of module
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    //// Module type
    #[prost(string, tag = "3")]
    pub mclass: std::string::String,
    //// Current status of module as a short, 1-line string
    #[prost(string, tag = "4")]
    pub desc: std::string::String,
    //// List of connected input gates
    #[prost(message, repeated, tag = "6")]
    pub igates: ::std::vec::Vec<get_module_info_response::IGate>,
    //// List of connected output gates
    #[prost(message, repeated, tag = "7")]
    pub ogates: ::std::vec::Vec<get_module_info_response::OGate>,
    //// List of metadata used by the module
    #[prost(message, repeated, tag = "8")]
    pub metadata: ::std::vec::Vec<get_module_info_response::Attribute>,
}
pub mod get_module_info_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IGate {
        //// Input gate ID
        #[prost(uint64, tag = "1")]
        pub igate: u64,
        //// The list of upstream output gates
        #[prost(message, repeated, tag = "2")]
        pub ogates: ::std::vec::Vec<i_gate::OGate>,
        //// # of packet batches seen
        #[prost(uint64, tag = "3")]
        pub cnt: u64,
        //// # of packets seen
        #[prost(uint64, tag = "4")]
        pub pkts: u64,
        //// # of bytes seen
        #[prost(uint64, tag = "5")]
        pub bytes: u64,
        //// The time that cnt/pkts counters were read
        #[prost(double, tag = "6")]
        pub timestamp: f64,
    }
    pub mod i_gate {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct OGate {
            //// Output gate of "previous" module
            #[prost(uint64, tag = "1")]
            pub ogate: u64,
            //// Name of "previous" module
            #[prost(string, tag = "2")]
            pub name: std::string::String,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OGate {
        //// Output gate ID
        #[prost(uint64, tag = "1")]
        pub ogate: u64,
        //// # of packet batches seen
        #[prost(uint64, tag = "2")]
        pub cnt: u64,
        //// # of packets seen
        #[prost(uint64, tag = "3")]
        pub pkts: u64,
        //// # of bytes seen
        #[prost(uint64, tag = "4")]
        pub bytes: u64,
        //// The time thatcnt/pkts counters were read
        #[prost(double, tag = "5")]
        pub timestamp: f64,
        //// Name of the "next" module it connects to
        #[prost(string, tag = "6")]
        pub name: std::string::String,
        //// Input gate ID of the "next" module
        #[prost(uint64, tag = "7")]
        pub igate: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attribute {
        //// Name of per-packet metadata attribute
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        //// Size of attribute (in bytes)
        #[prost(uint64, tag = "2")]
        pub size: u64,
        //// "read", "write", or "update"
        #[prost(string, tag = "3")]
        pub mode: std::string::String,
        //// (internal debugging purpose only)
        #[prost(int64, tag = "4")]
        pub offset: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectModulesRequest {
    //// Name of "previous" module name
    #[prost(string, tag = "1")]
    pub m1: std::string::String,
    //// name of "next" module name
    #[prost(string, tag = "2")]
    pub m2: std::string::String,
    //// m1's output gate ID
    #[prost(uint64, tag = "3")]
    pub ogate: u64,
    //// m2's input gate ID
    #[prost(uint64, tag = "4")]
    pub igate: u64,
    //// If true do not attach default hooks at the input/output gate.
    //// (Currently, the only default hook is the "track" hook at the ogate)
    #[prost(bool, tag = "5")]
    pub skip_default_hooks: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectModulesRequest {
    //// Name of previous module
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    //// Output gate ID of previous module
    #[prost(uint64, tag = "2")]
    pub ogate: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MempoolDump {
    //// The socket this mempool belongs to
    #[prost(int32, tag = "1")]
    pub socket: i32,
    //// True when this mempool has been initialized
    #[prost(bool, tag = "2")]
    pub initialized: bool,
    //// The maximum size of this mempool
    #[prost(uint32, tag = "3")]
    pub mp_size: u32,
    //// Size of per-lcore default local cache.
    #[prost(uint32, tag = "4")]
    pub mp_cache_size: u32,
    //// Size of one element
    #[prost(uint32, tag = "5")]
    pub mp_element_size: u32,
    //// Number of populated objects
    #[prost(uint32, tag = "6")]
    pub mp_populated_size: u32,
    //// Number of entries in this mempool
    #[prost(uint32, tag = "7")]
    pub mp_available_count: u32,
    //// Number of elements which have been allocated from this mempool
    #[prost(uint32, tag = "8")]
    pub mp_in_use_count: u32,
    //// Number of entries in the backing ring
    #[prost(uint32, tag = "9")]
    pub ring_count: u32,
    //// Number of free entries in the backing ring
    #[prost(uint32, tag = "10")]
    pub ring_free_count: u32,
    //// Size of the backing ring in bytes
    #[prost(uint64, tag = "11")]
    pub ring_bytes: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DumpMempoolRequest {
    /// ID of the socket whose mempool should be dumped. -1 for all sockets
    #[prost(int32, tag = "1")]
    pub socket: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DumpMempoolResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// The list of requested mempool dumps
    #[prost(message, repeated, tag = "2")]
    pub dumps: ::std::vec::Vec<MempoolDump>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    //// Name of module/port/driver
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    //// Name of command
    #[prost(string, tag = "2")]
    pub cmd: std::string::String,
    //// Command argument
    #[prost(message, optional, tag = "3")]
    pub arg: ::std::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    //// Command response (see *_msg.proto)
    #[prost(message, optional, tag = "2")]
    pub data: ::std::option::Option<::prost_types::Any>,
}
//  -------------------------------------------------------------------------
//  Gate hooks
//  -------------------------------------------------------------------------

//// Enable/Disable the "track" hook on a gate (or all gates)
////
//// "track" hook accumulates the number of total packets, batches and bits
////  passing through a gate. This incurs some amount of CPU overheads. While
////  the cost is very small, remember that the delay adds up at every gate.
////
//// NOTE: There should be no running worker to run this command.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackArg {
    //// Tracks bits too if True, else only packets and batches
    #[prost(bool, tag = "5")]
    pub bits: bool,
}
//// Enable/Disable tcpdump tapping at an input/output gate.
////
//// Once the tap is installed, all packets going through the gate will be
//// captured and sent in PCAP format to the specified named pipe (FIFO).
//// Thus you can run `tcpdump -r <path to FIFO>` or save the stream in a file.
//// This feature may affect performance.
////
//// NOTE: There should be no running worker to run this command.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpdumpArg {
    //// Path to the FIFO file.
    #[prost(string, tag = "5")]
    pub fifo: std::string::String,
    //// If set, we'll defer opening the FIFO.
    #[prost(bool, tag = "6")]
    pub defer: bool,
    //// If set, we'll reconnect after failure.
    #[prost(bool, tag = "7")]
    pub reconnect: bool,
}
//// Enable/Disable pcapng tapping at an input/output gate.
////
//// Once the tap is installed, all packets going through the gate will be
//// captured and sent in pcapng format to the specified named pipe (FIFO).
//// Unlike the Tcpdump hook, this also dumps a textual metadata representation,
//// in the form of a comment to the Enhanced Packet Block. Thus you can run
//// `tcpdump -r <path to FIFO>` or save the stream in a file.
//// This feature may affect performance.
////
//// NOTE: There should be no running worker to run this command.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PcapngArg {
    //// Path to the FIFO file.
    #[prost(string, tag = "5")]
    pub fifo: std::string::String,
    //// If set, we'll defer opening the FIFO.
    #[prost(bool, tag = "6")]
    pub defer: bool,
    //// If set, we'll reconnect after failure.
    #[prost(bool, tag = "7")]
    pub reconnect: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateHookInfo {
    //// Name of the hook
    #[prost(string, tag = "1")]
    pub hook_name: std::string::String,
    //// Name of module
    #[prost(string, tag = "2")]
    pub module_name: std::string::String,
    //// Hook-specific arguments
    #[prost(message, optional, tag = "5")]
    pub arg: ::std::option::Option<::prost_types::Any>,
    #[prost(oneof = "gate_hook_info::Gate", tags = "3, 4")]
    pub gate: ::std::option::Option<gate_hook_info::Gate>,
}
pub mod gate_hook_info {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Gate {
        //// Input gate index. All input gates if -1
        #[prost(int64, tag = "3")]
        Igate(i64),
        //// Output gate index. All output gates if -1
        #[prost(int64, tag = "4")]
        Ogate(i64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureGateHookRequest {
    #[prost(message, optional, tag = "1")]
    pub hook: ::std::option::Option<GateHookInfo>,
    #[prost(bool, tag = "2")]
    pub enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGateHooksResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<Error>,
    #[prost(message, repeated, tag = "2")]
    pub hooks: ::std::vec::Vec<GateHookInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GateHookCommandRequest {
    /// N.B.: this includes the command argument
    #[prost(message, optional, tag = "1")]
    pub hook: ::std::option::Option<GateHookInfo>,
    #[prost(string, tag = "2")]
    pub cmd: std::string::String,
}
//  -------------------------------------------------------------------------
//  Resume hooks
//  -------------------------------------------------------------------------

/// Enable/Disable a global ResumeHook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigureResumeHookRequest {
    //// Name of the hook
    #[prost(string, tag = "1")]
    pub hook_name: std::string::String,
    //// Installs the hook if True, else uninstalls
    #[prost(bool, tag = "2")]
    pub enable: bool,
    //// Hook-specific arguments
    #[prost(message, optional, tag = "3")]
    pub arg: ::std::option::Option<::prost_types::Any>,
}
//  -------------------------------------------------------------------------
//  Worker maniuplation
//  -------------------------------------------------------------------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseWorkerRequest {
    //// ID of the worker to be paused
    #[prost(int64, tag = "1")]
    pub wid: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeWorkerRequest {
    //// ID of the worker to be resumed
    #[prost(int64, tag = "1")]
    pub wid: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// 0 for success, errno (>0) for failure
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub errmsg: std::string::String,
}
// Module-specific messages.
// The header generated from this file should not be included in the BESS core
// source code.

// For your comments to come out in the auto-documentation:
// Format comments with two stars at the top, or use three slashes (///)
// Anything you write will show up as markdown, so feel free to add italics, etc.

/// The module_msg.proto file is stored in `bess/protobuf/` and it supplies the glue between
/// bessd modules and the outside world via GRPC.
/// bessctl uses GRPC to update modules. Whenever you call a function in bessctl, a corresponding function
/// is called on modules in bessd. This file lists all modules, their initialization parameters
/// and any functions that may be called on them.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyArg {}
///*
/// The BPF module has a command `clear()` that takes no parameters.
/// This command removes all filters from the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpfCommandClearArg {}
///*
/// The ExactMatch module has a command `add(...)` that takes two parameters.
/// The ExactMatch initializer specifies what fields in a packet to inspect; add() specifies
/// which values to check for over these fields.
/// add() inserts a new rule into the ExactMatch module such that traffic matching i
/// that bytestring will be forwarded
/// out a specified gate.
/// Example use: `add(fields=[aton('12.3.4.5'), aton('5.4.3.2')], gate=2)`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExactMatchCommandAddArg {
    //// The gate to forward out packets that mach this rule.
    #[prost(uint64, tag = "1")]
    pub gate: u64,
    //// The exact match values to check for
    #[prost(message, repeated, tag = "2")]
    pub fields: ::std::vec::Vec<FieldData>,
}
///*
/// The ExactMatch module has a command `delete(...)` which deletes an existing rule.
/// Example use: `delete(fields=[aton('12.3.4.5'), aton('5.4.3.2')])`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExactMatchCommandDeleteArg {
    //// The field values for the rule to be deleted.
    #[prost(message, repeated, tag = "2")]
    pub fields: ::std::vec::Vec<FieldData>,
}
///*
/// The ExactMatch module has a command `clear()` which takes no parameters.
/// This command removes all rules from the ExactMatch module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExactMatchCommandClearArg {}
///*
/// The ExactMatch module has a command `set_default_gate(...)` which takes one parameter.
/// This command routes all traffic which does _not_ match a rule to a specified gate.
/// Example use in bessctl: `setDefaultGate(gate=2)`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExactMatchCommandSetDefaultGateArg {
    //// The gate number to send the default traffic out.
    #[prost(uint64, tag = "1")]
    pub gate: u64,
}
///*
/// The FlowGen module has a command `set_burst(...)` that allows you to specify
/// the maximum number of packets to be stored in a single PacketBatch released
/// by the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowGenCommandSetBurstArg {
    #[prost(uint64, tag = "1")]
    pub burst: u64,
}
///*
/// The HashLB module has a command `set_mode(...)` which takes two parameters.
/// The `mode` parameter specifies whether the load balancer will hash over the
/// src/dest ethernet header (l2), over the src/dest IP addresses (l3), or over
/// the flow 5-tuple (l4).  Alternatively, if the `fields` parameter is set, the
/// load balancer will hash over the N-tuple with the specified offsets and
/// sizes.
/// Example use in bessctl: `lb.set_mode('l2')`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashLbCommandSetModeArg {
    //// What fields to hash over, l2, l3, and l4 are only valid values.
    #[prost(string, tag = "1")]
    pub mode: std::string::String,
    //// A list of fields that define a custom tuple.
    #[prost(message, repeated, tag = "2")]
    pub fields: ::std::vec::Vec<Field>,
}
///*
/// The HashLB module has a command `set_gates(...)` which takes one parameter.
/// This function takes in a list of gate numbers to send hashed traffic out over.
/// Example use in bessctl: `lb.setGates(gates=[0,1,2,3])`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashLbCommandSetGatesArg {
    ////A list of gate numbers to load balance traffic over
    #[prost(int64, repeated, tag = "1")]
    pub gates: ::std::vec::Vec<i64>,
}
///*
/// The IPLookup module has a command `add(...)` which takes three paramters.
/// This function accepts the routing rules -- CIDR prefix, CIDR prefix length,
/// and what gate to forward matching traffic out on.
/// Example use in bessctl: `table.add(prefix='10.0.0.0', prefix_len=8, gate=2)`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpLookupCommandAddArg {
    //// The CIDR IP part of the prefix to match
    #[prost(string, tag = "1")]
    pub prefix: std::string::String,
    //// The prefix length
    #[prost(uint64, tag = "2")]
    pub prefix_len: u64,
    //// The number of the gate to forward matching traffic on.
    #[prost(uint64, tag = "3")]
    pub gate: u64,
}
///*
/// The IPLookup module has a command `delete(...)` which takes two paramters.
/// This function accepts the routing rules -- CIDR prefix, CIDR prefix length,
/// Example use in bessctl: `table.delete(prefix='10.0.0.0', prefix_len=8)`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpLookupCommandDeleteArg {
    //// The CIDR IP part of the prefix to match
    #[prost(string, tag = "1")]
    pub prefix: std::string::String,
    //// The prefix length
    #[prost(uint64, tag = "2")]
    pub prefix_len: u64,
}
///*
/// The IPLookup module has a command `clear()` which takes no parameters.
/// This function removes all rules in the IPLookup table.
/// Example use in bessctl: `myiplookuptable.clear()`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpLookupCommandClearArg {}
///*
/// The L2Forward module forwards traffic via exact match over the Ethernet
/// destination address. The command `add(...)`  allows you to specifiy a
/// MAC address and which gate the L2Forward module should direct it out of.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct L2ForwardCommandAddArg {
    //// A list of L2Forward entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::std::vec::Vec<l2_forward_command_add_arg::Entry>,
}
pub mod l2_forward_command_add_arg {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        //// The MAC address to match
        #[prost(string, tag = "1")]
        pub addr: std::string::String,
        //// Which gate to send out traffic matching this address.
        #[prost(int64, tag = "2")]
        pub gate: i64,
    }
}
///*
/// The L2Forward module has a function `delete(...)` to remove a rule
/// from the MAC forwarding table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct L2ForwardCommandDeleteArg {
    //// The address to remove from the forwarding table
    #[prost(string, repeated, tag = "1")]
    pub addrs: ::std::vec::Vec<std::string::String>,
}
///*
/// For traffic reaching the L2Forward module which does not match a MAC rule,
/// the function `set_default_gate(...)` allows you to specify a default gate
/// to direct unmatched traffic to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct L2ForwardCommandSetDefaultGateArg {
    //// The default gate to forward traffic which matches no entry to.
    #[prost(int64, tag = "1")]
    pub gate: i64,
}
///*
/// The L2Forward module has a function `lookup(...)` to query what output gate
/// a given MAC address will be forwared to; it returns the gate ID number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct L2ForwardCommandLookupArg {
    //// The MAC address to query for
    #[prost(string, repeated, tag = "1")]
    pub addrs: ::std::vec::Vec<std::string::String>,
}
///*
/// This message type provides the reponse to the L2Forward function `lookup(..)`.
/// It returns the gate that a requested MAC address is currently assigned to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct L2ForwardCommandLookupResponse {
    //// The gate ID that the requested MAC address maps to
    #[prost(uint64, repeated, tag = "1")]
    pub gates: ::std::vec::Vec<u64>,
}
///*
/// The L2Forward module has a command `populate(...)` which allows for fast creation
/// of the forwarding table given a range of MAC addresses. The function takes in a
/// 'base' MAC address, a count (number of MAC addresses), and a gate_id. The module
/// will route all MAC addresses starting from the base address, up to base+count address
/// round-robin over gate_count total gates.
/// For example, `populate(base='11:22:33:44:00', count = 10, gate_count = 2) would
/// route addresses 11:22:33:44::(00, 02, 04, 06, 08) out a gate 0 and the odd-suffixed
/// addresses out gate 1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct L2ForwardCommandPopulateArg {
    //// The base MAC address
    #[prost(string, tag = "1")]
    pub base: std::string::String,
    //// How many addresses beyond base to populate into the routing table
    #[prost(int64, tag = "2")]
    pub count: i64,
    //// How many gates to create in the L2Forward module.
    #[prost(int64, tag = "3")]
    pub gate_count: i64,
}
///*
/// The Measure module measures and collects latency/jitter data for packets
/// annotated by a Timestamp module. Note that Timestamp and Measure module must reside
/// on the server for accurate measurement (as a result, the most typical use case is
/// measuring roundtrip time).
/// Optionally, you can also retrieve percentile values by specifying points in
/// "percentiles". For example, "percentiles" of [50.0, 99.0] will return
/// [median, 99'th %-ile tail latency] in "percentile_values_ns" in the response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasureCommandGetSummaryArg {
    //// if true, the data will be all cleared after read
    #[prost(bool, tag = "1")]
    pub clear: bool,
    //// ascending list of real numbers in [0.0, 100.0]
    #[prost(double, repeated, tag = "2")]
    pub latency_percentiles: ::std::vec::Vec<f64>,
    //// ascending list of real numbers in [0.0, 100.0]
    #[prost(double, repeated, tag = "3")]
    pub jitter_percentiles: ::std::vec::Vec<f64>,
}
///*
/// The Measure module function `get_summary()` returns the following values.
/// Note that the resolution value tells you how grainy the samples are,
/// e.g., 100 means that anything from 0-99 ns counts as "0",
/// anything from 100-199 counts as "100", and so on.  The average
/// is of samples using this graininess, but (being a result of division)
/// may not be a multiple of the resolution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasureCommandGetSummaryResponse {
    //// Seconds since boot.
    #[prost(double, tag = "1")]
    pub timestamp: f64,
    //// Total # of packets seen by this module.
    #[prost(uint64, tag = "2")]
    pub packets: u64,
    //// Total # of bits seen by this module.
    #[prost(uint64, tag = "3")]
    pub bits: u64,
    #[prost(message, optional, tag = "4")]
    pub latency: ::std::option::Option<measure_command_get_summary_response::Histogram>,
    #[prost(message, optional, tag = "5")]
    pub jitter: ::std::option::Option<measure_command_get_summary_response::Histogram>,
}
pub mod measure_command_get_summary_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Histogram {
        //// Total # of measured data points, including above_range
        #[prost(uint64, tag = "1")]
        pub count: u64,
        //// # of data points for the "too large value" bucket
        #[prost(uint64, tag = "2")]
        pub above_range: u64,
        //// resolution of measured data
        #[prost(uint64, tag = "8")]
        pub resolution_ns: u64,
        #[prost(uint64, tag = "3")]
        pub min_ns: u64,
        #[prost(uint64, tag = "4")]
        pub avg_ns: u64,
        #[prost(uint64, tag = "5")]
        pub max_ns: u64,
        #[prost(uint64, tag = "6")]
        pub total_ns: u64,
        #[prost(uint64, repeated, tag = "7")]
        pub percentile_values_ns: ::std::vec::Vec<u64>,
    }
}
///*
/// The Module DRR provides fair scheduling of flows based on a quantum which is
/// number of bytes allocated to each flow on each round of going through all flows.
/// Examples can be found [./bessctl/conf/samples/drr.bess]
/// __Input_Gates__: 1
/// __Output_Gates__:  1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DrrArg {
    //// Number of flows to handle in module
    #[prost(uint32, tag = "1")]
    pub num_flows: u32,
    //// the number of bytes to allocate to each on every round
    #[prost(uint64, tag = "2")]
    pub quantum: u64,
    //// the max size that any Flows queue can get
    #[prost(uint32, tag = "3")]
    pub max_flow_queue_size: u32,
}
///*
/// the SetQuantumSize function sets a new quantum for DRR module to operate on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DrrQuantumArg {
    //// the number of bytes to allocate to each on every round
    #[prost(uint32, tag = "1")]
    pub quantum: u32,
}
///*
/// The SetMaxQueueSize function sets a new maximum flow queue size for DRR module.
/// If the flow's queue gets to this size, the module starts dropping packets to
/// that flow until the queue is below this size.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DrrMaxFlowQueueSizeArg {
    //// the max size that any Flows queue can get
    #[prost(uint32, tag = "1")]
    pub max_queue_size: u32,
}
///*
/// The module PortInc has a function `set_burst(...)` that allows you to specify the
/// maximum number of packets to be stored in a single PacketBatch released by
/// the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortIncCommandSetBurstArg {
    //// The maximum "burst" of packets (ie, the maximum batch size)
    #[prost(uint64, tag = "1")]
    pub burst: u64,
}
///*
/// The module QueueInc has a function `set_burst(...)` that allows you to specify
/// the maximum number of packets to be stored in a single PacketBatch released
/// by the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueIncCommandSetBurstArg {
    //// The maximum "burst" of packets (ie, the maximum batch size)
    #[prost(uint64, tag = "1")]
    pub burst: u64,
}
///*
/// The module QueueInc has a function `set_burst(...)` that allows you to specify
/// the maximum number of packets to be stored in a single PacketBatch released
/// by the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueCommandSetBurstArg {
    //// The maximum "burst" of packets (ie, the maximum batch size)
    #[prost(uint64, tag = "1")]
    pub burst: u64,
}
///*
/// The module QueueInc has a function `set_size(...)` that allows specifying the
/// size of the queue in total number of packets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueCommandSetSizeArg {
    //// The maximum number of packets to store in the queue.
    #[prost(uint64, tag = "1")]
    pub size: u64,
}
///*
/// Modules that are queues or contain queues may contain functions
/// `get_status()` that return QueueCommandGetStatusResponse.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueCommandGetStatusArg {}
///*
/// Modules that are queues or contain queues may contain functions
/// `get_status()` that take no parameters and returns the queue occupancy and
/// size.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueCommandGetStatusResponse {
    //// The number of packets currently in the queue.
    #[prost(uint64, tag = "1")]
    pub count: u64,
    //// The maximum number of packets the queue can contain.
    #[prost(uint64, tag = "2")]
    pub size: u64,
    //// total enqueued
    #[prost(uint64, tag = "3")]
    pub enqueued: u64,
    //// total dequeued
    #[prost(uint64, tag = "4")]
    pub dequeued: u64,
    //// total dropped
    #[prost(uint64, tag = "5")]
    pub dropped: u64,
}
///*
/// The function `clear()` for RandomUpdate takes no parameters and clears all
/// state in the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RandomUpdateCommandClearArg {}
///*
/// The function `clear()` for Rewrite takes no parameters and clears all state
/// in the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewriteCommandClearArg {}
///*
/// The function `clear()` for Update takes no parameters and clears all state in
/// the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCommandClearArg {}
///*
/// The module WildcardMatch has a command `add(...)` which inserts a new rule
/// into the WildcardMatch module. For an example of code using WilcardMatch see
/// `bess/bessctl/conf/samples/wildcardmatch.bess`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WildcardMatchCommandAddArg {
    //// Traffic matching this new rule will be sent to this gate.
    #[prost(uint64, tag = "1")]
    pub gate: u64,
    ////If a packet matches multiple rules, the rule with higher priority will be applied. If priorities are equal behavior is undefined.
    #[prost(int64, tag = "2")]
    pub priority: i64,
    //// The values to check for in each fieild.
    #[prost(message, repeated, tag = "3")]
    pub values: ::std::vec::Vec<FieldData>,
    //// The bitmask for each field -- set 0x0 to ignore the field altogether.
    #[prost(message, repeated, tag = "4")]
    pub masks: ::std::vec::Vec<FieldData>,
}
///*
/// The module WildcardMatch has a command `delete(...)` which removes a rule -- simply specify the values and masks from the previously inserted rule to remove them.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WildcardMatchCommandDeleteArg {
    //// The values being checked for in the rule
    #[prost(message, repeated, tag = "1")]
    pub values: ::std::vec::Vec<FieldData>,
    //// The bitmask from the rule.
    #[prost(message, repeated, tag = "2")]
    pub masks: ::std::vec::Vec<FieldData>,
}
///*
/// The function `clear()` for WildcardMatch takes no parameters, it clears
/// all state in the WildcardMatch module (is equivalent to calling delete for all rules)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WildcardMatchCommandClearArg {}
///*
/// For traffic which does not match any rule in the WildcardMatch module,
/// the `set_default_gate(...)` function specifies which gate to send this extra traffic to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WildcardMatchCommandSetDefaultGateArg {
    #[prost(uint64, tag = "1")]
    pub gate: u64,
}
///*
/// The module ACL creates an access control module which by default blocks all traffic, unless it contains a rule which specifies otherwise.
/// Examples of ACL can be found in [acl.bess](https://github.com/NetSys/bess/blob/master/bessctl/conf/samples/acl.bess)
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AclArg {
    ////A list of ACL rules.
    #[prost(message, repeated, tag = "1")]
    pub rules: ::std::vec::Vec<acl_arg::Rule>,
}
pub mod acl_arg {
    ///*
    /// One ACL rule is represented by the following 6-tuple.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        //// Source IP block in CIDR. Wildcard if "".
        #[prost(string, tag = "1")]
        pub src_ip: std::string::String,
        //// Destination IP block in CIDR. Wildcard if "".
        #[prost(string, tag = "2")]
        pub dst_ip: std::string::String,
        //// TCP/UDP source port. Wildcard if 0.
        #[prost(uint32, tag = "3")]
        pub src_port: u32,
        //// TCP/UDP Destination port. Wildcard if 0.
        #[prost(uint32, tag = "4")]
        pub dst_port: u32,
        //// Not implemented
        #[prost(bool, tag = "5")]
        pub established: bool,
        //// Drop matched packets if true, forward if false. By default ACL drops all traffic.
        #[prost(bool, tag = "6")]
        pub drop: bool,
    }
}
///*
/// The BPF module is an access control module that sends packets out on a particular gate based on whether they match a BPF filter.
///
/// __Input Gates__: 1
/// __Output Gates__: many (configurable)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BpfArg {
    //// The BPF initialized function takes a list of BPF filters.
    #[prost(message, repeated, tag = "1")]
    pub filters: ::std::vec::Vec<bpf_arg::Filter>,
}
pub mod bpf_arg {
    ///*
    /// One BPF filter is represented by the following 3-tuple.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        //// The priority level for this rule. If a packet matches multiple rules, it will be forwarded out the gate with the highest priority. If a packet matches multiple rules with the same priority, the behavior is undefined.
        #[prost(int64, tag = "1")]
        pub priority: i64,
        //// The actual BPF string.
        #[prost(string, tag = "2")]
        pub filter: std::string::String,
        ////What gate to forward packets that match this BPF to.
        #[prost(int64, tag = "3")]
        pub gate: i64,
    }
}
///*
/// The Buffer module takes no parameters to initialize (ie, `Buffer()` is sufficient to create one).
/// Buffer accepts packets and stores them; it may forard them to the next module only after it has
/// received enough packets to fill an entire PacketBatch.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferArg {}
///*
/// The Bypass module forwards packets by emulating pre-defined packet processing overhead.
/// It burns cpu cycles per_batch, per_packet, and per-bytes.
/// Bypass is useful primarily for testing and performance evaluation.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BypassArg {
    #[prost(uint32, tag = "1")]
    pub cycles_per_batch: u32,
    #[prost(uint32, tag = "2")]
    pub cycles_per_packet: u32,
    #[prost(uint32, tag = "3")]
    pub cycles_per_byte: u32,
}
///*
/// The Dump module blindly forwards packets without modifying them. It periodically samples a packet and prints out out to the BESS log (by default stored in `/tmp/bessd.INFO`).
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DumpArg {
    ////How frequently to sample and print a packet, in seconds.
    #[prost(double, tag = "1")]
    pub interval: f64,
}
///*
/// The EtherEncap module wraps packets in an Ethernet header, but it takes no parameters. Instead, Ethernet source, destination, and type are pulled from a packet's metadata attributes.
/// For example: `SetMetadata('dst_mac', 11:22:33:44:55) -> EtherEncap()`
/// This is useful when upstream modules wish to assign a MAC address to a packet, e.g., due to an ARP request.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtherEncapArg {}
///*
/// The ExactMatch module splits packets along output gates according to exact match values in arbitrary packet fields.
/// To instantiate an ExactMatch module, you must specify which fields in the packet to match over. You can add rules using the function `ExactMatch.add(...)`
/// Fields may be stored either in the packet data or its metadata attributes.
/// An example script using the ExactMatch code is found
/// in [`bess/bessctl/conf/samples/exactmatch.bess`](https://github.com/NetSys/bess/blob/master/bessctl/conf/samples/exactmatch.bess).
///
/// __Input Gates__: 1
/// __Output Gates__: many (configurable)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExactMatchArg {
    ////A list of ExactMatch Fields
    #[prost(message, repeated, tag = "1")]
    pub fields: ::std::vec::Vec<Field>,
    //// mask(i) corresponds to the mask for field(i)
    #[prost(message, repeated, tag = "2")]
    pub masks: ::std::vec::Vec<FieldData>,
}
///*
/// ExactMatchConfig represents the current runtime configuration
/// of an ExactMatch module, as returned by get_runtime_config and
/// set by set_runtime_config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExactMatchConfig {
    #[prost(uint64, tag = "1")]
    pub default_gate: u64,
    #[prost(message, repeated, tag = "2")]
    pub rules: ::std::vec::Vec<ExactMatchCommandAddArg>,
}
///*
/// The FlowGen module generates simulated TCP flows of packets with correct SYN/FIN flags and sequence numbers.
/// This module is useful for testing, e.g., a NAT module or other flow-aware code.
/// Packets are generated off a base, "template" packet by modifying the IP src/dst and TCP src/dst. By default, only the ports are changed and will be modified by incrementing the template ports by up to 20000 more than the template values.
///
/// __Input Gates__: 0
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowGenArg {
    //// The packet "template". All data packets are derived from this template and contain the same payload.
    #[prost(bytes, tag = "1")]
    pub template: std::vec::Vec<u8>,
    //// The total number of packets per second to generate.
    #[prost(double, tag = "2")]
    pub pps: f64,
    //// The number of new flows to create every second. flow_rate must be <= pps.
    #[prost(double, tag = "3")]
    pub flow_rate: f64,
    //// The lifetime of a flow in seconds.
    #[prost(double, tag = "4")]
    pub flow_duration: f64,
    //// The packet arrival distribution -- must be either "uniform" or "exponential"
    #[prost(string, tag = "5")]
    pub arrival: std::string::String,
    //// The flow duration distribution -- must be either "uniform" or "pareto"
    #[prost(string, tag = "6")]
    pub duration: std::string::String,
    //// Whether or not to populate the flowgenerator with initial flows (start generating full pps rate immediately) or to wait for new flows to be generated naturally (all flows have a SYN packet).
    #[prost(bool, tag = "7")]
    pub quick_rampup: bool,
    //// When generating new flows, FlowGen modifies the template packet by changing the IP src, incrementing it by at most ip_src_range (e.g., if the base packet is 10.0.0.1 and range is 5, it will generate packets with IPs 10.0.0.1-10.0.0.6).
    #[prost(uint32, tag = "8")]
    pub ip_src_range: u32,
    //// When generating new flows, FlowGen modifies the template packet by changing the IP dst, incrementing it by at most ip_dst_range.
    #[prost(uint32, tag = "9")]
    pub ip_dst_range: u32,
    //// When generating new flows, FlowGen modifies the template packet by changing the TCP port, incrementing it by at most port_src_range.
    #[prost(uint32, tag = "10")]
    pub port_src_range: u32,
    //// When generating new flows, FlowGen modifies the template packet by changing the TCP dst port, incrementing it by at most port_dst_range.
    #[prost(uint32, tag = "11")]
    pub port_dst_range: u32,
}
///*
/// The GenericDecap module strips off the first few bytes of data from a packet.
///
/// __Input Gates__: 1
/// __Ouptut Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericDecapArg {
    //// The number of bytes to strip off.
    #[prost(uint64, tag = "1")]
    pub bytes: u64,
}
///*
/// The GenericEncap module adds a header to packets passing through it.
/// Takes a list of fields. Each field is either:
///
///  1. {'size': X, 'value': Y}          (for constant values)
///  2. {'size': X, 'attribute': Y}      (for metadata attributes)
///
/// e.g.: GenericEncap([{'size': 4, 'value': 0xdeadbeef},
///                     {'size': 2, 'attribute': 'foo'},
///                     {'size': 2, 'value': 0x1234}])
/// will prepend a 8-byte header:
///    de ad be ef <xx> <xx> 12 34
/// where the 2-byte <xx> <xx> comes from the value of metadata attribute 'foo'
/// for each packet.
/// An example script using GenericEncap is in [`bess/bessctl/conf/samples/generic_encap.bess`](https://github.com/NetSys/bess/blob/master/bessctl/conf/samples/generic_encap.bess).
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericEncapArg {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::std::vec::Vec<generic_encap_arg::EncapField>,
}
pub mod generic_encap_arg {
    ///*
    /// An EncapField represents one field in the new packet header.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EncapField {
        //// The length of the field.
        #[prost(uint64, tag = "1")]
        pub size: u64,
        #[prost(oneof = "encap_field::Insertion", tags = "2, 3")]
        pub insertion: ::std::option::Option<encap_field::Insertion>,
    }
    pub mod encap_field {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Insertion {
            //// The metadata attribute name to pull the field value from
            #[prost(string, tag = "2")]
            Attribute(std::string::String),
            //// Or, the fixed value to insert into the packet.
            #[prost(message, tag = "3")]
            Value(super::super::FieldData),
        }
    }
}
///*
/// The HashLB module partitions packets between output gates according to either
/// a hash over their MAC src/dst (mode=l2), their IP src/dst (mode=l3), the full
/// IP/TCP 5-tuple (mode=l4), or the N-tuple defined by `fields`.
///
/// __Input Gates__: 1
/// __Output Gates__: many (configurable)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashLbArg {
    //// A list of gate numbers over which to partition packets
    #[prost(int64, repeated, tag = "1")]
    pub gates: ::std::vec::Vec<i64>,
    //// The mode (l2, l3, or l4) for the hash function.
    #[prost(string, tag = "2")]
    pub mode: std::string::String,
    //// A list of fields that define a custom tuple.
    #[prost(message, repeated, tag = "3")]
    pub fields: ::std::vec::Vec<Field>,
}
///*
/// Encapsulates a packet with an IP header, where IP src, dst, and proto are filled in
/// by metadata values carried with the packet. Metadata attributes must include:
/// ip_src, ip_dst, ip_proto, ip_nexthop, and ether_type.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpEncapArg {}
///*
/// An IPLookup module perfroms LPM lookups over a packet destination.
/// IPLookup takes no parameters to instantiate.
/// To add rules to the IPLookup table, use `IPLookup.add()`
///
/// __Input Gates__: 1
/// __Output Gates__: many (configurable, depending on rule values)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpLookupArg {
    //// Maximum number of rules (default: 1024)
    #[prost(uint32, tag = "1")]
    pub max_rules: u32,
    //// Maximum number of IP prefixes with smaller than /24 (default: 128)
    #[prost(uint32, tag = "2")]
    pub max_tbl8s: u32,
}
///*
/// An L2Forward module forwards packets to an output gate according to exact-match rules over
/// an Ethernet destination.
/// Note that this is _not_ a learning switch -- forwards according to fixed
/// routes specified by `add(..)`.
///
/// __Input Gates__: 1
/// __Ouput Gates__: many (configurable, depending on rules)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct L2ForwardArg {
    //// Configures the forwarding hash table -- total number of hash table entries.
    #[prost(int64, tag = "1")]
    pub size: i64,
    //// Configures the forwarding hash table -- total number of slots per hash value.
    #[prost(int64, tag = "2")]
    pub bucket: i64,
}
///*
/// The MACSwap module takes no arguments. It swaps the src/destination MAC addresses
/// within a packet.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MacSwapArg {}
///*
/// The measure module tracks latencies, packets per second, and other statistics.
/// It should be paired with a Timestamp module, which attaches a timestamp to packets.
/// The measure module will log how long (in nanoseconds) it has been for each packet it received since it was timsestamped.
/// This module is somewhat experimental and undergoing various changes.
/// There is a test for the the Measure module in [`bessctl/module_tests/timestamp.py`](https://github.com/NetSys/bess/blob/master/bessctl/module_tests/timestamp.py).
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasureArg {
    /// int64 warmup = 1; /// removed: instead of warmup delay, user should Clear()
    ///
    //// Where to store the current time within the packet, offset in bytes.
    #[prost(uint64, tag = "2")]
    pub offset: u64,
    //// How often the module should sample packets for inter-packet arrival measurements (to measure jitter).
    #[prost(double, tag = "3")]
    pub jitter_sample_prob: f64,
    //// maximum latency expected, in ns (default 0.1 s)
    #[prost(uint64, tag = "4")]
    pub latency_ns_max: u64,
    //// resolution, in ns (default 100)
    #[prost(uint32, tag = "5")]
    pub latency_ns_resolution: u32,
}
///*
/// The merge module takes no parameters. It has multiple input gates,
/// and passes out all packets from a single output gate.
///
/// __Input Gates__: many (configurable)
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeArg {}
///*
/// The MetadataTest module is used for internal testing purposes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataTestArg {
    #[prost(map = "string, int64", tag = "1")]
    pub read: ::std::collections::HashMap<std::string::String, i64>,
    #[prost(map = "string, int64", tag = "2")]
    pub write: ::std::collections::HashMap<std::string::String, i64>,
    #[prost(map = "string, int64", tag = "3")]
    pub update: ::std::collections::HashMap<std::string::String, i64>,
}
///*
/// The NAT module implements Dynamic IPv4 address/port translation,
/// rewriting packet source addresses with external addresses as specified,
/// and destination addresses for packets on the reverse direction.
/// L3/L4 checksums are updated correspondingly.
/// To see an example of NAT in use, see:
/// [`bess/bessctl/conf/samples/nat.bess`](https://github.com/NetSys/bess/blob/master/bessctl/conf/samples/nat.bess)
///
/// Currently only supports TCP/UDP/ICMP.
/// Note that address/port in packet payload (e.g., FTP) are NOT translated.
///
/// __Input Gates__: 2 (0 for internal->external, and 1 for external->internal direction)
/// __Output Gates__: 2 (same as the input gate)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatArg {
    //// list of external IP addresses
    #[prost(message, repeated, tag = "1")]
    pub ext_addrs: ::std::vec::Vec<nat_arg::ExternalAddress>,
}
pub mod nat_arg {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PortRange {
        #[prost(uint32, tag = "1")]
        pub begin: u32,
        #[prost(uint32, tag = "2")]
        pub end: u32,
        #[prost(bool, tag = "3")]
        pub suspended: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExternalAddress {
        #[prost(string, tag = "1")]
        pub ext_addr: std::string::String,
        #[prost(message, repeated, tag = "2")]
        pub port_ranges: ::std::vec::Vec<PortRange>,
    }
}
///*
/// Static NAT module implements one-to-one translation of source/destination
/// IPv4 addresses. No port number is translated.
/// L3/L4 checksums are updated correspondingly.
/// To see an example of NAT in use, see:
/// [`bess/bessctl/conf/samples/nat.bess`](https://github.com/NetSys/bess/blob/master/bessctl/conf/samples/nat.bess)
///
/// Forward direction (from input gate 0 to output gate 0):
///  - Source IP address is updated, from internal to external address.
/// Reverse direction (from input gate 1 to output gate 1):
///  - Destination IP address is updated, from external to internal address.
/// If the original address is outside any of the ranges, packets are forwarded
/// without NAT.
///
/// Note that address in packet payload (e.g., FTP) are NOT translated.
///
/// __Input Gates__: 2 (0 for internal->external, and 1 for external->internal direction)
/// __Output Gates__: 2 (same as the input gate)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticNatArg {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::std::vec::Vec<static_nat_arg::AddressRangePair>,
}
pub mod static_nat_arg {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddressRange {
        //// first IP address to use
        #[prost(string, tag = "1")]
        pub start: std::string::String,
        //// last IP address to use
        #[prost(string, tag = "2")]
        pub end: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddressRangePair {
        #[prost(message, optional, tag = "1")]
        pub int_range: ::std::option::Option<AddressRange>,
        //// should be the same size as int_range
        #[prost(message, optional, tag = "2")]
        pub ext_range: ::std::option::Option<AddressRange>,
    }
}
///*
/// This module is used for testing purposes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoOpArg {}
///*
/// The PortInc module connects a physical or virtual port and releases
/// packets from it. PortInc does not support multiqueueing.
/// For details on how to configure PortInc using DPDK, virtual ports,
/// or libpcap, see the sidebar in the wiki.
///
/// __Input Gates__: 0
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortIncArg {
    //// The portname to connect to.
    #[prost(string, tag = "1")]
    pub port: std::string::String,
    //// Whether or not to prefetch packets from the port.
    #[prost(bool, tag = "2")]
    pub prefetch: bool,
}
///*
/// The PortOut module connects to a physical or virtual port and pushes
/// packets to it. For details on how to configure PortOut with DPDK,
/// virtual ports, libpcap, etc, see the sidebar in the wiki.
///
/// __Input Gates__: 1
/// __Output Gates__: 0
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortOutArg {
    //// The portname to connect to.
    #[prost(string, tag = "1")]
    pub port: std::string::String,
}
///*
/// The QueueInc produces input packets from a physical or virtual port.
/// Unlike PortInc, it supports multiqueue ports.
/// For details on how to configure QueueInc with DPDK, virtualports,
/// libpcap, etc, see the sidebar in the wiki.
///
/// __Input Gates__: 0
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueIncArg {
    //// The portname to connect to (read from).
    #[prost(string, tag = "1")]
    pub port: std::string::String,
    //// The queue on that port to read from. qid starts from 0.
    #[prost(uint64, tag = "2")]
    pub qid: u64,
    //// When prefetch is enabled, the module will perform CPU prefetch on the first 64B of each packet onto CPU L1 cache. Default value is false.
    #[prost(bool, tag = "3")]
    pub prefetch: bool,
}
///*
/// The QueueOut module releases packets to a physical or virtual port.
/// Unlike PortOut, it supports multiqueue ports.
/// For details on how to configure QueueOut with DPDK, virtualports,
/// libpcap, etc, see the sidebar in the wiki.
///
/// __Input Gates__: 1
/// __Output Gates__: 0
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueOutArg {
    //// The portname to connect to.
    #[prost(string, tag = "1")]
    pub port: std::string::String,
    //// The queue on that port to write out to.
    #[prost(uint64, tag = "2")]
    pub qid: u64,
}
///*
/// The Queue module implements a simple packet queue.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueArg {
    //// The maximum number of packets to store in the queue.
    #[prost(uint64, tag = "1")]
    pub size: u64,
    //// When prefetch is enabled, the module will perform CPU prefetch on the first 64B of each packet onto CPU L1 cache. Default value is false.
    #[prost(bool, tag = "2")]
    pub prefetch: bool,
    /// When backpressure is enabled, the module will notify upstream if it is overloaded.
    #[prost(bool, tag = "3")]
    pub backpressure: bool,
}
///*
/// The RandomSplit module randomly split/drop packets
///
/// __InputGates__: 1
/// __Output Gates__: many (configurable)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RandomSplitArg {
    //// Probability of dropping packet.
    #[prost(double, tag = "1")]
    pub drop_rate: f64,
    //// A list of gate numbers to split the traffic.
    #[prost(int64, repeated, tag = "2")]
    pub gates: ::std::vec::Vec<i64>,
}
///*
/// The RandomSplit module has a function `set_droprate(...)` which specifies
/// the probability of dropping packets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RandomSplitCommandSetDroprateArg {
    //// Probability of dropping packet.
    #[prost(double, tag = "1")]
    pub drop_rate: f64,
}
///*
/// The RandomSplit module has a function `set_gates(...)` which changes
/// the total number of output gates in the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RandomSplitCommandSetGatesArg {
    //// A list of gate numbers to split the traffic.
    #[prost(int64, repeated, tag = "1")]
    pub gates: ::std::vec::Vec<i64>,
}
///*
/// The RandomUpdate module rewrites a specified field (`offset` and `size`) in a packet
/// with a random value between a specified min and max values.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RandomUpdateArg {
    //// A list of Random Update Fields.
    #[prost(message, repeated, tag = "1")]
    pub fields: ::std::vec::Vec<random_update_arg::Field>,
}
pub mod random_update_arg {
    ///*
    /// RandomUpdate's Field specifies where to rewrite, and what values to rewrite
    /// in each packet processed.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Field {
        //// Offset in bytes for where to rewrite.
        #[prost(int64, tag = "1")]
        pub offset: i64,
        //// The number of bytes to write.
        #[prost(uint64, tag = "2")]
        pub size: u64,
        //// The minimum value to insert into the packet.
        #[prost(uint64, tag = "3")]
        pub min: u64,
        //// The maximum value to insert into the packet.
        #[prost(uint64, tag = "4")]
        pub max: u64,
    }
}
///*
/// The Rewrite module replaces an entire packet body with a packet "template"
/// converting all packets that pass through to copies of the of one of
/// the templates.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewriteArg {
    //// A list of bytestrings representing packet templates.
    #[prost(bytes, repeated, tag = "1")]
    pub templates: ::std::vec::Vec<std::vec::Vec<u8>>,
}
///*
/// The RoundRobin module has a function `set_gates(...)` which changes
/// the total number of output gates in the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundRobinCommandSetGatesArg {
    //// A list of gate numbers to round-robin the traffic over.
    #[prost(int64, repeated, tag = "1")]
    pub gates: ::std::vec::Vec<i64>,
}
///*
/// The RoundRobin module has a function `set_mode(...)` which specifies whether
/// to balance traffic across gates per-packet or per-batch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundRobinCommandSetModeArg {
    //// whether to perform 'packet' or 'batch' round robin partitioning.
    #[prost(string, tag = "1")]
    pub mode: std::string::String,
}
///*
/// The RoundRobin module splits packets from one input gate across multiple output
/// gates.
///
/// __Input Gates__: 1
/// __Output Gates__: many (configurable)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundRobinArg {
    //// A list of gate numbers to split packets across.
    #[prost(int64, repeated, tag = "1")]
    pub gates: ::std::vec::Vec<i64>,
    //// Whether to split across gate with every 'packet' or every 'batch'.
    #[prost(string, tag = "2")]
    pub mode: std::string::String,
}
///*
/// The Replicate module makes copies of a packet sending one copy out over each
/// of n output gates.
///
/// __Input Gates__: 1
/// __Output Gates__: many (configurable)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicateArg {
    //// A list of gate numbers to send packet copies to.
    #[prost(int64, repeated, tag = "1")]
    pub gates: ::std::vec::Vec<i64>,
}
///*
/// The Replicate module has a function `set_gates(...)` which changes
/// the total number of output gates in the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicateCommandSetGatesArg {
    //// A list of gate numbers to replicate the traffic over.
    #[prost(int64, repeated, tag = "1")]
    pub gates: ::std::vec::Vec<i64>,
}
///*
/// The SetMetadata module adds metadata attributes to packets, which are not stored
/// or sent out with packet data. For examples of SetMetadata use, see
/// [`bess/bessctl/conf/attr_match.bess`](https://github.com/NetSys/bess/blob/master/bessctl/conf/metadata/attr_match.bess)
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMetadataArg {
    //// A list of attributes to attach to the packet.
    #[prost(message, repeated, tag = "1")]
    pub attrs: ::std::vec::Vec<set_metadata_arg::Attribute>,
}
pub mod set_metadata_arg {
    ///*
    /// SetMetadata Attribute describes a metadata attribute and value to attach to every packet.
    /// If copying data from a packet buffer, SetMetadata can also logically shift
    /// then mask the value before storing it as metadata, i.e.,
    /// metadata_value = (packet_value >> `rshift_bits`) & `mask`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attribute {
        //// The metadata attribute name.
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        //// The size of values stored in this attribute in bytes.
        #[prost(uint64, tag = "2")]
        pub size: u64,
        //// An index in the packet data to store copy into the metadata attribute.
        #[prost(int32, tag = "5")]
        pub offset: i32,
        //// An array of bit masks to apply to each of the bytes copied starting from `offset`. If empty, the mask [0xFF,....,0xFF] will be used.
        #[prost(bytes, tag = "6")]
        pub mask: std::vec::Vec<u8>,
        //// The number of bits to shift the value at `offset` by before masking. Must be a multiple of 8. Positive and negative values represent right and left shifts respectively.
        #[prost(int32, tag = "7")]
        pub rshift_bits: i32,
        #[prost(oneof = "attribute::Value", tags = "3, 4")]
        pub value: ::std::option::Option<attribute::Value>,
    }
    pub mod attribute {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            //// An integer value to store in the packet (host-order).
            #[prost(uint64, tag = "3")]
            ValueInt(u64),
            //// A binary value to store in the packet (host-order).
            #[prost(bytes, tag = "4")]
            ValueBin(std::vec::Vec<u8>),
        }
    }
}
///*
/// The sink module drops all packets that are sent to it.
///
/// __Input Gates__: 1
/// __Output Gates__: 0
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SinkArg {}
///*
/// The Source module has a function `set_burst(...)` which
/// specifies the maximum number of packets to release in a single packetbatch
/// from the module.
///
/// __Input Gates__: 0
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCommandSetBurstArg {
    //// The maximum number of packets to release in a packetbatch from the module.
    #[prost(uint64, tag = "1")]
    pub burst: u64,
}
///*
/// The Source module has a function `set_pkt_size(...)` which specifies the size
/// of packets to be produced by the Source module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCommandSetPktSizeArg {
    //// The size (in bytes) of the packets for Source to create.
    #[prost(uint64, tag = "1")]
    pub pkt_size: u64,
}
///*
/// The Source module generates packets with no payload contents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceArg {
    //// The size (in bytes) of packet data to produce.
    #[prost(uint64, tag = "1")]
    pub pkt_size: u64,
}
///*
/// The Split module is a basic classifier which directs packets out a gate
/// based on data in the packet (e.g., if the read in value is 3, the packet
/// is directed out output gate 3).
///
/// __Input Gates__: 1
/// __Output Gates__: many (up to 2^(size * 8))
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitArg {
    //// The size of the value to read in bytes
    #[prost(uint64, tag = "1")]
    pub size: u64,
    #[prost(oneof = "split_arg::Type", tags = "2, 3")]
    pub r#type: ::std::option::Option<split_arg::Type>,
}
pub mod split_arg {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        //// The name of the metadata field to read.
        #[prost(string, tag = "2")]
        Attribute(std::string::String),
        //// The offset (in bytes) of the data field to read.
        #[prost(int64, tag = "3")]
        Offset(i64),
    }
}
///*
/// The timestamp module takes an offset parameter. It inserts the current
/// time in nanoseconds into the packet, to be used for latency measurements
/// alongside the Measure module.  The default offset is after an IPv4 UDP
/// header.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampArg {
    #[prost(uint64, tag = "1")]
    pub offset: u64,
}
///*
/// The Update module rewrites a field in a packet's data with a specific value.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArg {
    //// A list of Update Fields.
    #[prost(message, repeated, tag = "1")]
    pub fields: ::std::vec::Vec<update_arg::Field>,
}
pub mod update_arg {
    ///*
    /// Update Field describes where in a packet's data to rewrite, and with what value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Field {
        //// The offset in the packet in bytes to rewrite at.
        #[prost(int64, tag = "1")]
        pub offset: i64,
        //// The number of bytes to rewrite (max 8 bytes).
        #[prost(uint64, tag = "2")]
        pub size: u64,
        //// The value to write into the packet, max 8 bytes.
        #[prost(uint64, tag = "3")]
        pub value: u64,
    }
}
///*
/// The URLFilter performs TCP reconstruction over a flow and blocks
/// connections which mention a banned URL.
///
/// __Input Gates__: 2
/// __Output Gates__: 2
///
/// Note that the add() command takes this same argument, and the
/// clear() command takes an empty argument.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlFilterArg {
    //// A list of Urls to block.
    #[prost(message, repeated, tag = "1")]
    pub blacklist: ::std::vec::Vec<url_filter_arg::Url>,
}
pub mod url_filter_arg {
    ///*
    /// A URL consists of a host and a path.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Url {
        //// Host field, e.g. "www.google.com"
        #[prost(string, tag = "1")]
        pub host: std::string::String,
        //// Path prefix, e.g. "/"
        #[prost(string, tag = "2")]
        pub path: std::string::String,
    }
}
///*
/// The runtime configuration of a URLFilter is the current
/// blacklist.  This means that getting the Arg gets an *empty*
/// list: we assume anyone using get_initial_arg is also using
/// get_runtime_config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlFilterConfig {
    #[prost(message, repeated, tag = "1")]
    pub blacklist: ::std::vec::Vec<url_filter_arg::Url>,
}
///*
/// VLANPop removes the VLAN tag.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VlanPopArg {}
///*
/// VLANPush appends a VLAN tag with a specified TCI value.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VlanPushArg {
    //// The TCI value to insert in the VLAN tag.
    #[prost(uint64, tag = "1")]
    pub tci: u64,
}
///*
/// Splits packets across output gates according to VLAN id (e.g., id 3 goes out gate 3.
///
/// __Input Gates__: 1
/// __Output Gates__: many
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VlanSplitArg {}
///*
/// VXLANDecap module decapsulates a VXLAN header on a packet.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VxlanDecapArg {}
///*
/// VXLANEncap module wraps a packet in a VXLAN header with a specified destination port.
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VxlanEncapArg {
    //// The destination UDP port
    #[prost(uint64, tag = "1")]
    pub dstport: u64,
}
///*
/// The WildcardMatch module matches over multiple fields in a packet and
/// pushes packets that do match out specified gate, and those that don't out a default
/// gate. WildcardMatch is initialized wtih the fields it should inspect over,
/// rules are added via the `add(...)` function.
/// An example of WildcardMatch is in [`bess/bessctl/conf/samples/wildcardmatch.bess`](https://github.com/NetSys/bess/blob/master/bessctl/conf/samples/wildcardmatch.bess)
///
/// __Input Gates__: 1
/// __Output Gates__: many (configurable)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WildcardMatchArg {
    //// A list of WildcardMatch fields.
    #[prost(message, repeated, tag = "1")]
    pub fields: ::std::vec::Vec<Field>,
}
///*
/// WildcardMatchConfig represents the current runtime configuration
/// of a WildcardMatch module, as returned by get_runtime_config and
/// set by set_runtime_config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WildcardMatchConfig {
    #[prost(uint64, tag = "1")]
    pub default_gate: u64,
    #[prost(message, repeated, tag = "2")]
    pub rules: ::std::vec::Vec<WildcardMatchCommandAddArg>,
}
///*
/// The ARP Responder module is responding to ARP requests
/// TODO: Dynamic learn new MAC's-IP's mapping
///
/// __Input Gates__: 1
/// __Output Gates__: 1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArpResponderArg {
    ///*
    /// One ARP IP-MAC mapping
    ///
    /// The IP
    #[prost(string, tag = "1")]
    pub ip: std::string::String,
    //// The MAC address
    #[prost(string, tag = "2")]
    pub mac_addr: std::string::String,
}
///*
/// The MPLS pop module removes MPLS labels
///
/// __Input Gates__: 1
/// __Output Gates__: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MplsPopArg {
    /// Remove ETH header with the pop
    #[prost(bool, tag = "1")]
    pub remove_eth_header: bool,
    //// The next ETH type to set
    #[prost(uint32, tag = "2")]
    pub next_eth_type: u32,
}
///*
/// WorkerSplit splits packets based on the worker calling ProcessBatch(). It has
/// two modes.
/// 1) Packets from worker `x` are mapped to output gate `x`. This is the default
///    mode.
/// 2) When the `worker_gates` field is set, packets from a worker `x` are mapped
///    to `worker_gates[x]`.  In this mode, packet batches from workers not
///    mapped to an output gate will be dropped.
///
/// Calling the `reset` command with an empty `worker_gates` field will revert
/// WorkerSplit to the default mode.
///
/// __Input Gates__: 1
/// __Output Gates__: many
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerSplitArg {
    /// ogate -> worker mask
    #[prost(map = "uint32, uint32", tag = "1")]
    pub worker_gates: ::std::collections::HashMap<u32, u32>,
}
//// The Field message represents one field in a packet -- either stored in metadata or in the packet body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    //// The size of the data in bytes
    #[prost(uint32, tag = "3")]
    pub num_bytes: u32,
    #[prost(oneof = "field::Position", tags = "1, 2")]
    pub position: ::std::option::Option<field::Position>,
}
pub mod field {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Position {
        //// The metadata attribute assigned to store the data
        #[prost(string, tag = "1")]
        AttrName(std::string::String),
        //// The offset in bytes to store the data into
        #[prost(uint32, tag = "2")]
        Offset(u32),
    }
}
//// The FieldData message encodes a value to insert into a packet; the value can be supplied as either an int or a bytestring.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldData {
    #[prost(oneof = "field_data::Encoding", tags = "1, 2")]
    pub encoding: ::std::option::Option<field_data::Encoding>,
}
pub mod field_data {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Encoding {
        //// The value as a bytestring
        #[prost(bytes, tag = "1")]
        ValueBin(std::vec::Vec<u8>),
        //// The value in integer format
        #[prost(uint64, tag = "2")]
        ValueInt(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PcapPortArg {
    #[prost(string, tag = "1")]
    pub dev: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PmdPortArg {
    #[prost(bool, tag = "1")]
    pub loopback: bool,
    /// See http://dpdk.org/doc/dts/test_plans/dual_vlan_test_plan.html
    #[prost(bool, tag = "5")]
    pub vlan_offload_rx_strip: bool,
    #[prost(bool, tag = "6")]
    pub vlan_offload_rx_filter: bool,
    #[prost(bool, tag = "7")]
    pub vlan_offload_rx_qinq: bool,
    #[prost(oneof = "pmd_port_arg::Port", tags = "2, 3, 4")]
    pub port: ::std::option::Option<pmd_port_arg::Port>,
}
pub mod pmd_port_arg {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Port {
        #[prost(uint64, tag = "2")]
        PortId(u64),
        #[prost(string, tag = "3")]
        Pci(std::string::String),
        #[prost(string, tag = "4")]
        Vdev(std::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnixSocketPortArg {
    //// Set the first character to "@" in place of \0 for abstract path
    //// See manpage for unix(7).
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    //// Minimum RX polling interval for system calls, when *idle*.
    //// Use a negative number for unthrottled polling. If unspecified or 0,
    //// it is set to 50,000 (50 microseconds, or 20k polls per second)
    #[prost(int64, tag = "2")]
    pub min_rx_interval_ns: i64,
    //// If set, the port driver will send a confirmation once
    //// the port is connected.  This lets pybess avoid a race during
    //// testing.  See bessctl/test_utils.py for details.
    #[prost(bool, tag = "3")]
    pub confirm_connect: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZeroCopyVPortArg {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VPortArg {
    #[prost(string, tag = "1")]
    pub ifname: std::string::String,
    #[prost(int64, repeated, tag = "5")]
    pub rxq_cpus: ::std::vec::Vec<i64>,
    #[prost(uint64, tag = "6")]
    pub tx_tci: u64,
    #[prost(uint64, tag = "7")]
    pub tx_outer_tci: u64,
    #[prost(bool, tag = "8")]
    pub loopback: bool,
    #[prost(string, repeated, tag = "9")]
    pub ip_addrs: ::std::vec::Vec<std::string::String>,
    #[prost(oneof = "v_port_arg::Cpid", tags = "2, 3, 4")]
    pub cpid: ::std::option::Option<v_port_arg::Cpid>,
}
pub mod v_port_arg {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Cpid {
        #[prost(string, tag = "2")]
        Docker(std::string::String),
        #[prost(int64, tag = "3")]
        ContainerPid(i64),
        #[prost(string, tag = "4")]
        Netns(std::string::String),
    }
}
#[doc = r" Generated client implementations."]
pub mod bess_control_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct BessControlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BessControlClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BessControlClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = "/ Query version of bessd"]
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::VersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/GetVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Reset the current packet processing datapath to the initial state."]
        #[doc = "/"]
        #[doc = "/ This command is identical to the following sequence:"]
        #[doc = "/   ResetModules()"]
        #[doc = "/   ResetPorts()"]
        #[doc = "/   ResetTcs()"]
        #[doc = "/   ResetWorkers()"]
        #[doc = "/ As it clears everything, BESS should appear as if the daemon has freshly"]
        #[doc = "/ started (if not, it is a bug; please report)."]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn reset_all(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ResetAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Terminate the BESS daemon."]
        #[doc = "/"]
        #[doc = "/ BESS daemon shuts off in a graceful manner. Note that this command is"]
        #[doc = "/ \"asynchronous\": this command doesn't block until the BESS daemon has"]
        #[doc = "/ shut off."]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        #[doc = "/ FIXME: rename (e.g., Terminate)"]
        pub async fn kill_bess(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/KillBess");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Import a plugin"]
        #[doc = "/"]
        #[doc = "/ At the moment plugins can only contain module types,"]
        #[doc = "/ but might also support drivers/hooks/schedulers in the future."]
        pub async fn import_plugin(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportPluginRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ImportPlugin");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Unload a plugin"]
        #[doc = "/"]
        #[doc = "/ At the moment plugins can only contain module types,"]
        #[doc = "/ but might also support drivers/hooks/schedulers in the future."]
        pub async fn unload_plugin(
            &mut self,
            request: impl tonic::IntoRequest<super::UnloadPluginRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/UnloadPlugin");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ List imported plugins"]
        #[doc = "/"]
        #[doc = "/ At the moment plugins can only contain module types,"]
        #[doc = "/ but might also support drivers/hooks/schedulers in the future."]
        pub async fn list_plugins(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ListPluginsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ListPlugins");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Pause all running workers temporarily"]
        #[doc = "/"]
        #[doc = "/ Some RPC commands to BESS or individual modules/ports require that"]
        #[doc = "/ threads must be inactive, to avoid race conditions."]
        #[doc = "/ For such commands, use PauseALl at the beginning and ResumeAll at the end."]
        #[doc = "/  PauseAll()"]
        #[doc = "/   SomeCommand1()"]
        #[doc = "/   SomeCommand2()"]
        #[doc = "/   ..."]
        #[doc = "/  ResumeAll()"]
        #[doc = "/ Keep the duration as short as possible, to avoid packet drops."]
        pub async fn pause_all(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/PauseAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Pause the specified worker temporarily"]
        #[doc = "/"]
        #[doc = "/ Some RPC commands to BESS or individual modules/ports require that"]
        #[doc = "/ threads must be inactive, to avoid race conditions."]
        #[doc = "/ For such commands, use PauseWorker at the beginning and ResumeWorker at the end."]
        #[doc = "/  PauseWorker(0)"]
        #[doc = "/   SomeCommand1()"]
        #[doc = "/   SomeCommand2()"]
        #[doc = "/   ..."]
        #[doc = "/  ResumeWorker(0)"]
        #[doc = "/ Keep the duration as short as possible, to avoid packet drops."]
        pub async fn pause_worker(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseWorkerRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/PauseWorker");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Resume the specified worker"]
        pub async fn resume_worker(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeWorkerRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ResumeWorker");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Resume all paused workers"]
        pub async fn resume_all(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ResumeAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Remove all existing workers"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn reset_workers(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ResetWorkers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Enumerate all existing workers"]
        pub async fn list_workers(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ListWorkersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ListWorkers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Create a new worker"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn add_worker(
            &mut self,
            request: impl tonic::IntoRequest<super::AddWorkerRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/AddWorker");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Remove a single worker"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn destroy_worker(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyWorkerRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/DestroyWorker");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Remove all existing traffic classes"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn reset_tcs(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ResetTcs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Enumerate all existing workers"]
        pub async fn list_tcs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTcsRequest>,
        ) -> Result<tonic::Response<super::ListTcsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ListTcs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Check scheduling contraints"]
        pub async fn check_scheduling_constraints(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::CheckSchedulingConstraintsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bess.pb.BESSControl/CheckSchedulingConstraints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Create a new traffic class"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn add_tc(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTcRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/AddTc");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Update parameters of an existing traffic class"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn update_tc_params(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTcParamsRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/UpdateTcParams");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Change parent (and child arguments) of an existing traffic class"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn update_tc_parent(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTcParentRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/UpdateTcParent");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Collect statistics of a traffic class"]
        pub async fn get_tc_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTcStatsRequest>,
        ) -> Result<tonic::Response<super::GetTcStatsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/GetTcStats");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Enumerate all port drivers available"]
        pub async fn list_drivers(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ListDriversResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ListDrivers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Query detailed information of a port driver"]
        pub async fn get_driver_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDriverInfoRequest>,
        ) -> Result<tonic::Response<super::GetDriverInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/GetDriverInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Remove all initialized ports"]
        #[doc = "/"]
        #[doc = "/ Will fail if there are modules that are still using ports."]
        #[doc = "/ (e.g., PortInc, PortOut, QueueInc, QueueOut)"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn reset_ports(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ResetPorts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Enumerate all initialized ports"]
        pub async fn list_ports(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ListPortsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ListPorts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Create a new port from the specified driver"]
        pub async fn create_port(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePortRequest>,
        ) -> Result<tonic::Response<super::CreatePortResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/CreatePort");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Remove a port"]
        #[doc = "/"]
        #[doc = "/ The port should not be being used by a port-related module."]
        #[doc = "/ (e.g., PortInc, PortOut, QueueInc, QueueOut)"]
        pub async fn destroy_port(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyPortRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/DestroyPort");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Runtime-updatable configuration"]
        pub async fn set_port_conf(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPortConfRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/SetPortConf");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_port_conf(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPortConfRequest>,
        ) -> Result<tonic::Response<super::GetPortConfResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/GetPortConf");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Collect port statistics"]
        #[doc = "/"]
        #[doc = "/ At the moment, per-queue stats are not supported."]
        pub async fn get_port_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPortStatsRequest>,
        ) -> Result<tonic::Response<super::GetPortStatsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/GetPortStats");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Query link status"]
        pub async fn get_link_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLinkStatusRequest>,
        ) -> Result<tonic::Response<super::GetLinkStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/GetLinkStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Enumerate all module types available"]
        pub async fn list_mclass(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ListMclassResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ListMclass");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Query detailed information of a module type"]
        pub async fn get_mclass_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMclassInfoRequest>,
        ) -> Result<tonic::Response<super::GetMclassInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/GetMclassInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Remove all modules."]
        #[doc = "/"]
        #[doc = "/ This RPC will always succeed (unless there is a running worker)"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn reset_modules(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ResetModules");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Enumerate all initialized modules"]
        pub async fn list_modules(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ListModulesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ListModules");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Create a new module instance from the given module type"]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn create_module(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateModuleRequest>,
        ) -> Result<tonic::Response<super::CreateModuleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/CreateModule");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Destroy an exsting module"]
        #[doc = "/"]
        #[doc = "/ If the module is connected to other modules' input/output gate, they are"]
        #[doc = "/ disconnected first. All tasks created by the module will also be destoyed."]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn destroy_module(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyModuleRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/DestroyModule");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Fetch detailed information of an module instance"]
        pub async fn get_module_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModuleInfoRequest>,
        ) -> Result<tonic::Response<super::GetModuleInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/GetModuleInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Connect two modules."]
        #[doc = "/"]
        #[doc = "/ Connect between m1's ogate and n2's igate (i.e., ackets sent to m1's ogate"]
        #[doc = "/ will be fed to m2's igate). The oate can be connected to only one igate,"]
        #[doc = "/ while the igate can be connected to multiple output gates."]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn connect_modules(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectModulesRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ConnectModules");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Disconnect two modules."]
        #[doc = "/"]
        #[doc = "/ It removes a connection between two modules (you specify the previous one"]
        #[doc = "/ and its output gate). All packets coming out from the ogate will be"]
        #[doc = "/ dropped. Once disconnected, the ogate can be connected"]
        #[doc = "/ to any input gate."]
        #[doc = "/"]
        #[doc = "/ NOTE: There should be no running worker to run this command."]
        pub async fn disconnect_modules(
            &mut self,
            request: impl tonic::IntoRequest<super::DisconnectModulesRequest>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/DisconnectModules");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Dump various stats about BESS's packet pools"]
        pub async fn dump_mempool(
            &mut self,
            request: impl tonic::IntoRequest<super::DumpMempoolRequest>,
        ) -> Result<tonic::Response<super::DumpMempoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/DumpMempool");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Send a command to the specified module instance."]
        #[doc = "/"]
        #[doc = "/ Each module type defines a list of modyle-specific commands, which"]
        #[doc = "/ allow external programs to communicate with the module at runtime."]
        #[doc = "/ See module_msg.proto for details."]
        #[doc = "/"]
        #[doc = "/ NOTE: Some commands cannot be used if there are running workers."]
        #[doc = "/       For those commands you must pause all workers first."]
        pub async fn module_command(
            &mut self,
            request: impl tonic::IntoRequest<super::CommandRequest>,
        ) -> Result<tonic::Response<super::CommandResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ModuleCommand");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Enable/Disable a gate hook."]
        pub async fn configure_gate_hook(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureGateHookRequest>,
        ) -> Result<tonic::Response<super::CommandResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ConfigureGateHook");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Enable/Disable a gate hook."]
        pub async fn list_gate_hooks(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ListGateHooksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ListGateHooks");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Send command to gate hook instance."]
        pub async fn gate_hook_command(
            &mut self,
            request: impl tonic::IntoRequest<super::GateHookCommandRequest>,
        ) -> Result<tonic::Response<super::CommandResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/GateHookCommand");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ Enable/Disable a resume hook."]
        pub async fn configure_resume_hook(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureResumeHookRequest>,
        ) -> Result<tonic::Response<super::CommandResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/bess.pb.BESSControl/ConfigureResumeHook");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BessControlClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BessControlClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BessControlClient {{ ... }}")
        }
    }
}
