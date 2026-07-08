import api from './api'
import type { User } from '@/types'

export const adminService = {
  async listUsers(): Promise<{ total: number; users: User[] }> {
    const res = await api.get('/api/v1/admin/users')
    return res.data
  },

  async updateRole(userId: string, role: string): Promise<User> {
    const res = await api.patch(`/api/v1/admin/users/${userId}/role`, { role })
    return res.data.user
  },

  async deactivateUser(userId: string): Promise<User> {
    const res = await api.patch(`/api/v1/admin/users/${userId}/deactivate`)
    return res.data.user
  },

  async activateUser(userId: string): Promise<User> {
    const res = await api.patch(`/api/v1/admin/users/${userId}/activate`)
    return res.data.user
  },

  async deleteUser(userId: string): Promise<void> {
    await api.delete(`/api/v1/admin/users/${userId}`)
  },
}
