import { mathmlNs as _mathmlNs, jsx as _jsx } from "jsx-dom-runtime";
_jsx("math", { _: _mathmlNs });
_jsx("mi", { class: "my-class", _: _mathmlNs });
_jsx("math", { _: _mathmlNs }, _jsx("a", { _: _mathmlNs }));
_jsx("math", { _: _mathmlNs }, check ? _jsx("a", { _: _mathmlNs }) : null);
