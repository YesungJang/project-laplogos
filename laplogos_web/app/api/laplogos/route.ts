import { NextResponse } from "next/server";
import { getPageSchema } from "../../laplogos-schema";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const page = searchParams.get("page") ?? "/";
  const schema = getPageSchema(page);
  return NextResponse.json(schema);
}
