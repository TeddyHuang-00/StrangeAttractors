<template>
  <TresPerspectiveCamera
    :args="[45, 1, 0.1, 1000]"
    :position="[50, 50, 50]"
    :look-at="[0, 0, 0]"
  />
  <OrbitControls />
  <TresAmbientLight v-light-helper />
  <!-- <TresDirectionalLight v-light-helper /> -->
</template>

<script setup lang="ts"></script>

<style scoped></style>
