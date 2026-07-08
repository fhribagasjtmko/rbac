'use client'

import Navbar from '@/components/layout/Navbar'
import { useAuth } from '@/hooks/useAuth'
import { User, Mail, Calendar, ShieldCheck } from 'lucide-react'

export default function ProfilePage() {
  const { user } = useAuth()

  return (
    <>
      <Navbar />
      <main className="max-w-2xl mx-auto px-4 py-8">
        <h1 className="text-xl font-bold text-gray-900 mb-6">Profil Saya</h1>

        <div className="card p-6">
          {/* Avatar */}
          <div className="flex items-center gap-4 mb-6 pb-6 border-b border-gray-100">
            <div className="w-16 h-16 bg-brand-600 rounded-full flex items-center justify-center text-white text-2xl font-bold">
              {user?.name?.charAt(0).toUpperCase()}
            </div>
            <div>
              <h2 className="text-lg font-semibold text-gray-900">{user?.name}</h2>
              <p className="text-sm text-gray-500">{user?.email}</p>
              <span className={`inline-block mt-1 text-xs font-medium px-2 py-0.5 rounded-full capitalize ${
                user?.role === 'admin'
                  ? 'bg-purple-100 text-purple-700'
                  : 'bg-gray-100 text-gray-600'
              }`}>
                {user?.role}
              </span>
            </div>
          </div>

          {/* Info detail */}
          <div className="space-y-4 text-sm">
            {[
              { icon: User,        label: 'Nama',      value: user?.name },
              { icon: Mail,        label: 'Email',     value: user?.email },
              { icon: ShieldCheck, label: 'Role',      value: user?.role },
              {
                icon: Calendar,
                label: 'Bergabung',
                value: user?.created_at
                  ? new Date(user.created_at).toLocaleDateString('id-ID', {
                      weekday: 'long', day: 'numeric', month: 'long', year: 'numeric',
                    })
                  : '-',
              },
            ].map(({ icon: Icon, label, value }) => (
              <div key={label} className="flex items-start gap-3">
                <Icon size={16} className="text-gray-400 mt-0.5 flex-shrink-0" />
                <div>
                  <span className="block text-xs text-gray-400">{label}</span>
                  <span className="text-gray-900">{value}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      </main>
    </>
  )
}
