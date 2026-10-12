import assert from "node:assert/strict";
import test from "node:test";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { ErrorView } from "../components/ErrorView";
import { renderErrorHtml } from "../lib/laplogos-ssr";
import {
  ERROR_VIEW_CONTAINER_CLASS,
  ERROR_VIEW_DEBUG_TEXT,
  ERROR_VIEW_FALLBACK_MESSAGE,
  ERROR_VIEW_RECOVERY_TEXT,
  ERROR_VIEW_TITLE,
} from "../lib/laplogos-error-view";

// `test:node` globs `tests/*.test.ts`, so this file stays JSX-free and builds
// the element with `createElement`.
function render(message: unknown) {
  return renderToStaticMarkup(
    createElement(ErrorView, { message } as { message: string })
  );
}

function assertAlertShell(html: string) {
  assert.ok(html.includes('role="alert"'), "the container must be an alert");
  assert.ok(html.includes('aria-live="assertive"'));
  assert.ok(html.includes('aria-atomic="true"'));
  assert.ok(html.includes(`class="${ERROR_VIEW_CONTAINER_CLASS}"`));
  assert.ok(html.includes(ERROR_VIEW_TITLE));
  assert.ok(html.includes(ERROR_VIEW_RECOVERY_TEXT), "recovery instruction");
  assert.ok(html.includes(ERROR_VIEW_DEBUG_TEXT));
}

test("ErrorView announces the error and shows a recovery instruction", () => {
  const html = render("unsupported schema v: 2");

  assertAlertShell(html);
  assert.ok(html.includes("unsupported schema v: 2"));
});

test("ErrorView and the SSR error html stay in parity", () => {
  const message = "root if=false is fatal";

  for (const html of [render(message), renderErrorHtml(message)]) {
    assertAlertShell(html);
    assert.ok(html.includes(message));
  }
});

test("ErrorView falls back when the message is blank", () => {
  for (const message of ["", "   ", "\n\t"]) {
    const html = render(message);
    assertAlertShell(html);
    assert.ok(html.includes(ERROR_VIEW_FALLBACK_MESSAGE));
  }
});

test("ErrorView renders a very long message in full", () => {
  const message = `${"schema error ".repeat(1000)}end`;
  const html = render(message);

  assertAlertShell(html);
  assert.ok(html.includes(message));
});

test("ErrorView renders markup-looking messages as plain text", () => {
  const html = render('<script>alert("xss")</script> & <b>bold</b>');

  assertAlertShell(html);
  assert.ok(!html.includes("<script>"));
  assert.ok(!html.includes("<b>"));
  assert.ok(html.includes("&lt;script&gt;"));
  assert.ok(html.includes("&amp;"));
});

test("ErrorView keeps non-ASCII messages intact", () => {
  const message = "スキーマが不正です（v2 は未対応）";
  const html = render(message);

  assertAlertShell(html);
  assert.ok(html.includes(message));
});

test("ErrorView falls back for non-string messages without throwing", () => {
  const invalid: unknown[] = [undefined, null, 42, true, { code: 1 }, ["a"]];

  for (const message of invalid) {
    const html = render(message);
    assertAlertShell(html);
    assert.ok(html.includes(ERROR_VIEW_FALLBACK_MESSAGE));
    assert.ok(!html.includes("undefined"));
    assert.ok(!html.includes("[object Object]"));
  }
});
