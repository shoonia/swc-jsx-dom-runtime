/**
 * Source: https://github.com/swc-project/swc/blob/main/crates/swc_ecma_transforms_react/src/jsx/mod.rs
 */
use std::matches;
use swc_core::atoms::{
    wtf8::{Wtf8, Wtf8Buf},
    Atom, Wtf8Atom,
};
use swc_core::ecma::utils::str::is_line_terminator;

/// https://github.com/microsoft/TypeScript/blob/9e20e032effad965567d4a1e1c30d5433b0a3332/src/compiler/transformers/jsx.ts#L572-L608
///
/// JSX trims whitespace at the end and beginning of lines, except that the
/// start/end of a tag is considered a start/end of a line only if that line is
/// on the same line as the closing tag. See examples in
/// tests/cases/conformance/jsx/tsxReactEmitWhitespace.tsx
/// See also https://www.w3.org/TR/html4/struct/text.html#h-9.1 and https://www.w3.org/TR/CSS2/text.html#white-space-model
///
/// An equivalent algorithm would be:
/// - If there is only one line, return it.
/// - If there is only whitespace (but multiple lines), return `undefined`.
/// - Split the text into lines.
/// - 'trimRight' the first line, 'trimLeft' the last line, 'trim' middle lines.
/// - Decode entities on each line (individually).
/// - Remove empty lines and join the rest with " ".
///
/// This version takes both `value` (decoded) and `raw` (original source) to
/// preserve whitespace that was explicitly encoded as HTML entities like
/// `&#32;`, `&#9;`, `&#10;`, `&#13;`.
#[inline]
pub fn jsx_text_to_str_with_raw(value: &Wtf8Atom, raw: &Atom) -> Wtf8Atom {
    // Fast path: if no HTML entities (raw == value), use the simple algorithm
    if let Some(value) = value.as_str() {
        if value == raw.as_str() {
            return jsx_text_to_str_impl(value).into();
        }
    }
    // Build a mask of which code point positions in value came from HTML
    // entities.
    let entity_mask = build_entity_mask(value, raw);

    jsx_text_to_str_with_entity_mask(value, &entity_mask)
}

/// Build a mask indicating which character positions in `value` came from HTML
/// entities in `raw`.
///
/// Returns a Vec<bool> where true means the character at that index was from an
/// entity.
fn build_entity_mask(value: &Wtf8Atom, raw: &str) -> Vec<bool> {
    let cp = value.as_wtf8().code_points();
    let mut mask = vec![false; cp.count()];
    let mut value_char_idx = 0;
    let mut raw_chars = raw.chars().peekable();

    while let Some(raw_c) = raw_chars.next() {
        if raw_c == '&' {
            let mut entity = raw_chars.clone();
            let mut s = String::new();
            let mut consumed = 0;
            let mut has_prev_result = false;
            let mut entity_code_points = 0;

            for _ in 0..20 {
                let Some(c) = entity.next() else {
                    break;
                };
                consumed += 1;

                if c != ';' {
                    s.push(c);
                    continue;
                }

                if let Some(result) = parse_jsx_numeric_entity(&s) {
                    if (0xd800..=0xdfff).contains(&result) {
                        if result < 0xdc00 {
                            if has_prev_result {
                                entity_code_points += 1;
                            }

                            if entity.next() == Some('&') && entity.next() == Some('#') {
                                consumed += 2;
                                has_prev_result = true;
                                s.clear();
                                s.push('#');
                                continue;
                            }

                            entity_code_points += 1;
                            break;
                        } else if has_prev_result {
                            entity_code_points += 1;
                            break;
                        }
                    }

                    if has_prev_result {
                        entity_code_points += 1;
                    }
                    entity_code_points += 1;
                    break;
                } else if is_known_html_entity(&s) {
                    entity_code_points += 1;
                    break;
                } else {
                    break;
                }
            }

            if entity_code_points != 0 {
                raw_chars.by_ref().take(consumed).count();

                let end = value_char_idx + entity_code_points;
                let mask_end = mask.len().min(end);
                mask[value_char_idx..mask_end].fill(true);
                value_char_idx = end;
            } else {
                value_char_idx += 1;
            }
        } else {
            // Regular character
            value_char_idx += 1;
        }
    }

    mask
}

