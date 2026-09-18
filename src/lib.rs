use swc_core::{
    ecma::{ast::Program, visit::visit_mut_pass},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

mod builders;
mod collections;
mod consts;
mod import_manager;
mod jsx_text_to_str;
pub mod jsx_transformer;
mod utils;

use crate::jsx_transformer::JsxTransformer;

#[plugin_transform]
pub fn process_transform(program: Program, meta: TransformPluginProgramMetadata) -> Program {
    program.apply(visit_mut_pass(JsxTransformer::new(meta.comments)))
}
