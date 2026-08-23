use swc_core::ecma::visit::VisitMutWith;
use swc_core::ecma::{ast::Program, transforms::testing::test_inline, visit::VisitMut};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
}

#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    _metadata: TransformPluginProgramMetadata,
) -> Program {
    program.visit_mut_with(&mut TransformVisitor);
    program
}

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor),
    boo,
    // Input codes
    r#"console.log("transform");"#,
    // Output codes after transformed with plugin
    r#"console.log("transform");"#
);
