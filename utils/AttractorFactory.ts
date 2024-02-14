import type { Vec3D, Attractor } from "./types";

export function getAttractor(name: nameAttractor): Attractor {
  switch (name) {
    case "lorenz":
      return lorenzAttractor();
    case "rossler":
      return rosslerAttractor();
    default:
      return lorenzAttractor();
  }
}

function lorenzAttractor(
  sigma: number = 10,
  rho: number = 28,
  beta: number = 8 / 3,
): Attractor {
  return ([x, y, z]: Vec3D) => [
    sigma * (y - x),
    x * (rho - z) - y,
    x * y - beta * z,
  ];
}

function rosslerAttractor(
  alpha: number = 0.1,
  beta: number = 0.1,
  gamma: number = 14,
): Attractor {
  return ([x, y, z]: Vec3D) => [-y - z, x + alpha * y, beta + z * (x - gamma)];
}
