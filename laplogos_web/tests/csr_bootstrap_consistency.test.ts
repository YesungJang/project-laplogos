import assert from "node:assert/strict";
import test from "node:test";
import { escapeJsonForScript, renderLaplogosHtml } from "../lib/laplogos-ssr";

test("bootstrap keeps raw schema and renders consistently", async () => {
  const page = {
    v: 1,
    root: {
      i: "root",
      t: "Box",
      c: [{ i: "title", t: "Txt", p: { val: "$title" } }],
    },
  };
  const state = { title: "CSR" };

  const bootstrap = escapeJsonForScript({ page, state });
  const parsed = JSON.parse(bootstrap) as { page: typeof page; state: typeof state };

  assert.deepEqual(parsed.page, page);
  assert.deepEqual(parsed.state, state);

  const html = await renderLaplogosHtml(parsed.page, parsed.state);
  assert.equal(
    html,
    "<div data-laplogos-id=\"root\"><p data-laplogos-id=\"title\">CSR</p></div>"
  );
});
