'use client'

import { useAuth } from '@/hooks/useAuth'
import Navbar from '@/components/layout/Navbar'
import { User, Shield, CheckCircle } from 'lucide-react'

export default function DashboardPage() {
  const { user } = useAuth()

  return (
    <>
      <Navbar />
      <main className="max-w-6xl mx-auto px-4 py-8">
        <div className="mb-8">
          <h1 className="text-2xl font-bold text-gray-900">
            Halo, {user?.name?.split(' ')[0]} 👋
          </h1>
          <p className="text-gray-500 mt-1">Selamat datang di dashboard.</p>
        </div>

        {/* Stats cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
          <div className="card p-5">
            <div className="flex items-center gap-3 mb-2">
              <div className="w-9 h-9 bg-blue-50 rounded-lg flex items-center justify-center">
                <User size={18} className="text-blue-600" />
              </div>
              <span className="text-sm font-medium text-gray-600">Role</span>
            </div>
            <p className="text-xl font-bold text-gray-900 capitalize">{user?.role}</p>
          </div>

          <div className="card p-5">
            <div className="flex items-center gap-3 mb-2">
              <div className="w-9 h-9 bg-green-50 rounded-lg flex items-center justify-center">
                <CheckCircle size={18} className="text-green-600" />
              </div>
              <span className="text-sm font-medium text-gray-600">Status</span>
            </div>
            <p className="text-xl font-bold text-gray-900">
              {user?.is_active ? 'Aktif' : 'Nonaktif'}
            </p>
          </div>

          <div className="card p-5">
            <div className="flex items-center gap-3 mb-2">
              <div className="w-9 h-9 bg-purple-50 rounded-lg flex items-center justify-center">
                <Shield size={18} className="text-purple-600" />
              </div>
              <span className="text-sm font-medium text-gray-600">Bergabung</span>
            </div>
            <p className="text-xl font-bold text-gray-900">
              {user?.created_at
                ? new Date(user.created_at).toLocaleDateString('id-ID', {
                    day: 'numeric', month: 'short', year: 'numeric',
                  })
                : '-'}
            </p>
          </div>
        </div>

        {/* Info box */}
        <div className="card p-6">
          <h2 className="font-semibold text-gray-900 mb-3">Info Akun</h2>
          <div className="space-y-3 text-sm">
            {[
              { label: 'ID', value: user?.id },
              { label: 'Email', value: user?.email },
              { label: 'Nama', value: user?.name },
              { label: 'Role', value: user?.role },
            ].map(({ label, value }) => (
              <div key={label} className="flex items-center gap-4">
                <span className="w-16 text-gray-500 flex-shrink-0">{label}</span>
                <span className="text-gray-900 font-mono text-xs bg-gray-50 px-2 py-1 rounded">
                  {value}
                </span>
              </div>
            ))}
          </div>
        </div>
      </main>
    </>
  )
}
