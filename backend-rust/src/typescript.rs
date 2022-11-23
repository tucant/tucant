use std::collections::BTreeSet;

use actix_web::{
    dev::{HttpServiceFactory, ServiceFactory, ServiceRequest},
    Error,
};
use tucant_derive_lib::Typescriptable;

pub struct TypescriptableApp<T> {
    pub app: actix_web::App<T>,
    pub codes: BTreeSet<String>,
}

impl<T> TypescriptableApp<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: Typescriptable + HttpServiceFactory + 'static,
    {
        self.codes.extend(<F as Typescriptable>::code());
        self.app = self.app.service(factory);
        self
    }
}
