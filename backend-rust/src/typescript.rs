use actix_web::{dev::{HttpServiceFactory, ServiceFactory, ServiceRequest}, Error};

pub trait Typescriptable {
    fn name() -> String;
    fn code() -> String { "".to_string() }
}

impl Typescriptable for u32 {
    fn name() -> String {
        "number".to_string()
    }
}


impl Typescriptable for String {
    fn name() -> String {
        "string".to_string()
    }
}

pub struct TypescriptableApp<T> {
    pub app: actix_web::App<T>,
}

impl<T> TypescriptableApp<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: Typescriptable + HttpServiceFactory + 'static,
    {
        println!("{}", <F as Typescriptable>::code());
        self.app = self.app.service(factory);
        self
    }
}
