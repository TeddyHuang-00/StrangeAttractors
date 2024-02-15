<template>
  <TresCanvas
    window-size
    preset="realistic"
    class="absolute top-0 left-0"
    clear-color="#0f0f0f"
  >
    <Camera />
    <Axes />
    <TresMesh :position="coord" v-for="(coord, i) in coords">
      <TresSphereGeometry :args="[pointSize, detailLevel, detailLevel]" />
      <TresMeshToonMaterial :color="colors[i]" />
    </TresMesh>
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
const coords = ref([] as Vec3D[]);
const initColor = () =>
  (colors.value = randomArray([pointNumber.value], () =>
    randomColor(),
  ) as string[]);
const initCoords = () =>
  (coords.value = randomArray([pointNumber.value, 3], () =>
    randomNumber(-initRange.value, initRange.value),
  ) as Vec3D[]);
onMounted(() => {
  initColor();
  initCoords();
});
watchDebounced(
  pointNumber,
  () => {
    initCoords();
    initColor();
  },
  { debounce: 500, maxWait: 1000 },
);
onKeyStroke("r", initCoords);

let attractor = getAttractor(attrctrSelection.value);
watch(attrctrSelection, () => {
  attractor = getAttractor(attrctrSelection.value);
  initCoords();
});

const { onLoop } = useRenderLoop();
const [isPaused, togglePause] = useToggle(false);
onKeyStroke(" ", () => togglePause());
onLoop(({ delta, elapsed, clock }) => {
  if (isPaused.value) return;
  coords.value.forEach((coord, i) => {
    coords.value[i] = RK4(coord, attractor, delta * timeSpeed.value);
  });
});
</script>

<style scoped></style>
