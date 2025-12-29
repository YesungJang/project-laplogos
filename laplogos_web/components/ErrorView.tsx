'use client';

type Props = {
  message: string;
};

export function ErrorView({ message }: Props) {
  return (
    <div className="rounded-md border border-rose-200 bg-rose-50 p-4 text-rose-900">
      <h3 className="text-base font-semibold">Laplogos Error</h3>
      <p className="mt-2 text-sm">{message}</p>
      <p className="mt-2 text-xs text-rose-700">Debug View を確認してください。</p>
    </div>
  );
}
