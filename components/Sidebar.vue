<template>
  <div class="absolute right-0">
    <Icon
      name="material-symbols:chevron-left-rounded"
      size="64"
      class="opacity-75 hover:opacity-100 hover:cursor-pointer mt-[50vh] -translate-y-1/2"
      @click="isOpen = true"
    />

    <USlideover v-model="isOpen">
      <UCard
        class="flex flex-col flex-1"
        :ui="{
          body: { base: 'flex-1' },
          ring: '',
          divide: 'divide-y divide-gray-100 dark:divide-gray-800',
        }"
      >
        <template #header>
          <Placeholder class="h-8" />
        </template>

        <template class="flex flex-col space-y-4">
          <UTooltip
            text="Time step size used in solver"
            :popper="{ placement: 'right', arrow: true }"
          >
            <UBadge color="gray" variant="solid">
              Timescale: {{ timeScale }}
            </UBadge>
          </UTooltip>
          <URange
            :min="0.0"
            :max="1.0"
            v-model="timeScale"
            :step="0.01"
            color="gray"
          />
          <UTooltip
            text="Number of points in the simulation"
            :popper="{ placement: 'right', arrow: true }"
          >
            <UBadge color="gray" variant="solid">
              Point number: {{ numPoints }}
            </UBadge>
          </UTooltip>
          <URange
            :min="10"
            :max="10000"
            v-model="numPoints"
            :step="10"
            color="gray"
          />
          <UTooltip
            text="Sphere radius for each point"
            :popper="{ placement: 'right', arrow: true }"
          >
            <UBadge color="gray" variant="solid">
              Point size: {{ sizePoint }}
            </UBadge>
          </UTooltip>
          <URange
            :min="0.01"
            :max="1"
            v-model="sizePoint"
            :step="0.01"
            color="gray"
          />
          <UTooltip
            text="Number of segments in the sphere for each point"
            :popper="{ placement: 'right', arrow: true }"
          >
            <UBadge color="gray" variant="solid">
              Detail level: {{ levelDetail }}
            </UBadge>
          </UTooltip>
          <URange
            :min="3"
            :max="16"
            v-model="levelDetail"
            :step="1"
            color="gray"
          />

          <UDivider>
            <Icon name="material-symbols:keyboard" size="24" />
          </UDivider>

          <UTable :rows="keyBindings">
            <template #key-data="{ row }">
              <UKbd>{{ row.key }}</UKbd>
            </template>
          </UTable>

          <Placeholder class="h-full" />
        </template>

        <template #footer>
          <TresLeches class="w-fit mx-auto" />
        </template>
      </UCard>
    </USlideover>
  </div>
</template>

<script setup lang="ts">
const isOpen = ref(false);
const timeScale = useState("timeScale", () => 0.2);
const numPoints = useState("numPoints", () => 500);
const sizePoint = useState("sizePoint", () => 0.1);
const levelDetail = useState("levelDetail", () => 6);

onKeyStroke("Enter", () => (isOpen.value = !isOpen.value));
const keyBindings = [
  {
    key: "Enter",
    description: "Toggle sidebar",
  },
  {
    key: "Space",
    description: "Pause/Resume simulation",
  },
  {
    key: "R",
    description: "Reset simulation",
  },
  {
    key: "H",
    description: "Toggle axes",
  },
];
</script>

<style scoped></style>
