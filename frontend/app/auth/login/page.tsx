'use client'

import { Suspense, useState, FormEvent } from 'react'
import Link from 'next/link'
import { useRouter, useSearchParams } from 'next/navigation'
import toast from 'react-hot-toast'
import { Eye, EyeOff, Loader2 } from 'lucide-react'
import { authService } from '@/lib/auth'
import { useAuth } from '@/hooks/useAuth'

function LoginForm() {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [showPass, setShowPass] = useState(false)
  const [loading, setLoading] = useState(false)

  const router = useRouter()
  const searchParams = useSearchParams()
  const { refreshUser } = useAuth()

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault()

    if (!email || !password) {
      toast.error('Isi semua field dulu')
      return
    }

    setLoading(true)

    try {
      await authService.login({
        email,
        password,
      })

      await refreshUser()

      toast.success('Login berhasil!')

      const redirect =
        searchParams.get('redirect') || '/dashboard'

      router.push(redirect)
    } catch (err: any) {
      const msg =
        err?.response?.data?.message ||
        'Email atau password salah'

      toast.error(msg)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 px-4">
      <div className="w-full max-w-md">

        <div className="text-center mb-8">
          <div className="inline-flex items-center justify-center w-12 h-12 bg-blue-600 rounded-xl mb-4">
            <span className="text-white font-bold text-xl">
              M
            </span>
          </div>

          <h1 className="text-3xl font-bold text-gray-900">
            Masuk ke akun
          </h1>

          <p className="text-gray-500 mt-2">
            Selamat datang kembali
          </p>
        </div>

        <div className="bg-white rounded-xl shadow border p-6">

          <form
            onSubmit={handleSubmit}
            className="space-y-4"
          >
            <div>
              <label className="block text-sm font-medium mb-2">
                Email
              </label>

              <input
                type="email"
                value={email}
                onChange={(e) =>
                  setEmail(e.target.value)
                }
                placeholder="kamu@example.com"
                className="w-full border rounded-lg px-4 py-3 focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium mb-2">
                Password
              </label>

              <div className="relative">
                <input
                  type={showPass ? 'text' : 'password'}
                  value={password}
                  onChange={(e) =>
                    setPassword(e.target.value)
                  }
                  placeholder="Masukkan password"
                  className="w-full border rounded-lg px-4 py-3 pr-12 focus:outline-none focus:ring-2 focus:ring-blue-500"
                />

                <button
                  type="button"
                  onClick={() =>
                    setShowPass(!showPass)
                  }
                  className="absolute right-3 top-1/2 -translate-y-1/2"
                >
                  {showPass ? (
                    <EyeOff size={18} />
                  ) : (
                    <Eye size={18} />
                  )}
                </button>
              </div>
            </div>

            <button
              type="submit"
              disabled={loading}
              className="w-full bg-blue-600 text-white py-3 rounded-lg hover:bg-blue-700 transition disabled:opacity-50"
            >
              {loading ? (
                <span className="flex items-center justify-center gap-2">
                  <Loader2
                    size={18}
                    className="animate-spin"
                  />
                  Login...
                </span>
              ) : (
                'Login'
              )}
            </button>
          </form>
        </div>

        <p className="text-center text-sm text-gray-500 mt-4">
          Belum punya akun?{' '}
          <Link
            href="/auth/register"
            className="text-blue-600 font-medium hover:underline"
          >
            Daftar sekarang
          </Link>
        </p>
      </div>
    </div>
  )
}

export default function LoginPage() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <LoginForm />
    </Suspense>
  )
}