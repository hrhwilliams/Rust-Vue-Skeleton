import type { ApiGroup } from '@/types/group'

export async function getGroup(id: string): Promise<ApiGroup> {
  const res = await fetch(`/api/group/${id}`)
  if (!res.ok) throw new Error(`Failed to load events: ${res.status}`)
  return res.json()
}