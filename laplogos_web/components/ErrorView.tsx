'use client';

type Props = {
  message: string;
};

// Keep in sync with the recovery line in lib/laplogos-ssr.ts so that an
// SSR-rendered error and its hydrated counterpart stay identical.
const RECOVERY_INSTRUCTION =
  "ページを再読み込みし、解決しない場合は Debug View を確認してください。";

export function ErrorView({ message }: Props) {
  return (
    <div
      role="alert"
      className="rounded-md border border-rose-200 bg-rose-50 p-4 text-rose-900"
    >
      <h3 className="text-base font-semibold">Laplogos Error</h3>
      <p className="mt-2 text-sm">{message}</p>
      <p className="mt-2 text-xs text-rose-700">{RECOVERY_INSTRUCTION}</p>
    </div>
  );
}
