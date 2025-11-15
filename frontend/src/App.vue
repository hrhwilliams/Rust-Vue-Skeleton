<script lang="ts">
import { defineComponent, ref, onMounted, onBeforeUnmount } from 'vue'
import FullCalendar from '@fullcalendar/vue3'
import dayGridPlugin from '@fullcalendar/daygrid'
import interactionPlugin from '@fullcalendar/interaction'
import type { CalendarOptions, EventInput, EventClickArg } from '@fullcalendar/core'

import { fetchEvents } from '@/api/events'
import type { ApiEvent } from '@/types/event'

function toCalendarEvents(events: ApiEvent[]): EventInput[] {
  return events.map(evt => ({
    id: evt.id,
    title: evt.name,
    start: evt.starts_at,
    end: evt.ends_at,
    extendedProps: {
      description: evt.description,
      groupId: evt.group_id,
    },
  }))
}

export default defineComponent({
  name: 'App',
  components: { FullCalendar },
  setup() {
    const activeEvent = ref<{ id: string; title: string; description: string } | null>(null)
    const isPopoverOpen = ref(false)
    const popoverPosition = ref<{ top: number; left: number } | null>(null)
    const popoverRef = ref<HTMLElement | null>(null)

    function handleEventClick(info: EventClickArg) {
      info.jsEvent?.preventDefault()
      info.jsEvent?.stopPropagation()
      const props = info.event.extendedProps as { description?: string } | undefined
      activeEvent.value = {
        id: info.event.id,
        title: info.event.title || 'Untitled event',
        description: props?.description ?? '',
      }
      const pageX = info.jsEvent?.pageX ?? 0
      const pageY = info.jsEvent?.pageY ?? 0
      popoverPosition.value = {
        top: pageY + 8,
        left: pageX + 8,
      }
      isPopoverOpen.value = true
    }

    const calendarOptions = ref<CalendarOptions>({
      plugins: [dayGridPlugin, interactionPlugin],
      initialView: 'dayGridMonth',
      events: [] as EventInput[],
      eventClick: handleEventClick,
    })
    const isLoading = ref(false)

    async function loadEvents() {
      isLoading.value = true
      try {
        const data = await fetchEvents()
        calendarOptions.value.events = toCalendarEvents(data)
      } catch (err) {
        console.error('Failed to load events', err)
      } finally {
        isLoading.value = false
      }
    }

    onMounted(loadEvents)

    function closePopover() {
      isPopoverOpen.value = false
      activeEvent.value = null
      popoverPosition.value = null
    }

    function handleDocumentClick(event: MouseEvent) {
      if (!isPopoverOpen.value) return
      const popoverEl = popoverRef.value
      if (popoverEl && popoverEl.contains(event.target as Node)) return
      closePopover()
    }

    onMounted(() => document.addEventListener('click', handleDocumentClick))
    onBeforeUnmount(() => document.removeEventListener('click', handleDocumentClick))

    return {
      calendarOptions,
      isLoading,
      isPopoverOpen,
      activeEvent,
      popoverPosition,
      popoverRef,
      closePopover,
    }
  },
})
</script>

<template>
  <section>
    <FullCalendar :options="calendarOptions" />

    <div
      v-if="isPopoverOpen && activeEvent && popoverPosition"
      class="event-popover"
      :style="{ top: `${popoverPosition.top}px`, left: `${popoverPosition.left}px` }"
      ref="popoverRef"
    >
      <header>
        <strong>{{ activeEvent.title }}</strong>
        <button type="button" class="close-btn" @click="closePopover">×</button>
      </header>
      <p>{{ activeEvent.id }}</p>
      <p>{{ activeEvent.description || 'No description provided.' }}</p>
    </div>
  </section>
</template>

<style scoped>
.event-popover {
  position: absolute;
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 0.5rem;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  padding: 0.75rem 1rem;
  max-width: 16rem;
  z-index: 1100;
}

.event-popover header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.close-btn {
  border: none;
  background: transparent;
  font-size: 1.1rem;
  cursor: pointer;
}
</style>