import { jsx as _jsx } from "jsx-dom-runtime";
Hello.World({});
A.B.C.D({});
lowercase.member({});
Member.Exp({ children: _jsx("div", {}) });
Member.Exp({ children: [_jsx("div", {}), _jsx("span", {})] });
A.B({ num: 1, str: "s", bool: true, obj: {}, arr: [1, 2, 3], empty: true });
A.B({ num: 1, str: "s", bool: true, obj: {}, arr: [1, 2, 3], children: "hello" });
A.B({ ...data, num: 1, str: "s", bool: true, obj: {}, arr: [1, 2, 3] });
