// SAFE: Redirect target validated against an allowlist of safe paths
import { NextRequest, NextResponse } from 'next/server';

const ALLOWED_PATHS = ['/dashboard', '/profile', '/settings', '/help'];

export async function GET(request: NextRequest) {
    const { searchParams } = new URL(request.url);
    const dest = searchParams.get('url') || '/';
    if (!ALLOWED_PATHS.includes(dest)) {
        return NextResponse.json({ error: 'Invalid redirect' }, { status: 400 });
    }
    return NextResponse.redirect(new URL(dest, request.url));
}
