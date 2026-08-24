use std::path::PathBuf;

use swc_core::ecma::{
    parser::{EsSyntax, Syntax},
    transforms::testing::{test_fixture, FixtureTestConfig},
    visit::visit_mut_pass,
};
use swc_jsx_dom_runtime::jsx_transformer::JsxTransformer;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/**/input.js")]
fn jsx_transformer_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        syntax(),
        &|_| visit_mut_pass(JsxTransformer::new()),
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            module: Some(true),
            ..Default::default()
        },
    );
}
