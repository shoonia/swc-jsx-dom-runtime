use std::collections::HashMap;
use std::matches;
use std::sync::LazyLock;
use swc_core::ecma::ast::Ident;

#[inline(always)]
pub fn is_fn_component(ident: &Ident) -> bool {
    matches!(
        ident.sym.as_bytes().first(),
        Some(b'A'..=b'Z' | b'_' | b'$')
    )
}

pub fn is_html_tag(tag: &str) -> bool {
    matches!(
        tag,
        // Valid HTML tags
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input"
        | "link" | "meta" | "param" | "source" | "track" | "wbr"
        // Non-void HTML tags
        | "a" | "abbr" | "address" | "article" | "aside" | "audio"
        | "b" | "bdi" | "bdo" | "blockquote" | "body" | "button"
        | "canvas" | "caption" | "cite" | "code" | "colgroup"
        | "data" | "datalist" | "dd" | "del" | "details" | "dfn" | "dialog" | "div" | "dl" | "dt"
        | "em"
        | "fieldset" | "figcaption" | "figure" | "footer" | "form"
        | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "head" | "header" | "hgroup" | "html"
        | "i" | "iframe" | "ins"
        | "kbd"
        | "label" | "legend" | "li"
        | "main" | "map" | "mark" | "menu" | "menuitem" | "meter"
        | "nav" | "noscript"
        | "object" | "ol" | "optgroup" | "option" | "output"
        | "p" | "picture" | "pre" | "progress"
        | "q"
        | "rb" | "rp" | "rt" | "rtc" | "ruby"
        | "s" | "samp" | "script" | "search" | "section" | "select" | "slot" | "small"
        | "span" | "strong" | "style" | "sub" | "summary" | "sup"
        | "table" | "tbody" | "td" | "template" | "textarea" | "tfoot" | "th" | "thead" | "time"
        | "title" | "tr"
        | "u" | "ul"
        | "var" | "video"
        // non-standard
        | "fencedframe" | "selectedcontent" | "geolocation"
        // deprecated
        | "acronym" | "applet" | "basefont" | "bgsound" | "big" | "blink" | "center" | "noframes"
        | "tt" | "strike" | "xmp" | "isindex" | "keygen"
    )
}

#[inline(always)]
pub fn is_svg_tag(tag: &str) -> bool {
    matches!(
        tag,
        /* "a" */
        "altGlyph" | "altGlyphDef" | "altGlyphItem" | "animate" | "animateColor" | "animateMotion"
        | "animateTransform" | "circle" | "clipPath" | "color-profile" | "cursor" | "defs"
        | "desc" | "ellipse" | "feBlend" | "feColorMatrix" | "feComponentTransfer" | "feComposite"
        | "feConvolveMatrix" | "feDiffuseLighting" | "feDisplacementMap" | "feDistantLight" 
        | "feFlood" | "feFuncA" | "feFuncB" | "feFuncG" | "feFuncR" | "feGaussianBlur"
        | "feImage" | "feMerge" | "feMergeNode" | "feMorphology" | "feOffset" | "fePointLight"
        | "feSpecularLighting" | "feSpotLight" | "feTile" | "feTurbulence" | "filter" | "font"
        | "font-face" | "font-face-format" | "font-face-name" | "font-face-src" | "font-face-uri" 
        | "foreignObject" | "g" | "glyph" | "glyphRef" | "hkern" | "image"  | "line" 
        | "linearGradient" | "marker" | "mask" | "metadata" | "missing-glyph" | "mpath" | "path"
        | "pattern" | "polygon" | "polyline" | "radialGradient" | "rect" /* "script"| */
        | "set" | "stop" /* "style"| */  | "svg" | "switch" | "symbol" | "text" | "textPath"
        /* "title"| */  | "tref" | "tspan" | "use" | "view" | "vkern"
        // non-standard
        | "discard" | "mesh" | "meshgradient" | "meshpatch" | "meshrow" | "solidcolor"
    )
}

#[inline(always)]
pub fn is_mathml_tag(tag: &str) -> bool {
    matches!(
        tag,
        "annotation" | "annotation-xml" | "maction" | "math" | "merror" | "mfrac" | "mi"
        | "mmultiscripts" | "mn" | "mo" | "mover" | "mpadded" | "mphantom" | "mprescripts"
        | "mroot" | "mrow" | "ms" | "mspace" | "msqrt" | "mstyle" | "msub" | "msubsup"
        | "msup" | "mtable" | "mtd" | "mtext" | "mtr" | "munder" | "munderover"
        | "semantics" | "mfenced"
        // non-standard
        | "menclose" | "mlabeledtr" | "maligngroup" | "malignmark"
    )
}

