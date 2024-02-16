<template>
  <div class="flex h-full w-full flex-col justify-center bg-[#1e1e2e]">
    <svg
      class="absolute right-0 top-0"
      width="1200"
      height="675"
      viewBox="0 0 1200 675"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <g
        style="mix-blend-mode: overlay"
        opacity="0.7"
        filter="url(#filter0_f_448_25)"
      >
        <circle cx="901.5" cy="45.5" r="199.5" fill="#b4befe" />
        <circle cx="600.5" cy="216.5" r="199.5" fill="#89b4fa" />
        <circle cx="179.5" cy="317.5" r="199.5" fill="#74c7ec" />
      </g>
      <defs>
        <filter
          id="filter0_f_448_25"
          x="-240"
          y="-374"
          width="1561"
          height="1111"
          filterUnits="userSpaceOnUse"
          color-interpolation-filters="sRGB"
        >
          <feFlood flood-opacity="0" result="BackgroundImageFix" />
          <feBlend
            mode="normal"
            in="SourceGraphic"
            in2="BackgroundImageFix"
            result="shape"
          />
          <feGaussianBlur
            stdDeviation="110"
            result="effect1_foregroundBlur_448_25"
          />
        </filter>
      </defs>
    </svg>

    <div class="w-full">
      <p
        v-if="headline"
        class="mx-auto mb-4 text-center text-[24px] font-semibold uppercase text-[#cdd6f4]"
      >
        {{ headline }}
      </p>
      <h1
        class="mx-auto mb-4 max-w-md text-center text-[75px] font-semibold text-white"
      >
        {{ title }}
      </h1>
      <p
        class="mx-auto max-w-md text-center text-[32px] leading-tight text-[#E4E4E7]"
      >
        {{ description.slice(0, 200) }}
      </p>
    </div>

    <div class="absolute flex h-full w-full items-center justify-center">
      <Icon
        name="flowbite:atom-outline"
        size="1024"
        class="mx-auto text-black opacity-10 backdrop:blur-md"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
const props = withDefaults(
  defineProps<{ title?: string; description?: string; headline?: string }>(),
  {
    title: "title",
    description: "description",
    headline: "headline",
  },
);

const title = computed(() => props.title.slice(0, 60));
</script>
