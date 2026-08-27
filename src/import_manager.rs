use std::collections::HashMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportName {
    Jsx,
    SvgNs,
    MathmlNs,
    // SetStyle,
    // SetDataset,
    // SetSignalish,
    // SetAttributes,
}

impl ImportName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Jsx => "jsx",
            Self::SvgNs => "svgNs",
            Self::MathmlNs => "mathmlNs",
            // Self::SetStyle => "setStyle",
            // Self::SetDataset => "setDataset",
            // Self::SetSignalish => "setSignalish",
            // Self::SetAttributes => "setAttributes",
        }
    }
}

pub struct ImportManager {
    cache: HashMap<ImportName, Expr>,
    specifiers: Vec<ImportSpecifier>,
}

impl ImportManager {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            specifiers: Vec::new(),
        }
    }

    pub fn add(&mut self, import_name: ImportName) -> Expr {
        if let Some(ident) = self.cache.get(&import_name) {
            return ident.clone();
        }

        let local_name: String = format!("_{}", import_name.as_str());
        let local_ident = Ident::from(local_name);
        let expr_ident = Expr::Ident(local_ident.clone());

        self.cache.insert(import_name, expr_ident.clone());
        self.specifiers
            .push(ImportSpecifier::Named(ImportNamedSpecifier {
                span: DUMMY_SP,
                local: local_ident,
                imported: Some(ModuleExportName::Ident(Ident::from(import_name.as_str()))),
                is_type_only: false,
            }));

        expr_ident
    }

    pub fn inject_into_module(&self, module: &mut Module) {
        if self.specifiers.is_empty() {
            return;
        }

        module.body.insert(
            0,
            ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                span: DUMMY_SP,
                specifiers: self.specifiers.clone(),
                src: Box::new(Str::from("jsx-dom-runtime")),
                type_only: false,
                with: None,
                phase: Default::default(),
            })),
        );
    }
}
