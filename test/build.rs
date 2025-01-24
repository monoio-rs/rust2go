fn main() {
    monoio_rust2go::Builder::new()
        .with_go_src("./go")
        // .with_regen("./src/user.rs", "./go/gen.go")
        .build();
}
