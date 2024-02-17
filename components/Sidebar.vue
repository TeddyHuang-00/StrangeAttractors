<template>
  <div class="absolute right-0">
    <Icon
      name="material-symbols:chevron-left-rounded"
      size="64"
      class="mt-[50vh] -translate-y-1/2 opacity-75 hover:cursor-pointer hover:opacity-100"
      @click="isOpen = true"
    />

    <USlideover v-model="isOpen" class="h-screen overflow-y-scroll">
      <UCard
        class="flex flex-1 flex-col"
        :ui="{
          body: { base: 'flex-1' },
          ring: '',
          divide: 'divide-y divide-gray-100 dark:divide-gray-800',
        }"
      >
        <template #header>
          <div class="mb-2 flex flex-row items-center gap-3">
            <UAvatar src="https://github.com/TeddyHuang-00.png" alt="Avatar" />
            <b>TeddyHuang-00</b>
            <span class="flex-grow"></span>
            <UButton class="p-1" @click="isOpen = false">
              <Icon
                name="material-symbols:close-rounded"
                size="24"
                class="m-0"
              />
            </UButton>
          </div>
          <p>
            Strange Attractors is a tiny project to visualize the strange
            attractors in 3D space.
          </p>
          <p>
            Please consider giving it a star on
            <ULink
              to="https://github.com/TeddyHuang-00/StrangeAttractors"
              target="_blank"
              active-class="text-white"
              inactive-class="text-violet-500 hover:text-violet-200"
              class="whitespace-nowrap"
            >
              <Icon name="grommet-icons:github" size="16" class="mb-1" />
              GitHub
            </ULink>
            .<br />
            Or, check out my blog at
            <ULink
              to="https://teddyhuang-00.github.io/"
              target="_blank"
              active-class="text-white"
              inactive-class="text-pink-500 hover:text-pink-200"
              class="whitespace-nowrap"
            >
              <Icon name="bx:bx-link-external" size="16" class="mb-0.5" />
              teddyhuang-00.github.io
            </ULink>
            .
          </p>
        </template>

        <template class="flex flex-col space-y-4">
          <p>Choose an attractor</p>
          <div class="grid grid-cols-2 gap-1 space-y-1">
            <URadio
              v-for="att of attractors"
              :key="att.value"
              v-model="attrctrSelection"
              v-bind="att"
            />
          </div>
          <!-- <URadioGroup v-model="attrctrSelection" :options="attractors" :ui="{
            wrapper: 'relative grid grid-cols-2 gap-2 items-start',
          }">
            <template #legend>
              <p class="text-base">Choose an attractor</p>
            </template>
          </URadioGroup> -->
          <UTooltip
            text="Time step size used in solver"
            :popper="{ placement: 'right', arrow: true }"
          >
            Time speed:
            <UBadge variant="solid" class="mx-1">
              {{ timeSpeed }}
            </UBadge>
          </UTooltip>
          <URange :min="0.0" :max="1.0" v-model="timeSpeed" :step="0.01" />
          <UTooltip
            text="Range of uniform distribution of initial points"
            :popper="{ placement: 'right', arrow: true }"
          >
            Initial range:
            <UBadge variant="solid" class="mx-1">
              {{ initRange }}
            </UBadge>
          </UTooltip>
          <URange :min="0.1" :max="10" v-model="initRange" :step="0.1" />
          <UTooltip
            text="Number of points in the simulation"
            :popper="{ placement: 'right', arrow: true }"
          >
            Point number:
            <UBadge variant="solid" class="mx-1">
              {{ pointNumber }}
            </UBadge>
          </UTooltip>
          <URange :min="10" :max="10000" v-model="pointNumber" :step="10" />
          <UTooltip
            text="Sphere radius for each point"
            :popper="{ placement: 'right', arrow: true }"
          >
            Point size:
            <UBadge variant="solid" class="mx-1">
              {{ pointSize }}
            </UBadge>
          </UTooltip>
          <URange :min="0.01" :max="1" v-model="pointSize" :step="0.01" />
          <UTooltip
            text="Number of segments in the sphere for each point"
            :popper="{ placement: 'right', arrow: true }"
          >
            Detail level:
            <UBadge variant="solid" class="mx-1">
              {{ detailLevel }}
            </UBadge>
          </UTooltip>
          <URange :min="3" :max="16" v-model="detailLevel" :step="1" />

          <UDivider>
            <Icon name="material-symbols:keyboard" size="24" />
          </UDivider>

          <UTable :rows="keyBindings">
            <template #key-data="{ row }">
              <UKbd>{{ row.key }}</UKbd>
            </template>
          </UTable>
        </template>

        <template #footer>
          <div class="flex flex-col gap-1">
            <TresLeches />
            <p class="text-sm text-gray-400">
              Built with Nuxt, TresJS and WASM. Shared under MIT license.
            </p>
          </div>
        </template>
      </UCard>
    </USlideover>
  </div>
</template>

<script setup lang="ts">
import { useInitialRange } from "~/composables/useStates";

const isOpen = ref(false);

const attractors = useSorted(
  [
    {
      value: "lorenz",
      label: "Lorenz",
    },
    {
      value: "rossler",
      label: "Rössler",
    },
    {
      value: "thomas",
      label: "Thomas",
    },
    {
      value: "lu_chen",
      label: "Lu Chen",
    },
    {
      value: "dequan_li",
      label: "Dequan Li",
    },
    {
      value: "newton_leipnik",
      label: "Newton-Leipnik",
    },
    {
      value: "nose_hoover",
      label: "Nosé-Hoover",
    },
    {
      value: "halvorsen",
      label: "Halvorsen",
    },
    {
      value: "chen_lee",
      label: "Chen-Lee",
    },
    {
      value: "bouali",
      label: "Bouali",
    },
    {
      value: "finance",
      label: "Finance",
    },
    {
      value: "arneodo",
      label: "Arneodo",
    },
    {
      value: "sprott_b",
      label: "Sprott B",
    },
    {
      value: "sprott_linz_f",
      label: "Sprott-Linz F",
    },
    {
      value: "dadras",
      label: "Dadras",
    },
  ] as {
    value: nameAttractor;
    label: string;
  }[],
  (a, b) => a.label.localeCompare(b.label),
);
const attrctrSelection = useAttractorSelection();

const timeSpeed = useTimeSpeed();
const initRange = useInitialRange();
const pointNumber = usePointNumber();
const pointSize = usePointSize();
const detailLevel = useDetailLevel();

const keyBindings = [
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
