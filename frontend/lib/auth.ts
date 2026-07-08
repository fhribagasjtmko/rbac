import api, { clearTokens, setTokens } from './api'
import type { AuthResponse, LoginRequest, RegisterRequest, User } from '@/types'

export const authService = {
  async login(data: LoginRequest): Promise<AuthResponse> {
    const res = await api.post<AuthResponse>('/api/v1/auth/login', data)
    const { access_token, refresh_token } = res.data
    setTokens(access_token, refresh_token)
    return res.data
  },

  async register(data: RegisterRequest): Promise<{ success: boolean; message: string; user: User }> {
    const res = await api.post('/api/v1/auth/register', data)
    return res.data
  },

  async logout(): Promise<void> {
    try {
      await api.post('/api/v1/auth/logout')
    } finally {
      clearTokens()
    }
  },

  async getMe(): Promise<User> {
    const res = await api.get<{ success: boolean; user: User }>('/api/v1/users/me')
    return res.data.user
  },
}
