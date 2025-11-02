<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'  

const counter = ref<{ counter: number }>({ counter: 0 })

function read_counter() {
  fetch("http://localhost:3000/counter")
    .then(response => response.json())
    .then((data: any) => {
      if (data && typeof data.counter === 'number') counter.value = data
    })
    .catch(() => {})
}

function increment() {
  fetch("http://localhost:3000/counter", {
    method: "POST",
    headers: { "Content-Type": "application/json" }
  }).catch(() => {})
}

let pollTimer: number | null = null

onMounted(() => {
  read_counter()
  pollTimer = window.setInterval(read_counter, 1000)
})

onBeforeUnmount(() => {
  if (pollTimer !== null) {
    window.clearInterval(pollTimer)
    pollTimer = null
  }
})
</script>

<template>
  <h1>You did it!</h1>
  <p>
    Visit <a href="https://vuejs.org/" target="_blank" rel="noopener">vuejs.org</a> to read the
    documentation
  </p>
  <p>The counter is at: {{ counter.counter }}</p>
  <input type="button" @click="increment" value="button" />
</template>

<style scoped></style>
