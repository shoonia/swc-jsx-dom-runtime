import { jsx as _jsx } from "jsx-dom-runtime";
_jsx("div", {});
_jsx("button", {});
_jsx("p", {}, _jsx("span", {}));
_jsx("p", {}, [_jsx("span", {}), _jsx("strong", {})]);
_jsx("div", {}, _jsx("p", {}, _jsx("span", {}, "hello")));
_jsx("div", { tabindex: 0 });
_jsx("a", { class: "my-class" });