fn parse_jsx_numeric_entity(s: &str) -> Option<u32> {
    if let Some(stripped) = s.strip_prefix('#') {
        if let Some(hex) = stripped
            .strip_prefix('x')
            .or_else(|| stripped.strip_prefix('X'))
        {
            if !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit()) {
                u32::from_str_radix(hex, 16).ok().filter(|&v| v <= 0x10ffff)
            } else {
                None
            }
        } else if !stripped.is_empty() && stripped.chars().all(|c| c.is_ascii_digit()) {
            stripped.parse::<u32>().ok().filter(|&v| v <= 0x10ffff)
        } else {
            None
        }
    } else {
        None
    }
}

/// Check if name is a known HTML entity
fn is_known_html_entity(name: &str) -> bool {
    // Common HTML entities that decode to whitespace or other characters
    // This list matches the entities defined in swc_ecma_lexer jsx.rs xhtml!
    // macro
    matches!(
        name,
        "nbsp"
            | "iexcl"
            | "cent"
            | "pound"
            | "curren"
            | "yen"
            | "brvbar"
            | "sect"
            | "uml"
            | "copy"
            | "ordf"
            | "laquo"
            | "not"
            | "shy"
            | "reg"
            | "macr"
            | "deg"
            | "plusmn"
            | "sup2"
            | "sup3"
            | "acute"
            | "micro"
            | "para"
            | "middot"
            | "cedil"
            | "sup1"
            | "ordm"
            | "raquo"
            | "frac14"
            | "frac12"
            | "frac34"
            | "iquest"
            | "Agrave"
            | "Aacute"
            | "Acirc"
            | "Atilde"
            | "Auml"
            | "Aring"
            | "AElig"
            | "Ccedil"
            | "Egrave"
            | "Eacute"
            | "Ecirc"
            | "Euml"
            | "Igrave"
            | "Iacute"
            | "Icirc"
            | "Iuml"
            | "ETH"
            | "Ntilde"
            | "Ograve"
            | "Oacute"
            | "Ocirc"
            | "Otilde"
            | "Ouml"
            | "times"
            | "Oslash"
            | "Ugrave"
            | "Uacute"
            | "Ucirc"
            | "Uuml"
            | "Yacute"
            | "THORN"
            | "szlig"
            | "agrave"
            | "aacute"
            | "acirc"
            | "atilde"
            | "auml"
            | "aring"
            | "aelig"
            | "ccedil"
            | "egrave"
            | "eacute"
            | "ecirc"
            | "euml"
            | "igrave"
            | "iacute"
            | "icirc"
            | "iuml"
            | "eth"
            | "ntilde"
            | "ograve"
            | "oacute"
            | "ocirc"
            | "otilde"
            | "ouml"
            | "divide"
            | "oslash"
            | "ugrave"
            | "uacute"
            | "ucirc"
            | "uuml"
            | "yacute"
            | "thorn"
            | "yuml"
            | "OElig"
            | "oelig"
            | "Scaron"
            | "scaron"
            | "Yuml"
            | "fnof"
            | "circ"
            | "tilde"
            | "Alpha"
            | "Beta"
            | "Gamma"
            | "Delta"
            | "Epsilon"
            | "Zeta"
            | "Eta"
            | "Theta"
            | "Iota"
            | "Kappa"
            | "Lambda"
            | "Mu"
            | "Nu"
            | "Xi"
            | "Omicron"
            | "Pi"
            | "Rho"
            | "Sigma"
            | "Tau"
            | "Upsilon"
            | "Phi"
            | "Chi"
            | "Psi"
            | "Omega"
            | "alpha"
            | "beta"
            | "gamma"
            | "delta"
            | "epsilon"
            | "zeta"
            | "eta"
            | "theta"
            | "iota"
            | "kappa"
            | "lambda"
            | "mu"
            | "nu"
            | "xi"
            | "omicron"
            | "pi"
            | "rho"
            | "sigmaf"
            | "sigma"
            | "tau"
            | "upsilon"
            | "phi"
            | "chi"
            | "psi"
            | "omega"
            | "thetasym"
            | "upsih"
            | "piv"
            | "ensp"
            | "emsp"
            | "thinsp"
            | "zwnj"
            | "zwj"
            | "lrm"
            | "rlm"
            | "ndash"
            | "mdash"
            | "lsquo"
            | "rsquo"
            | "sbquo"
            | "ldquo"
            | "rdquo"
            | "bdquo"
            | "dagger"
            | "Dagger"
            | "bull"
            | "hellip"
            | "permil"
            | "prime"
            | "Prime"
            | "lsaquo"
            | "rsaquo"
            | "oline"
            | "frasl"
            | "euro"
            | "image"
            | "weierp"
            | "real"
            | "trade"
            | "alefsym"
            | "larr"
            | "uarr"
            | "rarr"
            | "darr"
            | "harr"
            | "crarr"
            | "lArr"
            | "uArr"
            | "rArr"
            | "dArr"
            | "hArr"
            | "forall"
            | "part"
            | "exist"
            | "empty"
            | "nabla"
            | "isin"
            | "notin"
            | "ni"
            | "prod"
            | "sum"
            | "minus"
            | "lowast"
            | "radic"
            | "prop"
            | "infin"
            | "ang"
            | "and"
            | "or"
            | "cap"
            | "cup"
            | "int"
            | "there4"
            | "sim"
            | "cong"
            | "asymp"
            | "ne"
            | "equiv"
            | "le"
            | "ge"
            | "sub"
            | "sup"
            | "nsub"
            | "sube"
            | "supe"
            | "oplus"
            | "otimes"
            | "perp"
            | "sdot"
            | "lceil"
            | "rceil"
            | "lfloor"
            | "rfloor"
            | "lang"
            | "rang"
            | "loz"
            | "spades"
            | "clubs"
            | "hearts"
            | "diams"
            | "quot"
            | "amp"
            | "apos"
            | "lt"
            | "gt"
    )
}

