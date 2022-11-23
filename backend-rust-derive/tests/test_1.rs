use tucant_derive::Typescriptable;

// cargo expand --test test_1

#[derive(Typescriptable)]
struct TestT1 {
    inner: i32,
}

#[derive(Typescriptable)]
struct TestT2<T: tucant_derive_lib::Typescriptable> {
    inner: T,
}
