import { LaplogosApp } from "../components/LaplogosApp";

const initialPage = {
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

const initialState = {};

async function renderLaplogosHtml(page: unknown, state: unknown) {
  const mod = await import("laplogos_core");
  if (typeof mod.default === "function") {
    await mod.default();
  }
  return mod.render_laplogos_to_string(page, state);
}

export default async function Home() {
  const renderedHtml = await renderLaplogosHtml(initialPage, initialState);
  const bootstrapJson = JSON.stringify({ page: initialPage, state: initialState }).replace(
    /</g,
    "\\u003c"
  );
  const initialJson = JSON.stringify(initialPage);

  return (
    <LaplogosApp
      initialJson={initialJson}
      bootstrapJson={bootstrapJson}
      renderedHtml={renderedHtml}
    />
  );
}