/// JSX text processing with entity mask - preserves whitespace from HTML
/// entities.
fn jsx_text_to_str_with_entity_mask(t: &Wtf8, entity_mask: &[bool]) -> Wtf8Atom {
    // Fast path: if no line terminators and no trimmable whitespace
    // (whitespace that's not from entities at the leading edge)
    let chars: Vec<_> = wtf8_chars(t).collect();
    let has_line_terminator = chars.iter().any(|(_, _, c)| is_line_terminator(*c));

    // For single-line text, we keep all whitespace (matching original behavior)
    // The original jsx_text_to_str_impl preserves leading/trailing whitespace on
    // single-line text
    if !t.is_empty() && !has_line_terminator {
        return t.into();
    }

    let mut acc: Option<Wtf8Buf> = None;
    let mut only_line: Option<(usize, usize)> = None;
    let mut line_start: Option<usize> = Some(0);
    let mut line_end: Option<usize> = None;
    // The first line preserves leading whitespace; subsequent lines trim it.
    let mut is_first_line = true;

    for (char_idx, (_, _, c)) in chars.iter().enumerate() {
        let is_from_entity = *entity_mask.get(char_idx).unwrap_or(&false);

        if is_line_terminator(*c) {
            // Process current line - trim both leading AND trailing (intermediate
            // line)
            if let (Some(start), Some(end)) = (line_start, line_end) {
                let line_range =
                    extract_line_content(&chars, start, end, entity_mask, !is_first_line, true);
                if let Some((line_start, line_end)) = line_range {
                    add_line_of_jsx_text_wtf8(line_start, line_end, t, &mut acc, &mut only_line);
                }
            }
            is_first_line = false;
            line_start = None;
            line_end = None;
        } else if !is_white_space_single_line(*c) || is_from_entity {
            // Non-whitespace or entity-derived whitespace - counts as content
            line_end = Some(char_idx + 1);
            if line_start.is_none() {
                line_start = Some(char_idx);
            }
        }
    }

    // Handle final line. Leading whitespace is preserved only if this is still
    // the first line (single-line input).
    if let Some(start) = line_start {
        let line_range = extract_line_content(
            &chars,
            start,
            chars.len(),
            entity_mask,
            !is_first_line,
            false,
        );
        if let Some((line_start, line_end)) = line_range {
            add_line_of_jsx_text_wtf8(line_start, line_end, t, &mut acc, &mut only_line);
        }
    }

    if let Some(acc) = acc {
        acc.into()
    } else if let Some((start, end)) = only_line {
        t.slice(start, end).into()
    } else {
        Wtf8Atom::default()
    }
}

