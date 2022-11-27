use std::collections::BTreeSet;

use axum::{Router, routing::{MethodRouter, get}};
use tucant_derive_lib::{Typescriptable, TypescriptRoute};

pub struct TypescriptableApp<S: Clone + Send + Sync + 'static = ()> {
    pub app: Router<S>,
    pub codes: BTreeSet<String>,
}

impl<S: Clone + Send + Sync> TypescriptableApp<S>
{
    pub fn route<TR: TypescriptRoute>(mut self, path: &str, method_router: MethodRouter<S>) -> Self
    {
        self.codes.extend(TR::code(path));
        self.app = self.app.route(path, method_router);
        self
    }
}
