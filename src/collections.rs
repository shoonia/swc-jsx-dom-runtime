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

#[inline(always)]
pub fn html_dom_attribute(name: &str) -> Option<&'static str> {
    match name {
        "acceptCharset" => Some("accept-charset"),
        "className" => Some("class"),
        "httpEquiv" => Some("http-equiv"),
        "htmlFor" => Some("for"),
        "xlinkHref" => Some("href"),
        _ => None,
    }
}

#[inline(always)]
pub fn svg_dom_attribute(name: &str) -> Option<&'static str> {
    match name {
        "accentHeight" => Some("accent-height"),
        "alignmentBaseline" => Some("alignment-baseline"),
        "arabicForm" => Some("arabic-form"),
        "baselineShift" => Some("baseline-shift"),
        "capHeight" => Some("cap-height"),
        "clipPath" => Some("clip-path"),
        "clipRule" => Some("clip-rule"),
        "colorInterpolation" => Some("color-interpolation"),
        "colorInterpolationFilters" => Some("color-interpolation-filters"),
        "colorProfile" => Some("color-profile"),
        "colorRendering" => Some("color-rendering"),
        "dominantBaseline" => Some("dominant-baseline"),
        "enableBackground" => Some("enable-background"),
        "fillOpacity" => Some("fill-opacity"),
        "fillRule" => Some("fill-rule"),
        "floodColor" => Some("flood-color"),
        "floodOpacity" => Some("flood-opacity"),
        "fontFamily" => Some("font-family"),
        "fontSize" => Some("font-size"),
        "fontSizeAdjust" => Some("font-size-adjust"),
        "fontStretch" => Some("font-stretch"),
        "fontStyle" => Some("font-style"),
        "fontVariant" => Some("font-variant"),
        "fontWeight" => Some("font-weight"),
        "glyphName" => Some("glyph-name"),
        "glyphOrientationHorizontal" => Some("glyph-orientation-horizontal"),
        "glyphOrientationVertical" => Some("glyph-orientation-vertical"),
        "horizAdvX" => Some("horiz-adv-x"),
        "horizOriginX" => Some("horiz-origin-x"),
        "imageRendering" => Some("image-rendering"),
        "letterSpacing" => Some("letter-spacing"),
        "lightingColor" => Some("lighting-color"),
        "markerEnd" => Some("marker-end"),
        "markerMid" => Some("marker-mid"),
        "markerStart" => Some("marker-start"),
        "overlinePosition" => Some("overline-position"),
        "overlineThickness" => Some("overline-thickness"),
        "paintOrder" => Some("paint-order"),
        "panose1" => Some("panose-1"),
        "pointerEvents" => Some("pointer-events"),
        "renderingIntent" => Some("rendering-intent"),
        "shapeRendering" => Some("shape-rendering"),
        "stopColor" => Some("stop-color"),
        "stopOpacity" => Some("stop-opacity"),
        "strikethroughPosition" => Some("strikethrough-position"),
        "strikethroughThickness" => Some("strikethrough-thickness"),
        "strokeDasharray" => Some("stroke-dasharray"),
        "strokeDashoffset" => Some("stroke-dashoffset"),
        "strokeLinecap" => Some("stroke-linecap"),
        "strokeLinejoin" => Some("stroke-linejoin"),
        "strokeMiterlimit" => Some("stroke-miterlimit"),
        "strokeOpacity" => Some("stroke-opacity"),
        "strokeWidth" => Some("stroke-width"),
        "textAnchor" => Some("text-anchor"),
        "textDecoration" => Some("text-decoration"),
        "textRendering" => Some("text-rendering"),
        "underlinePosition" => Some("underline-position"),
        "underlineThickness" => Some("underline-thickness"),
        "unicodeBidi" => Some("unicode-bidi"),
        "unicodeRange" => Some("unicode-range"),
        "unitsPerEm" => Some("units-per-em"),
        "vAlphabetic" => Some("v-alphabetic"),
        "vHanging" => Some("v-hanging"),
        "vIdeographic" => Some("v-ideographic"),
        "vMathematical" => Some("v-mathematical"),
        "vectorEffect" => Some("vector-effect"),
        "vertAdvY" => Some("vert-adv-y"),
        "vertOriginX" => Some("vert-origin-x"),
        "vertOriginY" => Some("vert-origin-y"),
        "wordSpacing" => Some("word-spacing"),
        "writingMode" => Some("writing-mode"),
        "xHeight" => Some("x-height"),
        "xlinkActuate" => Some("xlink:actuate"),
        "xlinkArcrole" => Some("xlink:arcrole"),
        "xlinkHref" => Some("href"),
        "xlinkRole" => Some("xlink:role"),
        "xlinkShow" => Some("xlink:show"),
        "xlinkTitle" => Some("xlink:title"),
        "xlinkType" => Some("xlink:type"),
        "xmlBase" => Some("xml:base"),
        "xmlLang" => Some("xml:lang"),
        "xmlSpace" => Some("xml:space"),
        _ => None,
    }
}