fn wtf8_chars(t: &Wtf8) -> impl Iterator<Item = (usize, usize, char)> + '_ {
    let mut byte_pos = 0;

    t.code_points().map(move |cp| {
        let start = byte_pos;
        let cp_value = cp.to_u32();
        let cp_byte_len = if cp_value < 0x80 {
            1
        } else if cp_value < 0x800 {
            2
        } else if cp_value < 0x10000 {
            3
        } else {
            4
        };
        byte_pos += cp_byte_len;

        (start, byte_pos, cp.to_char_lossy())
    })
}

/// Extract line content, optionally trimming non-entity whitespace from edges
///
/// - `trim_leading`: if true, trim leading non-entity whitespace
/// - `trim_trailing`: if true, trim trailing non-entity whitespace
fn extract_line_content(
    chars: &[(usize, usize, char)],
    start: usize,
    end: usize,
    entity_mask: &[bool],
    trim_leading: bool,
    trim_trailing: bool,
) -> Option<(usize, usize)> {
    // Find first non-trimmable position (if trim_leading is true)
    let mut actual_start = start;
    if trim_leading {
        while actual_start < end {
            let c = chars[actual_start].2;
            let is_from_entity = *entity_mask.get(actual_start).unwrap_or(&false);
            if !is_white_space_single_line(c) || is_from_entity {
                break;
            }
            actual_start += 1;
        }
    }

    // Find last non-trimmable position (if trim_trailing is true)
    let mut actual_end = end;
    if trim_trailing {
        while actual_end > actual_start {
            let c = chars[actual_end - 1].2;
            let is_from_entity = *entity_mask.get(actual_end - 1).unwrap_or(&false);
            if !is_white_space_single_line(c) || is_from_entity {
                break;
            }
            actual_end -= 1;
        }
    }

    if actual_start == actual_end {
        None
    } else {
        Some((chars[actual_start].0, chars[actual_end - 1].1))
    }
}

/// Helper for adding lines of JSX text when handling Wtf8 with surrogates
fn add_line_of_jsx_text_wtf8(
    line_start: usize,
    line_end: usize,
    source: &Wtf8,
    acc: &mut Option<Wtf8Buf>,
    only_line: &mut Option<(usize, usize)>,
) {
    if let Some((only_start, only_end)) = only_line.take() {
        // Second line - create accumulator
        let mut buffer = Wtf8Buf::with_capacity(source.len());
        buffer.push_wtf8(source.slice(only_start, only_end));
        buffer.push_str(" ");
        buffer.push_wtf8(source.slice(line_start, line_end));
        *acc = Some(buffer);
    } else if let Some(ref mut buffer) = acc {
        // Subsequent lines
        buffer.push_str(" ");
        buffer.push_wtf8(source.slice(line_start, line_end));
    } else {
        // First line
        *only_line = Some((line_start, line_end));
    }
}

