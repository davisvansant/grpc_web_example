fn main() {
    tonic_build::configure()
        .build_client(false)
        .out_dir("../proto/output")
        .compile(&["../proto/hello/v1/hello.proto"], &["../proto"])
        .unwrap();
}
