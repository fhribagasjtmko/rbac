'use client'

import { useEffect, useState } from 'react'
import Navbar from '@/components/layout/Navbar'
import { adminService } from '@/lib/admin'
import { useAuth } from '@/hooks/useAuth'
import { useRouter } from 'next/navigation'
import toast from 'react-hot-toast'
import { Loader2, Shield, UserCheck, UserX, Trash2 } from 'lucide-react'
import type { User } from '@/types'

const ROLES = ['user', 'moderator', 'admin']

export default function AdminUsersPage() {
  const { isAdmin, loading: authLoading } = useAuth()
  const router = useRouter()

  const [users, setUsers]       = useState<User[]>([])
  const [loading, setLoading]   = useState(true)
  const [actionId, setActionId] = useState<string | null>(null)

  // Guard: kalau bukan admin, redirect
  useEffect(() => {
    if (!authLoading && !isAdmin) router.replace('/dashboard')
  }, [isAdmin, authLoading])

  const fetchUsers = async () => {
    try {
      const data = await adminService.listUsers()
      setUsers(data.users)
    } catch {
      toast.error('Gagal memuat daftar user')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => { fetchUsers() }, [])

  const handleRoleChange = async (userId: string, newRole: string) => {
    setActionId(userId)
    try {
      await adminService.updateRole(userId, newRole)
      toast.success('Role berhasil diubah')
      fetchUsers()
    } catch (err: any) {
      toast.error(err?.response?.data?.message || 'Gagal ubah role')
    } finally {
      setActionId(null)
    }
  }

  const handleToggleActive = async (user: User) => {
    setActionId(user.id)
    try {
      if (user.is_active) {
        await adminService.deactivateUser(user.id)
        toast.success('User dinonaktifkan')
      } else {
        await adminService.activateUser(user.id)
        toast.success('User diaktifkan kembali')
      }
      fetchUsers()
    } catch (err: any) {
      toast.error(err?.response?.data?.message || 'Gagal')
    } finally {
      setActionId(null)
    }
  }

  const handleDelete = async (user: User) => {
    if (!confirm(`Hapus user "${user.name}"? Tindakan ini tidak bisa dibatalkan.`)) return
    setActionId(user.id)
    try {
      await adminService.deleteUser(user.id)
      toast.success('User dihapus')
      fetchUsers()
    } catch (err: any) {
      toast.error(err?.response?.data?.message || 'Gagal menghapus user')
    } finally {
      setActionId(null)
    }
  }

  const roleBadge = (role: string) => {
    const map: Record<string, string> = {
      admin: 'bg-purple-100 text-purple-700',
      moderator: 'bg-blue-100 text-blue-700',
      user: 'bg-gray-100 text-gray-600',
    }
    return map[role] ?? map.user
  }

  if (authLoading || loading) {
    return (
      <>
        <Navbar />
        <div className="flex justify-center items-center h-64">
          <Loader2 className="animate-spin text-brand-600" size={28} />
        </div>
      </>
    )
  }

  return (
    <>
      <Navbar />
      <main className="max-w-6xl mx-auto px-4 py-8">
        <div className="flex items-center gap-3 mb-6">
          <Shield size={22} className="text-purple-600" />
          <div>
            <h1 className="text-xl font-bold text-gray-900">Manajemen User</h1>
            <p className="text-sm text-gray-500">{users.length} user terdaftar</p>
          </div>
        </div>

        <div className="card overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead className="bg-gray-50 border-b border-gray-200">
                <tr>
                  {['Nama', 'Email', 'Role', 'Status', 'Bergabung', 'Aksi'].map(h => (
                    <th key={h} className="text-left px-4 py-3 text-xs font-semibold text-gray-500 uppercase tracking-wide">
                      {h}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-100">
                {users.map(user => (
                  <tr key={user.id} className="hover:bg-gray-50 transition-colors">
                    <td className="px-4 py-3 font-medium text-gray-900">{user.name}</td>
                    <td className="px-4 py-3 text-gray-600">{user.email}</td>

                    {/* Role selector */}
                    <td className="px-4 py-3">
                      <select
                        value={user.role}
                        disabled={actionId === user.id}
                        onChange={e => handleRoleChange(user.id, e.target.value)}
                        className={`text-xs font-medium px-2 py-1 rounded-full border-0 cursor-pointer focus:ring-2 focus:ring-brand-500 ${roleBadge(user.role)}`}
                      >
                        {ROLES.map(r => (
                          <option key={r} value={r}>{r}</option>
                        ))}
                      </select>
                    </td>

                    {/* Status */}
                    <td className="px-4 py-3">
                      <span className={`inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full ${user.is_active ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-600'}`}>
                        <span className={`w-1.5 h-1.5 rounded-full ${user.is_active ? 'bg-green-500' : 'bg-red-500'}`} />
                        {user.is_active ? 'Aktif' : 'Nonaktif'}
                      </span>
                    </td>

                    {/* Tanggal */}
                    <td className="px-4 py-3 text-gray-500">
                      {new Date(user.created_at).toLocaleDateString('id-ID', {
                        day: 'numeric', month: 'short', year: 'numeric',
                      })}
                    </td>

                    {/* Actions */}
                    <td className="px-4 py-3">
                      <div className="flex items-center gap-2">
                        <button
                          onClick={() => handleToggleActive(user)}
                          disabled={actionId === user.id}
                          title={user.is_active ? 'Nonaktifkan' : 'Aktifkan'}
                          className={`p-1.5 rounded-lg transition-colors disabled:opacity-50 ${
                            user.is_active
                              ? 'text-orange-500 hover:bg-orange-50'
                              : 'text-green-600 hover:bg-green-50'
                          }`}
                        >
                          {user.is_active ? <UserX size={16} /> : <UserCheck size={16} />}
                        </button>
                        <button
                          onClick={() => handleDelete(user)}
                          disabled={actionId === user.id}
                          title="Hapus user"
                          className="p-1.5 rounded-lg text-red-500 hover:bg-red-50 transition-colors disabled:opacity-50"
                        >
                          <Trash2 size={16} />
                        </button>
                        {actionId === user.id && (
                          <Loader2 size={14} className="animate-spin text-gray-400" />
                        )}
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </main>
    </>
  )
}
