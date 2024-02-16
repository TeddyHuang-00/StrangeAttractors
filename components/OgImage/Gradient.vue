<template>
  <div class="w-full h-full flex flex-col justify-center bg-[#1e1e2e]">
    <svg
      class="absolute top-0 right-0"
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

    <div class="w-[600px] pl-[100px]">
      <p
        v-if="headline"
        class="uppercase text-[24px] text-[#cdd6f4] mb-4 font-semibold"
      >
        {{ headline }}
      </p>
      <h1
        class="w-[600px] m-0 text-[75px] font-semibold mb-4 text-white flex items-center"
      >
        <span>{{ title }}</span>
      </h1>
      <p class="text-[32px] text-[#E4E4E7] leading-tight">
        {{ description.slice(0, 200) }}
      </p>
    </div>

    <div
      class="absolute h-100vh w-[50vw] bottom-0 right-0 flex items-center justify-center"
    >
      <Icon
        name="flowbite:atom-outline"
        size="512"
        class="text-white opacity-25 backdrop:blur-md"
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