/// Internal implementation that works with &str
#[inline]
fn jsx_text_to_str_impl(t: &str) -> Atom {
    // Fast path: if no line terminators and no leading/trailing whitespace
    if !t.is_empty()
        && !t.chars().any(is_line_terminator)
        && !t.starts_with(is_white_space_single_line)
        && !t.ends_with(is_white_space_single_line)
    {
        return t.into();
    }

    let mut acc: Option<String> = None;
    let mut only_line: Option<&str> = None;
    let mut first_non_whitespace: Option<usize> = Some(0);
    let mut last_non_whitespace: Option<usize> = None;

    for (index, c) in t.char_indices() {
        if is_line_terminator(c) {
            if let (Some(first), Some(last)) = (first_non_whitespace, last_non_whitespace) {
                let line_text = &t[first..last];
                add_line_of_jsx_text(line_text, &mut acc, &mut only_line);
            }
            first_non_whitespace = None;
        } else if !is_white_space_single_line(c) {
            last_non_whitespace = Some(index + c.len_utf8());
            if first_non_whitespace.is_none() {
                first_non_whitespace.replace(index);
            }
        }
    }

    if let Some(first) = first_non_whitespace {
        let line_text = &t[first..];
        add_line_of_jsx_text(line_text, &mut acc, &mut only_line);
    }

    if let Some(acc) = acc {
        acc.into()
    } else if let Some(only_line) = only_line {
        only_line.into()
    } else {
        "".into()
    }
}

/// [TODO]: Re-validate this whitespace handling logic.
///
/// We cannot use [swc_ecma_utils::str::is_white_space_single_line] because
/// HTML entities (like `&nbsp;` → `\u{00a0}`) are pre-processed by the parser,
/// making it impossible to distinguish them from literal Unicode characters. We
/// should never trim HTML entities.
///
/// As a reference, Babel only trims regular spaces and tabs, so this is a
/// simplified implementation already in use.
/// https://github.com/babel/babel/blob/e5c8dc7330cb2f66c37637677609df90b31ff0de/packages/babel-types/src/utils/react/cleanJSXElementLiteralChild.ts#L28-L39
fn is_white_space_single_line(c: char) -> bool {
    matches!(c, ' ' | '\t')
}

// less allocations trick from OXC
// https://github.com/oxc-project/oxc/blob/4c35f4abb6874bd741b84b34df7889637425e9ea/crates/oxc_transformer/src/jsx/jsx_impl.rs#L1061-L1091
fn add_line_of_jsx_text<'a>(
    trimmed_line: &'a str,
    acc: &mut Option<String>,
    only_line: &mut Option<&'a str>,
) {
    if let Some(buffer) = acc.as_mut() {
        // Already some text in accumulator. Push a space before this line is added to
        // `acc`.
        buffer.push(' ');
    } else if let Some(only_line_content) = only_line.take() {
        // This is the 2nd line containing text. Previous line did not contain any HTML
        // entities. Generate an accumulator containing previous line and a
        // trailing space. Current line will be added to the accumulator after
        // it.
        let mut buffer = String::with_capacity(trimmed_line.len() * 2); // rough estimate
        buffer.push_str(only_line_content);
        buffer.push(' ');
        *acc = Some(buffer);
    }

    // [TODO]: Decode any HTML entities in this line

    // For now, just use the trimmed line directly
    if let Some(buffer) = acc.as_mut() {
        buffer.push_str(trimmed_line);
    } else {
        // This is the first line containing text, and there are no HTML entities in
        // this line. Record this line in `only_line`.
        // If this turns out to be the only line, we won't need to construct a String,
        // so avoid all copying.
        *only_line = Some(trimmed_line);
    }
}
