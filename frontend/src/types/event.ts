export interface ApiEvent {
  vrc_event_id: string
  vrc_group_id: string
  name: string
  description: string
  starts_at: string
  ends_at: string
  category: string,
  access_type: string,
  platforms: string[],
  image_url?: string,
  tags?: string[],
  created_at: string
}