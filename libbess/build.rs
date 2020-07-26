fn main() {
    tonic_build::configure()
        .out_dir("./src")
        .build_server(false)
        .format(true)
        .compile(&["./bess.proto"], &["."]).unwrap();
}
