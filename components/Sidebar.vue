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
          <div class="flex flex-row items-center gap-3 mb-2">
            <UAvatar src="https://github.com/TeddyHuang-00.png" alt="Avatar" />
            <b>TeddyHuang-00</b>
          </div>
          <p>
            Strange Attractors is a tiny project to visualize the strange
            attractors in 3D space.
          </p>
          <p>
            If you got interested, please give it a star on
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
            . Or check out my blog at
            <ULink
              to="https://teddyhuang-00.github.io/"
              target="_blank"
              active-class="text-white"
              inactive-class="text-pink-500 hover:text-pink-200"
              class="whitespace-nowrap"
            >
              <Icon name="bx:bx-link-external" size="16" class="mb-1" />
              teddyhuang-00.github.io
            </ULink>
            .
          </p>
        </template>

        <template class="flex flex-col space-y-4">
          <URadioGroup
            v-model="attrctrSelection"
            :options="attractors"
            color="gray"
          >
            <template #legend>
              <p class="text-base">Choose an attractor</p>
            </template>
          </URadioGroup>
          <UTooltip
            text="Time step size used in solver"
            :popper="{ placement: 'right', arrow: true }"
          >
            Timescale:
            <UBadge color="gray" variant="solid" class="mx-1">
              {{ timeSpeed }}
            </UBadge>
          </UTooltip>
          <URange
            :min="0.0"
            :max="1.0"
            v-model="timeSpeed"
            :step="0.01"
            color="gray"
          />
          <UTooltip
            text="Number of points in the simulation"
            :popper="{ placement: 'right', arrow: true }"
          >
            Point number:
            <UBadge color="gray" variant="solid" class="mx-1">
              {{ pointNumber }}
            </UBadge>
          </UTooltip>
          <URange
            :min="10"
            :max="10000"
            v-model="pointNumber"
            :step="10"
            color="gray"
          />
          <UTooltip
            text="Sphere radius for each point"
            :popper="{ placement: 'right', arrow: true }"
          >
            Point size:
            <UBadge color="gray" variant="solid" class="mx-1">
              {{ pointSize }}
            </UBadge>
          </UTooltip>
          <URange
            :min="0.01"
            :max="1"
            v-model="pointSize"
            :step="0.01"
            color="gray"
          />
          <UTooltip
            text="Number of segments in the sphere for each point"
            :popper="{ placement: 'right', arrow: true }"
          >
            Detail level:
            <UBadge color="gray" variant="solid" class="mx-1">
              {{ detailLevel }}
            </UBadge>
          </UTooltip>
          <URange
            :min="3"
            :max="16"
            v-model="detailLevel"
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
        </template>

        <template #footer>
          <div class="flex flex-col gap-1">
            <TresLeches />
            <p class="text-sm text-gray-400">
              Built with Nuxt and TresJS. Shared under MIT license.
            </p>
          </div>
        </template>
      </UCard>
    </USlideover>
  </div>
</template>

<script setup lang="ts">
const isOpen = ref(false);

const attractors: { value: nameAttractor; label: string }[] = [
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
    value: "luchen",
    label: "Lu Chen",
  },
  {
    value: "dequanli",
    label: "Dequan Li",
  },
  {
    value: "newton",
    label: "Newton-Leipnik",
  },
  {
    value: "nose",
    label: "Nosé-Hoover",
  },
];
const attrctrSelection = useAttractorSelection();

const timeSpeed = useTimeSpeed();
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
