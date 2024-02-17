<template>
  <div>
    <ClientOnly>
      <Canvas />
    </ClientOnly>
    <Sidebar />
  </div>
</template>

<script setup lang="ts">
defineOgImage({
  component: "Gradient",
  props: {
    title: "Strange Attractors",
    description: "A collection & visualization of strange attractors",
    headline: "TeddyHuang-00",
  },
});
useSeoMeta({
  title: "Strange Attractors",
  ogTitle: "Strange Attractors",
  description: "A collection & visualization of strange attractors",
  ogDescription: "A collection & visualization of strange attractors",
});
</script>
