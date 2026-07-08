import axios, { AxiosError, InternalAxiosRequestConfig } from 'axios'
import Cookies from 'js-cookie'

// ─── Token keys ───────────────────────────────────────────────────────────────

export const ACCESS_TOKEN_KEY  = 'access_token'
export const REFRESH_TOKEN_KEY = 'refresh_token'

export const getAccessToken  = () => Cookies.get(ACCESS_TOKEN_KEY)
export const getRefreshToken = () => Cookies.get(REFRESH_TOKEN_KEY)

export const setTokens = (access: string, refresh: string) => {
  // access token: session (hilang kalau tab ditutup)
  Cookies.set(ACCESS_TOKEN_KEY, access, { sameSite: 'strict' })
  // refresh token: 7 hari
  Cookies.set(REFRESH_TOKEN_KEY, refresh, { expires: 7, sameSite: 'strict' })
}

export const clearTokens = () => {
  Cookies.remove(ACCESS_TOKEN_KEY)
  Cookies.remove(REFRESH_TOKEN_KEY)
}

// ─── Axios instance ───────────────────────────────────────────────────────────

const api = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080',
  headers: { 'Content-Type': 'application/json' },
  timeout: 10_000,
})

// ─── Request interceptor: inject Bearer token ─────────────────────────────────

api.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  const token = getAccessToken()
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// ─── Response interceptor: auto refresh kalau 401 ────────────────────────────

let isRefreshing = false
let failedQueue: Array<{
  resolve: (token: string) => void
  reject:  (err: unknown) => void
}> = []

const processQueue = (error: unknown, token: string | null = null) => {
  failedQueue.forEach(({ resolve, reject }) => {
    if (error) reject(error)
    else resolve(token!)
  })
  failedQueue = []
}

api.interceptors.response.use(
  (res) => res,
  async (error: AxiosError) => {
    const original = error.config as InternalAxiosRequestConfig & { _retry?: boolean }

    // Kalau 401 dan belum di-retry
    if (error.response?.status === 401 && !original._retry) {
      const refreshToken = getRefreshToken()

      // Kalau tidak ada refresh token → langsung logout
      if (!refreshToken) {
        clearTokens()
        window.location.href = '/auth/login'
        return Promise.reject(error)
      }

      if (isRefreshing) {
        // Kalau sedang refresh, queue request ini
        return new Promise((resolve, reject) => {
          failedQueue.push({ resolve, reject })
        }).then((token) => {
          original.headers.Authorization = `Bearer ${token}`
          return api(original)
        })
      }

      original._retry = true
      isRefreshing = true

      try {
        const { data } = await axios.post(
          `${process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080'}/api/v1/auth/refresh`,
          { refresh_token: refreshToken }
        )

        const newAccessToken = data.access_token
        Cookies.set(ACCESS_TOKEN_KEY, newAccessToken, { sameSite: 'strict' })

        api.defaults.headers.common.Authorization = `Bearer ${newAccessToken}`
        original.headers.Authorization = `Bearer ${newAccessToken}`

        processQueue(null, newAccessToken)
        return api(original)
      } catch (refreshError) {
        processQueue(refreshError, null)
        clearTokens()
        window.location.href = '/auth/login'
        return Promise.reject(refreshError)
      } finally {
        isRefreshing = false
      }
    }

    return Promise.reject(error)
  }
)

export default api
