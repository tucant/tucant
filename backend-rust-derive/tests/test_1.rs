use tucant_derive::Typescriptable;

#[derive(Typescriptable)]
struct Test { inner: i32 }


#[derive(Typescriptable)]
struct TestT<T> { inner: T }
