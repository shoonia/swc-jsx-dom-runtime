use std::collections::HashMap;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportName {
    Jsx,
    SvgNs,
    MathmlNs,
    SetStyle,
    SetDataset,
    SetSignalish,
    SetAttributes,
}

impl ImportName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Jsx => "jsx",
            Self::SvgNs => "svgNs",
            Self::MathmlNs => "mathmlNs",
            Self::SetStyle => "setStyle",
            Self::SetDataset => "setDataset",
            Self::SetSignalish => "setSignalish",
            Self::SetAttributes => "setAttributes",
        }
    }
}

pub struct ImportManager {
    cache: HashMap<ImportName, Ident>,
    specifiers: Vec<ImportSpecifier>,
}

impl ImportManager {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            specifiers: Vec::new(),
        }
    }

    pub fn add(&mut self, import_name: ImportName) -> Ident {
        if let Some(ident) = self.cache.get(&import_name) {
            return ident.clone();
        }

        let local_name: String = format!("_{}", import_name.as_str());
        let local_ident = Ident::new_no_ctxt(local_name.into(), DUMMY_SP);

        self.cache.insert(import_name, local_ident.clone());
        self.specifiers
            .push(ImportSpecifier::Named(ImportNamedSpecifier {
                span: DUMMY_SP,
                local: local_ident.clone(),
                imported: Some(ModuleExportName::Ident(Ident::new_no_ctxt(
                    import_name.as_str().into(),
                    DUMMY_SP,
                ))),
                is_type_only: false,
            }));

        local_ident
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
                src: Box::new(Str {
                    span: DUMMY_SP,
                    value: "jsx-dom-runtime".into(),
                    raw: None,
                }),
                type_only: false,
                with: None,
                phase: Default::default(),
            })),
        );
    }
}
