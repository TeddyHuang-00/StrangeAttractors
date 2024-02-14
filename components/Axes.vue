<template>
  <TresMesh :position="[0, 0, 0]">
    <TresSphereGeometry :args="[0.5, 16, 16]" />
    <TresMeshToonMaterial color="white" />
  </TresMesh>
  <TresAxesHelper :args="[length]" />
  <TresMesh :position="[size / 2, size / 2, 0]">
    <TresBoxGeometry :args="[size, size, 0.01]" />
    <TresMeshBasicMaterial color="red" :opacity="opacity" :transparent="true" />
  </TresMesh>
  <TresMesh :position="[size / 2, 0, size / 2]">
    <TresBoxGeometry :args="[size, 0.01, size]" />
    <TresMeshBasicMaterial
      color="navy"
      :opacity="opacity"
      :transparent="true"
    />
  </TresMesh>
  <TresMesh :position="[0, size / 2, size / 2]">
    <TresBoxGeometry :args="[0.01, size, size]" />
    <TresMeshBasicMaterial
      color="green"
      :opacity="opacity"
      :transparent="true"
    />
  </TresMesh>
</template>

<script setup lang="ts">
const { length, size } = defineProps({
  length: {
    type: Number,
    default: 10,
  },
  size: {
    type: Number,
    default: 2,
  },
  opacity: {
    type: Number,
    default: 0.75,
  },
});
</script>

<style scoped></style>