#[inline(always)]
pub fn is_bool_attr(attr: &str) -> bool {
    matches!(
        attr,
        "async"
            | "autofocus"
            | "autocomplete"
            | "autoplay"
            | "attributionsrc"
            | "controls"
            | "checked"
            | "crossorigin"
            | "capture"
            | "defer"
            | "disabled"
            | "contenteditable"
            | "formnovalidate"
            | "readonly"
            | "multiple"
            | "loop"
            | "required"
            | "hidden"
            | "open"
            | "selected"
            | "nomodule"
            | "noshade"
            | "novalidate"
            | "playsinline"
            | "reversed"
            | "inert"
            | "disablepictureinpicture"
            | "disableremoteplayback"
            | "popover"
            | "itemscope"
            | "declare"
            | "moz-opaque"
            | "ismap"
            | "shadowrootclonable"
            | "shadowrootdelegatesfocus"
            | "shadowrootserializable"
            | "webkitdirectory"
    )
}

#[inline(always)]
pub fn is_enumerated_attr(attr: &str) -> bool {
    matches!(attr, "draggable" | "spellcheck" | "writingsuggestions")
        || attr.starts_with("aria-")
        || attr.starts_with("data-")
}

#[inline(always)]
pub fn event_types(name: &str) -> bool {
    matches!(
        name,
        // ClipboardEvent
        "copy" | "cut" | "paste" |
        // CompositionEvent
        "compositionend" | "compositionstart" | "compositionupdate" |
        // [Form] Event
        "change" | "reset" | "invalid" |
        // Event
        "load" | "error" | "select" | "selectionchange" | "beforematch" |
        // FocusEvent
        "focus" | "blur" | "focusin" | "focusout" |
        // InputEvent
        "beforeinput" | "input" |
        // SubmitEvent
        "submit" |
        // FormDataEvent
        "formdata" |
        // KeyboardEvent
        "keydown" | "keypress" | "keyup" |
        // [Media] Event
        "abort" | "canplay" | "canplaythrough" | "durationchange" |
        "emptied" | "ended" | "loadeddata" | "loadedmetadata" |
        "loadstart" | "pause" | "play" | "playing" | "progress" |
        "ratechange" | "seeked" | "seeking" | "stalled" | "suspend" |
        "timeupdate" | "volumechange" | "waiting" | "waitingforkey" |
        // MediaEncryptedEvent
        "encrypted" |
        // MouseEvents
        "auxclick" | "click" | "contextmenu" | "dblclick" |
        "mousedown" | "mouseenter" | "mouseleave" |
        "mousemove" | "mouseout" | "mouseover" | "mouseup" |
        // DragEvent
        "drag" | "dragend" | "dragenter" | "dragleave" | "dragover" |
        "dragstart" | "drop" | "dragexit" |
        // TouchEvent
        "touchcancel" | "touchend" | "touchmove" | "touchstart" |
        // PointerEvent
        "pointerdown" | "pointermove" | "pointerup" | "pointercancel" |
        "pointerenter" | "pointerleave" | "pointerover" | "pointerout" |
        "gotpointercapture" | "lostpointercapture" |
        // UIEvent
        "scroll" | "scrollend" |
        // SnapEvent
        "scrollsnapchange" | "scrollsnapchanging" |
        // WheelEvent
        "wheel" |
        // AnimationEvent
        "animationstart" | "animationend" | "animationiteration" | "animationcancel" |
        // TransitionEvent
        "transitionend" | "transitionstart" | "transitioncancel" | "transitionrun" |
        // PictureInPicture Events
        "enterpictureinpicture" | "leavepictureinpicture" | "resize" |
        // ToggleEvent
        "beforetoggle" | "toggle" |
        // HTMLDialogElement
        "cancel" | "close" |
        // Fullscreen API
        "fullscreenchange" | "fullscreenerror" |
        // HTMLTrackElement
        "cuechange" |
        // ContentVisibilityAutoStateChangeEvent
        "contentvisibilityautostatechange" |
        // CommandEvent
        "command" |
        // HTMLCanvasElement
        "contextlost" | "contextrestored" |
        // WebGLContextEvent
        "webglcontextlost" | "webglcontextrestored" | "webglcontextcreationerror"
    )
}

pub static HTML_DOM_ATTRIBUTES: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from([
            ("acceptCharset", "accept-charset"),
            ("className", "class"),
            ("httpEquiv", "http-equiv"),
            ("htmlFor", "for"),
            // SVG 2 removed the need for the `xlink` namespace, so instead of `xlink:href` you should use `href`
            ("xlinkHref", "href"),
        ])
    });

pub static SVG_DOM_ATTRIBUTES: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from([
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
