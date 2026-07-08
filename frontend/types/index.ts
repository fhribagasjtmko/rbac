// ─── User ─────────────────────────────────────────────────────────────────────

export interface User {
  id: string
  email: string
  name: string
  role: 'user' | 'admin' | 'moderator'
  is_active: boolean
  created_at: string
}

// ─── Auth ─────────────────────────────────────────────────────────────────────

export interface AuthResponse {
  success: boolean
  access_token: string
  refresh_token: string
  user: User
}

export interface LoginRequest {
  email: string
  password: string
}

export interface RegisterRequest {
  name: string
  email: string
  password: string
}

// ─── API Response wrapper ─────────────────────────────────────────────────────

export interface ApiResponse<T = unknown> {
  success: boolean
  message?: string
  data?: T
}

export interface ApiError {
  success: false
  message: string
  status: number
}
