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
      <TresSphereGeometry :args="[sizePoint, levelDetail, levelDetail]" />
      <TresMeshToonMaterial :color="colors[i]" />
    </TresMesh>
  </TresCanvas>
</template>

<script setup lang="ts">
const { value } = useControls("fpsgraph");
const timeScale = useState("timeScale", () => 0.2);
const numPoints = useState("numPoints", () => 500);
const sizePoint = useState("sizePoint", () => 0.1);
const levelDetail = useState("levelDetail", () => 6);
const selectedAttractor = useState(
  "choiceAttractor",
  () => "lorenz" as nameAttractor,
);

const colors = ref([] as string[]);
const coords = ref([] as Vec3D[]);
const initColor = () =>
  (colors.value = randomArray([numPoints.value], () =>
    randomColor(),
  ) as string[]);
const initCoords = () =>
  (coords.value = randomArray([numPoints.value, 3], () =>
    randomNumber(-25, 25),
  ) as Vec3D[]);
onMounted(() => {
  initColor();
  initCoords();
});
watchDebounced(
  numPoints,
  () => {
    initCoords();
    initColor();
  },
  { debounce: 500, maxWait: 1000 },
);
onKeyStroke("r", initCoords);

let attractor = getAttractor(selectedAttractor.value);
watch(selectedAttractor, () => {
  attractor = getAttractor(selectedAttractor.value);
  initCoords();
});

const { onLoop } = useRenderLoop();
const [isPaused, togglePause] = useToggle(false);
onKeyStroke(" ", () => togglePause());
onLoop(({ delta, elapsed, clock }) => {
  if (isPaused.value) return;
  coords.value = coords.value.map((coord) =>
    RK4(coord, attractor, delta * timeScale.value),
  );
});
</script>

<style scoped></style>
