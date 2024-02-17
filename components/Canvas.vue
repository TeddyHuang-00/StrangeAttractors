<template>
  <Suspense>
    <TresCanvas
      window-size
      preset="realistic"
      class="absolute left-0 top-0"
      clear-color="#0f0f0f"
    >
      <Camera />
      <Axes />
      <TresPoints>
        <TresBufferGeometry :position="[rawCoords, 3]" :color="[colors, 3]" />
        <TresPointsMaterial
          :size="pointSize"
          :vertexColors="true"
          :map="map"
          :transparent="true"
          :alpha-test="0.5"
        />
      </TresPoints>
    </TresCanvas>
  </Suspense>
</template>

<script setup lang="ts">
const { value } = useControls("fpsgraph");
const timeSpeed = useTimeSpeed();
const initRange = useInitialRange();
const pointNumber = usePointNumber();
const pointSize = usePointSize();
const attrctrSelection = useAttractorSelection();

const colors = shallowRef(Float32Array.from([]) as Float32Array);
const rawCoords = shallowRef(Float32Array.from([]) as Float32Array);

const { map } = await useTexture({
  map: "textures/disc.png",
});

import init, { Attractor } from "attractors";
onMounted(async () => {
  await init();

  const attractor = Attractor.new();
  const getRawCoords = () => attractor.points() as unknown as Float32Array;
  const getColors = () => attractor.colors() as unknown as Float32Array;

  // Initialize the attractor points
  attractor.init_points(pointNumber.value, initRange.value);
  rawCoords.value = getRawCoords();
  attractor.init_colors(pointNumber.value);
  colors.value = getColors();
  watchDebounced(
    pointNumber,
    () => {
      attractor.init_points(pointNumber.value, initRange.value);
      rawCoords.value = getRawCoords();
      attractor.init_colors(pointNumber.value);
      colors.value = getColors();
    },
    { debounce: 500, maxWait: 1000 },
  );
  onKeyStroke("r", () => {
    attractor.init_points(pointNumber.value, initRange.value);
    rawCoords.value = getRawCoords();
  });

  // Initialize the attractor system
  attractor.set_system(attrctrSelection.value);
  watch(attrctrSelection, () => {
    attractor.set_system(attrctrSelection.value);
    attractor.init_points(pointNumber.value, initRange.value);
    rawCoords.value = getRawCoords();
  });

  // Main animation loop
  const { onLoop } = useRenderLoop();
  const [isPaused, togglePause] = useToggle(false);
  onKeyStroke(" ", () => togglePause());
  onLoop(({ delta, elapsed, clock }) => {
    if (isPaused.value) return;
    attractor.step(timeSpeed.value * delta);
    rawCoords.value = getRawCoords();
  });
});
</script>

<style scoped></style>
