use std::array;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

#[derive(Clone, Copy)]
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
    const COUNT: usize = 7;

    const fn index(self) -> usize {
        match self {
            Self::Jsx => 0,
            Self::SvgNs => 1,
            Self::MathmlNs => 2,
            Self::SetStyle => 3,
            Self::SetDataset => 4,
            Self::SetSignalish => 5,
            Self::SetAttributes => 6,
        }
    }

    pub fn as_str(self) -> &'static str {
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
    cache: [Option<Ident>; ImportName::COUNT],
    specifiers: Vec<ImportSpecifier>,
}

impl ImportManager {
    pub fn new() -> Self {
        Self {
            cache: array::from_fn(|_| None),
            specifiers: Vec::new(),
        }
    }

    pub fn add(&mut self, import_name: ImportName) -> Expr {
        let index = import_name.index();

        if let Some(ident) = &self.cache[index] {
            return Expr::Ident(ident.clone());
        }

        let local_ident = Ident::from(format!("_{}", import_name.as_str()));

        self.cache[index] = Some(local_ident.clone());
        self.specifiers
            .push(ImportSpecifier::Named(ImportNamedSpecifier {
                span: DUMMY_SP,
                local: local_ident.clone(),
                imported: Some(ModuleExportName::Ident(Ident::from(import_name.as_str()))),
                is_type_only: false,
            }));

        Expr::Ident(local_ident)
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
                phase: ImportPhase::Evaluation,
            })),
        );
    }
}
