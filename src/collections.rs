use std::matches;
use swc_core::ecma::ast::Ident;

#[inline(always)]
pub fn is_fn_component(ident: &Ident) -> bool {
    matches!(
        ident.sym.as_bytes().first(),
        Some(b'A'..=b'Z' | b'_' | b'$')
    )
}

#[inline(always)]
pub fn is_svg_tag(tag: &str) -> bool {
    matches!(
        tag,
        /* "a" */
        "altGlyph"
        | "altGlyphDef"
        | "altGlyphItem"
        | "animate"
        | "animateColor"
        | "animateMotion"
        | "animateTransform"
        | "circle"
        | "clipPath"
        | "color-profile"
        | "cursor"
        | "defs"
        | "desc"
        | "ellipse"
        | "feBlend"
        | "feColorMatrix"
        | "feComponentTransfer"
        | "feComposite"
        | "feConvolveMatrix"
        | "feDiffuseLighting"
        | "feDisplacementMap"
        | "feDistantLight"
        | "feFlood"
        | "feFuncA"
        | "feFuncB"
        | "feFuncG"
        | "feFuncR"
        | "feGaussianBlur"
        | "feImage"
        | "feMerge"
        | "feMergeNode"
        | "feMorphology"
        | "feOffset"
        | "fePointLight"
        | "feSpecularLighting"
        | "feSpotLight"
        | "feTile"
        | "feTurbulence"
        | "filter"
        | "font"
        | "font-face"
        | "font-face-format"
        | "font-face-name"
        | "font-face-src"
        | "font-face-uri"
        | "foreignObject"
        | "g"
        | "glyph"
        | "glyphRef"
        | "hkern"
        | "image"
        | "line"
        | "linearGradient"
        | "marker"
        | "mask"
        | "metadata"
        | "missing-glyph"
        | "mpath"
        | "path"
        | "pattern"
        | "polygon"
        | "polyline"
        | "radialGradient"
        | "rect"
        /* "script"| */
        | "set"
        | "stop"
        /* "style"| */ 
        | "svg"
        | "switch"
        | "symbol"
        | "text"
        | "textPath"
        /* "title"| */ 
        | "tref"
        | "tspan"
        | "use"
        | "view"
        | "vkern"
        // non-standard
        | "discard"
        | "mesh"
        | "meshgradient"
        | "meshpatch"
        | "meshrow"
        | "solidcolor"
    )
}

#[inline(always)]
pub fn is_mathml_tag(tag: &str) -> bool {
    matches!(
        tag,
        "annotation"
        | "annotation-xml"
        | "maction"
        | "math"
        | "merror"
        | "mfrac"
        | "mi"
        | "mmultiscripts"
        | "mn"
        | "mo"
        | "mover"
        | "mpadded"
        | "mphantom"
        | "mprescripts"
        | "mroot"
        | "mrow"
        | "ms"
        | "mspace"
        | "msqrt"
        | "mstyle"
        | "msub"
        | "msubsup"
        | "msup"
        | "mtable"
        | "mtd"
        | "mtext"
        | "mtr"
        | "munder"
        | "munderover"
        | "semantics"
        | "mfenced"
        // non-standard
        | "menclose"
        | "mlabeledtr"
        | "maligngroup"
        | "malignmark"
    )
}
