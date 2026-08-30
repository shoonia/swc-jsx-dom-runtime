import { mathmlNs as _mathmlNs, jsx as _jsx } from "jsx-dom-runtime";
import { mathmlNs } from "jsx-dom-runtime";
/*#__PURE__*/ _jsx("math", {
    _: _mathmlNs
});
/*#__PURE__*/ _jsx("mi", {
    class: "my-class",
    _: _mathmlNs
});
/*#__PURE__*/ _jsx("math", {
    _: _mathmlNs
}, _jsx("a", {
    _: _mathmlNs
}));
/*#__PURE__*/ _jsx("math", {
    _: _mathmlNs
}, check ? _jsx("a", {
    _: _mathmlNs
}) : null);
/*#__PURE__*/ _jsx("mi", {
    _: _mathmlNs
}, _jsx("a", {
    href: "https://example.com",
    _: _mathmlNs
}));
/*#__PURE__*/ _jsx("mi", {
    _: _mathmlNs
}, [
    _jsx("a", {
        href: "https://example.com",
        _: _mathmlNs
    }),
    _jsx("a", {
        href: "https://example.com",
        _: _mathmlNs
    })
]);
/*#__PURE__*/ _jsx("a", {
    _: mathmlNs
});
/*#__PURE__*/ _jsx("math", {
    ref: (e)=>e.foo = {},
    _: _mathmlNs
});
/*#__PURE__*/ _jsx("math", {
    ref: (e)=>e.foo = [],
    _: _mathmlNs
});
/*#__PURE__*/ _jsx("math", {
    ref: (e)=>e.hello = "world",
    _: _mathmlNs
});
/*#__PURE__*/ _jsx("math", {
    ref: (e)=>e.hello = `hello ${user}`,
    _: _mathmlNs
});
