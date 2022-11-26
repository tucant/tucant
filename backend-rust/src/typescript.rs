use std::collections::BTreeSet;

use axum::Router;
use tucant_derive_lib::Typescriptable;

pub struct TypescriptableApp {
    pub app: Router,
    pub codes: BTreeSet<String>,
}

impl TypescriptableApp
{
    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: Typescriptable + 'static,
    {
        self.codes.extend(<F as Typescriptable>::code());
        self.app = self.app.service(factory);
        self
    }
}
