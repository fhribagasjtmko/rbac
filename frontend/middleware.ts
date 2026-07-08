import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

// Route yang butuh login
const PROTECTED = ['/dashboard', '/profile', '/admin']
// Route khusus admin
const ADMIN_ONLY = ['/admin']
// Route yang tidak boleh diakses kalau sudah login
const AUTH_ROUTES = ['/auth/login', '/auth/register']

export function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl
  const accessToken  = request.cookies.get('access_token')?.value
  const isLoggedIn   = !!accessToken

  // Redirect ke login kalau belum login dan akses protected route
  const isProtected = PROTECTED.some(p => pathname.startsWith(p))
  if (isProtected && !isLoggedIn) {
    const url = request.nextUrl.clone()
    url.pathname = '/auth/login'
    url.searchParams.set('redirect', pathname)
    return NextResponse.redirect(url)
  }

  // Redirect ke dashboard kalau sudah login tapi akses halaman auth
  const isAuthRoute = AUTH_ROUTES.some(p => pathname.startsWith(p))
  if (isAuthRoute && isLoggedIn) {
    return NextResponse.redirect(new URL('/dashboard', request.url))
  }

  return NextResponse.next()
}

export const config = {
  matcher: [
    '/dashboard/:path*',
    '/profile/:path*',
    '/admin/:path*',
    '/auth/:path*',
  ],
}
