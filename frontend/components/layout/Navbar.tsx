'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { useAuth } from '@/hooks/useAuth'
import { LogOut, User, LayoutDashboard, Shield, Loader2 } from 'lucide-react'
import clsx from 'clsx'

const navLinks = [
  { href: '/dashboard', label: 'Dashboard', icon: LayoutDashboard },
  { href: '/profile',   label: 'Profil',    icon: User },
]

export default function Navbar() {
  const { user, isAdmin, logout, loading } = useAuth()
  const pathname = usePathname()

  if (loading) return null

  return (
    <nav className="bg-white border-b border-gray-200 sticky top-0 z-50">
      <div className="max-w-6xl mx-auto px-4 h-14 flex items-center justify-between">

        {/* Logo */}
        <Link href="/dashboard" className="flex items-center gap-2 font-bold text-gray-900">
          <div className="w-7 h-7 bg-brand-600 rounded-lg flex items-center justify-center">
            <span className="text-white text-xs font-bold">M</span>
          </div>
          MyApp
        </Link>

        {/* Nav links */}
        <div className="hidden md:flex items-center gap-1">
          {navLinks.map(({ href, label, icon: Icon }) => (
            <Link
              key={href}
              href={href}
              className={clsx(
                'flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-colors',
                pathname.startsWith(href)
                  ? 'bg-brand-50 text-brand-600'
                  : 'text-gray-600 hover:bg-gray-100'
              )}
            >
              <Icon size={15} />
              {label}
            </Link>
          ))}

          {/* Admin link - hanya tampil untuk admin */}
          {isAdmin && (
            <Link
              href="/admin/users"
              className={clsx(
                'flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-colors',
                pathname.startsWith('/admin')
                  ? 'bg-purple-50 text-purple-600'
                  : 'text-gray-600 hover:bg-gray-100'
              )}
            >
              <Shield size={15} />
              Admin
            </Link>
          )}
        </div>

        {/* User info + logout */}
        <div className="flex items-center gap-3">
          <span className="text-sm text-gray-600 hidden md:block">
            {user?.name}
            {user?.role === 'admin' && (
              <span className="ml-1.5 text-xs bg-purple-100 text-purple-700 px-1.5 py-0.5 rounded-full font-medium">
                Admin
              </span>
            )}
          </span>
          <button
            onClick={logout}
            className="flex items-center gap-1.5 text-sm text-gray-500 hover:text-red-600 transition-colors"
          >
            <LogOut size={15} />
            <span className="hidden md:block">Keluar</span>
          </button>
        </div>
      </div>
    </nav>
  )
}
