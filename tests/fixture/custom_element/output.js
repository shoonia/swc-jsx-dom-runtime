import { jsx as _jsx } from "jsx-dom-runtime";
_jsx("web-component", {});
_jsx("web-component", { class: "a", className: "b" });
_jsx("web-component", { $: { "custom-event": () => 0, click: () => 1 } });
_jsx("x-component", { $: { CaseSensitiveEvent: fn, Snake_Case_Event_Name: fn, "Kebab-Case-Name": fn } });
