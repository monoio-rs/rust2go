// Include the binding file. There are 2 ways to include it:
// 1. Use monoio_rust2go's macro:
// ```rust
// pub mod binding {
//     monoio_rust2go::r2g_include_binding!();
// }
// ```
// 2. Include it manually:
// ```rust
// pub mod binding {
//     include!(concat!(env!("OUT_DIR"), "/_go_bindings.rs"));
// }
// ```
// If you want to use your own binding file name, use:
// `monoio_rust2go::r2g_include_binding!("_go_bindings.rs");`
pub mod binding {
    #![allow(warnings)]
    monoio_rust2go::r2g_include_binding!();
}

// Define your own structs. You must derive `monoio_rust2go::R2G` for each struct.
#[derive(monoio_rust2go::R2G, Clone)]
pub struct DemoUser {
    pub name: String,
    pub age: u8,
}

// Define your own structs. You must derive `monoio_rust2go::R2G` for each struct.
#[derive(monoio_rust2go::R2G, Clone)]
pub struct DemoComplicatedRequest {
    pub users: Vec<DemoUser>,
    pub balabala: Vec<u8>,
}

// Define your own structs. You must derive `monoio_rust2go::R2G` for each struct.
#[derive(monoio_rust2go::R2G, Clone, Copy)]
pub struct DemoResponse {
    pub pass: bool,
}

// Define the call trait.
// It can be defined in 2 styles: sync and async.
// If the golang side is purely calculation logic, and not very heavy, use sync can be more efficient.
// Otherwise, use async style:
// Both `async fn` and `impl Future` styles are supported.
//
// If you want to use your own binding mod name, use:
// `#[monoio_rust2go::r2g(binding)]`
#[monoio_rust2go::r2g]
pub trait DemoCall {
    fn demo_oneway(req: &DemoUser);
    fn demo_check(req: &DemoComplicatedRequest) -> DemoResponse;
    fn demo_check_async(
        req: &DemoComplicatedRequest,
    ) -> impl std::future::Future<Output = DemoResponse>;
    #[drop_safe]
    fn demo_check_async_safe(
        req: DemoComplicatedRequest,
    ) -> impl std::future::Future<Output = DemoResponse>;
}
