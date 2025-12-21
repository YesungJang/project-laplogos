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

export function getPageSchema(pathname: string) {
  if (pathname === "/" || pathname === "") {
    return defaultPage;
  }
  return defaultPage;
}
