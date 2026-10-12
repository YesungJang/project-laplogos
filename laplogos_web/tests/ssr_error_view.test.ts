import assert from "node:assert/strict";
import test from "node:test";
import { renderErrorHtml } from "../lib/laplogos-ssr";
import {
  ERROR_VIEW_CONTAINER_CLASS,
  ERROR_VIEW_DEBUG_TEXT,
  ERROR_VIEW_FALLBACK_MESSAGE,
  ERROR_VIEW_RECOVERY_TEXT,
  ERROR_VIEW_TITLE,
} from "../lib/laplogos-error-view";

// `renderErrorHtml` is typed for strings; the invalid-input cases below go
// through this helper to exercise the runtime guard.
function render(message: unknown) {
  return renderErrorHtml(message as string);
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

test("SSR error view escapes message", () => {
  const html = renderErrorHtml("<tag>");
  assert.ok(html.includes("Laplogos Error"));
  assert.ok(html.includes("&lt;tag&gt;"));
  assert.ok(!html.includes("<tag>"));
});

test("SSR error view announces the error and shows a recovery instruction", () => {
  const html = renderErrorHtml("unsupported schema v: 2");

  assertAlertShell(html);
  assert.ok(html.includes("unsupported schema v: 2"));
});

test("SSR error view falls back when the message is blank", () => {
  for (const message of ["", "   ", "\n\t"]) {
    const html = renderErrorHtml(message);
    assertAlertShell(html);
    assert.ok(html.includes(ERROR_VIEW_FALLBACK_MESSAGE));
  }
});

test("SSR error view renders a very long message in full", () => {
  const message = `${"schema error ".repeat(1000)}end`;
  const html = renderErrorHtml(message);

  assertAlertShell(html);
  assert.ok(html.includes(message));
});

test("SSR error view escapes markup and quotes in the message", () => {
  const html = renderErrorHtml('<script>alert("xss")</script> & <b>bold</b>');

  assertAlertShell(html);
  assert.ok(!html.includes("<script>"));
  assert.ok(!html.includes("<b>"));
  assert.ok(html.includes("&lt;script&gt;"));
  assert.ok(html.includes("&quot;xss&quot;"));
  assert.ok(html.includes("&amp;"));
});

test("SSR error view keeps non-ASCII messages intact", () => {
  const message = "スキーマが不正です（v2 は未対応）";
  const html = renderErrorHtml(message);

  assertAlertShell(html);
  assert.ok(html.includes(message));
});

test("SSR error view falls back for non-string messages without throwing", () => {
  const invalid: unknown[] = [undefined, null, 42, true, { code: 1 }, ["a"]];

  for (const message of invalid) {
    const html = render(message);
    assertAlertShell(html);
    assert.ok(html.includes(ERROR_VIEW_FALLBACK_MESSAGE));
    assert.ok(!html.includes("undefined"));
    assert.ok(!html.includes("[object Object]"));
  }
});
