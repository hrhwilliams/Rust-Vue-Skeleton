import type { DiscordUser } from '@/types/discord_user'

export async function getInfo(): Promise<DiscordUser | null> {
  const res = await fetch("/api/auth/me")
  if (res.ok) {
    return res.json()
  }

  if (res.status === 401 || res.status === 403) {
    return null
  }

  throw new Error(`Failed to fetch user info: ${res.status}`)
}