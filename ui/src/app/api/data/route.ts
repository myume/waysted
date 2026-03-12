import { NextRequest, NextResponse } from 'next/server';
import { getScreentime, getLogs, getTitleBreakdown } from '@/lib/cli';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const date = searchParams.get('date') || new Date().toISOString().split('T')[0];
  const type = searchParams.get('type') || 'screentime';

  try {
    let data;
    if (type === 'screentime') {
      data = await getScreentime(date);
    } else if (type === 'logs') {
      data = await getLogs(date);
    } else if (type === 'titles') {
      data = await getTitleBreakdown(date);
    }
    return NextResponse.json(data);
  } catch (error) {
    return NextResponse.json({ error: (error as Error).message }, { status: 500 });
  }
}
