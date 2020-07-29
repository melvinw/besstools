fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .include("proto")
        .include("/usr/local/include")
        .inputs(&[
            "proto/bess_msg.proto",
            "proto/module_msg.proto",
            "proto/service.proto",
            "proto/util_msg.proto",
            "proto/error.proto",
            "proto/ports/port_msg.proto",
        ])
        .rust_protobuf(true)
        .rust_protobuf_customize(protoc_rust::Customize {
            serde_derive: Some(true),
            ..Default::default()
        })
        .run()
        .expect("protoc-rust-grpc");
}
