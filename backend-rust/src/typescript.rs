use std::collections::BTreeSet;

use axum::{Router, routing::{MethodRouter, get}};
use tucant_derive_lib::{Typescriptable, TypescriptRoute};

pub struct TypescriptableApp {
    pub app: Router,
    pub codes: BTreeSet<String>,
}

impl TypescriptableApp
{
    pub fn service<TR: TypescriptRoute>(mut self, path: &str, method_router: MethodRouter) -> Self
    {
        self.codes.extend(TR::code(path));
        self.app = self.app.route(path, method_router);
        self
    }
}
