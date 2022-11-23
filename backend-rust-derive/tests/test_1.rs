use tucant_derive::Typescriptable;

// cargo expand --test test_1

#[derive(Typescriptable)]
struct TestT1 {
    #[allow(dead_code)]
    inner: i32,
}

#[derive(Typescriptable)]
struct TestT2<T: tucant_derive_lib::Typescriptable> {
    #[allow(dead_code)]
    inner: T,
}
