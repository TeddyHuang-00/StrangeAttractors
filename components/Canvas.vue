<template>
  <TresCanvas
    window-size
    preset="realistic"
    class="absolute left-0 top-0"
    clear-color="#0f0f0f"
  >
    <Camera />
    <Axes />
    <TresGroup v-if="rawCoords.length > 0">
      <TresMesh :position="convertCoords(i) as Vec3D" v-for="i in indices">
        <TresSphereGeometry :args="[pointSize, detailLevel, detailLevel]" />
        <TresMeshToonMaterial :color="colors[i]" />
      </TresMesh>
    </TresGroup>
  </TresCanvas>
</template>

<script setup lang="ts">
const { value } = useControls("fpsgraph");
const timeSpeed = useTimeSpeed();
const initRange = useInitialRange();
const pointNumber = usePointNumber();
const pointSize = usePointSize();
const detailLevel = useDetailLevel();
const attrctrSelection = useAttractorSelection();

const colors = ref([] as string[]);
const rawCoords = ref(Float64Array.from([]) as Float64Array);
const indices = ref(Array.from(Array(pointNumber.value).keys()));

const convertCoords = (i: number) => {
  return [
    rawCoords.value.at(3 * i),
    rawCoords.value.at(3 * i + 1),
    rawCoords.value.at(3 * i + 2),
  ];
};

import init, { Attractor } from "attractors";
onMounted(async () => {
  await init();

  const attractor = Attractor.new();
  const getRawCoords = () => attractor.points() as unknown as Float64Array;
  const getCoords = () => indices.value.map(convertCoords);
  const getColors = () => attractor.colors() as unknown as string[];

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
      indices.value = Array.from(Array(pointNumber.value).keys());
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
