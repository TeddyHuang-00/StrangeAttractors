export const randomNumber = (min: number, max: number) =>
  Math.random() * (max - min) + min;
export const randomColor = () => {
  let r = randomNumber(0.25, 1);
  let g = randomNumber(0.25, 1);
  let b = randomNumber(0.25, 1);
  let max = Math.max(r, g, b);
  r = Math.floor((r / max) * 255);
  g = Math.floor((g / max) * 255);
  b = Math.floor((b / max) * 255);
  let hex = ((r << 16) | (g << 8) | b).toString(16);
  return `#${hex}`;
};

type NestedArray<T> = T | NestedArray<T>[];
export const randomArray = <T>(
  size: number[],
  unitFunc: () => T,
): NestedArray<T> =>
  Array.from({ length: size[0] }, () =>
    size.length > 1 ? randomArray(size.slice(1), unitFunc) : unitFunc(),
  );
