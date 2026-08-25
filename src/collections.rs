use std::matches;
use swc_core::ecma::ast::Ident;
use std::collections::HashMap;
use std::sync::LazyLock;

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

pub static HTML_DOM_ATTRIBUTES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("acceptCharset", "accept-charset"),
        ("className", "class"),
        ("httpEquiv", "http-equiv"),
        ("htmlFor", "for"),
        // SVG 2 removed the need for the `xlink` namespace, so instead of `xlink:href` you should use `href`
        ("xlinkHref", "href"),
    ])
});

pub static SVG_DOM_ATTRIBUTES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("className", "class"),
        ("accentHeight", "accent-height"),
        ("alignmentBaseline", "alignment-baseline"),
        ("arabicForm", "arabic-form"),
        ("baselineShift", "baseline-shift"),
        ("capHeight", "cap-height"),
        ("clipPath", "clip-path"),
        ("clipRule", "clip-rule"),
        ("colorInterpolation", "color-interpolation"),
        ("colorInterpolationFilters", "color-interpolation-filters"),
        ("colorProfile", "color-profile"),
        ("colorRendering", "color-rendering"),
        ("dominantBaseline", "dominant-baseline"),
        ("enableBackground", "enable-background"),
        ("fillOpacity", "fill-opacity"),
        ("fillRule", "fill-rule"),
        ("floodColor", "flood-color"),
        ("floodOpacity", "flood-opacity"),
        ("fontFamily", "font-family"),
        ("fontSize", "font-size"),
        ("fontSizeAdjust", "font-size-adjust"),
        ("fontStretch", "font-stretch"),
        ("fontStyle", "font-style"),
        ("fontVariant", "font-variant"),
        ("fontWeight", "font-weight"),
        ("glyphName", "glyph-name"),
        ("glyphOrientationHorizontal", "glyph-orientation-horizontal"),
        ("glyphOrientationVertical", "glyph-orientation-vertical"),
        ("horizAdvX", "horiz-adv-x"),
        ("horizOriginX", "horiz-origin-x"),
        ("imageRendering", "image-rendering"),
        ("letterSpacing", "letter-spacing"),
        ("lightingColor", "lighting-color"),
        ("markerEnd", "marker-end"),
        ("markerMid", "marker-mid"),
        ("markerStart", "marker-start"),
        ("overlinePosition", "overline-position"),
        ("overlineThickness", "overline-thickness"),
        ("paintOrder", "paint-order"),
        ("panose1", "panose-1"),
        ("pointerEvents", "pointer-events"),
        ("renderingIntent", "rendering-intent"),
        ("shapeRendering", "shape-rendering"),
        ("stopColor", "stop-color"),
        ("stopOpacity", "stop-opacity"),
        ("strikethroughPosition", "strikethrough-position"),
        ("strikethroughThickness", "strikethrough-thickness"),
        ("strokeDasharray", "stroke-dasharray"),
        ("strokeDashoffset", "stroke-dashoffset"),
        ("strokeLinecap", "stroke-linecap"),
        ("strokeLinejoin", "stroke-linejoin"),
        ("strokeMiterlimit", "stroke-miterlimit"),
        ("strokeOpacity", "stroke-opacity"),
        ("strokeWidth", "stroke-width"),
        ("textAnchor", "text-anchor"),
        ("textDecoration", "text-decoration"),
        ("textRendering", "text-rendering"),
        ("underlinePosition", "underline-position"),
        ("underlineThickness", "underline-thickness"),
        ("unicodeBidi", "unicode-bidi"),
        ("unicodeRange", "unicode-range"),
        ("unitsPerEm", "units-per-em"),
        ("vAlphabetic", "v-alphabetic"),
        ("vHanging", "v-hanging"),
        ("vIdeographic", "v-ideographic"),
        ("vMathematical", "v-mathematical"),
        ("vectorEffect", "vector-effect"),
        ("vertAdvY", "vert-adv-y"),
        ("vertOriginX", "vert-origin-x"),
        ("vertOriginY", "vert-origin-y"),
        ("wordSpacing", "word-spacing"),
        ("writingMode", "writing-mode"),
        ("xHeight", "x-height"),
        ("xlinkActuate", "xlink:actuate"),
        ("xlinkArcrole", "xlink:arcrole"),
        // SVG 2 removed the need for the `xlink` namespace, so instead of `xlink:href` you should use `href`
        ("xlinkHref", "href"),
        ("xlinkRole", "xlink:role"),
        ("xlinkShow", "xlink:show"),
        ("xlinkTitle", "xlink:title"),
        ("xlinkType", "xlink:type"),
        ("xmlBase", "xml:base"),
        ("xmlLang", "xml:lang"),
        ("xmlSpace", "xml:space"),
    ])
});
