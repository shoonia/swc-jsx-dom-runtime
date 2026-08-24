use swc_core::ecma::ast::Program;
use swc_core::ecma::visit::visit_mut_pass;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub mod jsx_transformer;

use crate::jsx_transformer::JsxTransformer;

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(visit_mut_pass(JsxTransformer))
}
