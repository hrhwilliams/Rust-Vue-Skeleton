import type { ApiEvent } from '@/types/event'

export async function fetchEvents(): Promise<ApiEvent[]> {
  const res = await fetch('/api/events')
  if (!res.ok) throw new Error(`Failed to load events: ${res.status}`)
  return res.json()
}

export async function fetchEvent(id: string): Promise<ApiEvent> {
  const res = await fetch(`/api/event/${id}`)
  if (!res.ok) throw new Error(`Failed to load event ${id}`)
  return res.json()
}
