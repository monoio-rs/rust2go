---
title: Use attribute to control code generation
date: 2023-12-22 15:12:00
author: ihciah
---

Now monoio-rust2go supports 3 attributes on trait's async function:

1. `#[send]`: the function will be generated as `impl Future<Output=..> + Send + Sync`.
2. `#[drop_safe]`: this makes the function safe, but requires all paramters passing ownership.
3. `#[drop_safe_ret]`: to make the function safe, it requires passing ownership; this attribute allow users to get the paramters ownership back.
4. `#[go_ptr]`: make the generated go side code use pointer instead of value at parameters. This is useful when the parameter is large. This does not affect the rust side code.

For example, here is the original trait:

```rust
#[monoio_rust2go::r2g]
pub trait DemoCall {
    #[send]
    fn demo_check_async(
        req: &DemoComplicatedRequest,
    ) -> impl std::future::Future<Output = DemoResponse>;
    #[drop_safe]
    fn demo_check_async_safe(
        req: DemoComplicatedRequest,
    ) -> impl std::future::Future<Output = DemoResponse>;
    #[drop_safe_ret]
    fn demo_check_async_safe_with_ret(
        req: DemoComplicatedRequest,
    ) -> impl std::future::Future<Output = DemoResponse>;
}
```

Here is the generated trait:

```rust
pub trait DemoCall {
    unsafe fn demo_check_async(
        req: &DemoComplicatedRequest,
    ) -> impl ::std::future::Future<Output = DemoResponse> + Send + Sync;

    fn demo_check_async_safe(
        req: DemoComplicatedRequest,
    ) -> impl ::std::future::Future<Output = DemoResponse> + 'static;

    fn demo_check_async_safe_with_ret(
        req: DemoComplicatedRequest,
    ) -> impl ::std::future::Future<Output = (DemoResponse, (DemoComplicatedRequest,))> + 'static;
}
```

Note, if all parameters are with ownership, the generated impl Future will be added with a `'static` lifetime automatically. This is useful for spawning tasks.
