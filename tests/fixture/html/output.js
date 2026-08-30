import { jsx as _jsx, setSignalish as _setSignalish, setStyle as _setStyle, setAttributes as _setAttributes, setDataset as _setDataset } from "jsx-dom-runtime";
/*#__PURE__*/ _jsx("div", {});
/*#__PURE__*/ _jsx("button", {});
/*#__PURE__*/ _jsx("p", {}, _jsx("span", {}));
/*#__PURE__*/ _jsx("p", {}, [
    _jsx("span", {}),
    _jsx("strong", {})
]);
/*#__PURE__*/ _jsx("div", {}, _jsx("p", {}, _jsx("span", {}, "hello")));
/*#__PURE__*/ _jsx("div", {
    tabindex: 0
});
/*#__PURE__*/ _jsx("a", {
    class: "my-class"
});
/*#__PURE__*/ _jsx("a", {
    href: "https://example.com"
});
/*#__PURE__*/ _jsx("input", {
    required: "",
    readonly: "",
    disabled: "",
    spellcheck: "true"
});
/*#__PURE__*/ _jsx("div", {
    "aria-hidden": "true",
    "data-test": "true"
});
/*#__PURE__*/ _jsx("div", {
    "aria-hidden": "false",
    "data-test": "true"
});
/*#__PURE__*/ _jsx("div", {
    $: {
        click: ()=>console.log("clicked")
    }
});
/*#__PURE__*/ _jsx("div", {
    $: {
        click: ()=>1,
        mouseover: ()=>2
    }
});
/*#__PURE__*/ _jsx("input", {
    type: "text",
    $: {
        focusin: {
            handleEvent
        }
    }
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>e.foo = {}
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>e.foo = []
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>e.hello = "world"
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>e.hello = `hello ${user}`
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>e.setAttribute("test", "value")
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>{
        e.setAttribute("foo", "value");
        e.setAttribute("bar", 1);
    }
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>{
        e.setAttribute("foo", 'hello');
        e["bar-baz"] = {};
    }
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>{
        _setSignalish(data, (i)=>e.setAttribute("foo", i));
        _setSignalish(data, (i)=>e["bar-baz"] = i);
    }
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>e.setAttribute("foo", `hello ${user}`)
});
/*#__PURE__*/ _jsx("button", {
    type: "submit",
    ref: (e)=>e.onclick = handleClick
});
/*#__PURE__*/ _jsx("div", {
    style: "color: red; background-color: blue;"
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>_setStyle(e, {
            color: 'red',
            backgroundColor: 'blue'
        })
});
/*#__PURE__*/ _jsx("span", {
    ref: (e)=>_setAttributes(e, [])
});
/*#__PURE__*/ _jsx("iframe", {
    ref: (e)=>_setDataset(e, {
            foo: 'bar'
        })
});
/*#__PURE__*/ _jsx("div", {
    ref: (e)=>e.focus()
});
/*#__PURE__*/ _jsx("div", {
    ref: [
        (e)=>e.focus(),
        (e)=>{
            e.setAttribute("test", "value");
            e.foo = "bar";
            _setStyle(e, style);
            _setAttributes(e, attrs);
            e.onclick = handleClick;
        }
    ]
});
/*#__PURE__*/ _jsx("div", {}, _jsx("span", {}));
/*#__PURE__*/ _jsx("div", {}, [
    _jsx("small", {}),
    _jsx("strong", {})
]);
/*#__PURE__*/ _jsx("div", {}, Component({}));
/*#__PURE__*/ _jsx("em", {}, 10);
/*#__PURE__*/ _jsx("li", {}, "2");
/*#__PURE__*/ _jsx("ol", {}, "used");
