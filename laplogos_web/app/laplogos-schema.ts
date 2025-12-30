export const defaultPage = {
  v: 1,
  root: {
    i: "root",
    t: "Box",
    p: { pad: 10 },
    c: [
      { i: "t1", t: "Txt", p: { val: "Hello laplogos" } },
      {
        i: "btn1",
        t: "Btn",
        p: { label: "Set demo count" },
        on: { click: [{ t: "set", d: { key: "count", val: "demo" } }] },
      },
    ],
  },
};

const focusPage = {
  v: 1,
  root: {
    i: "root",
    t: "Box",
    c: [
      { i: "focus-label", t: "Txt", p: { val: "Focus test" } },
      { i: "focus-input", t: "Inp", p: { val: "$name" } },
    ],
  },
};

function reorderPage(order: string[], nextHref: string) {
  return {
    v: 1,
    root: {
      i: "root",
      t: "Box",
      c: [
        { i: "swap", t: "Lnk", p: { href: nextHref, label: "Reorder" } },
        { i: "row", t: "Inp", p: { val: "$item" }, for: order },
      ],
    },
  };
}

export function getPageSchema(pathname: string) {
  if (pathname === "/" || pathname === "") {
    return defaultPage;
  }
  if (pathname === "/e2e/focus") {
    return focusPage;
  }
  if (pathname === "/e2e/reorder-a") {
    return reorderPage(["a", "b", "c"], "/e2e/reorder-b");
  }
  if (pathname === "/e2e/reorder-b") {
    return reorderPage(["c", "b", "a"], "/e2e/reorder-a");
  }
  return defaultPage;
}
