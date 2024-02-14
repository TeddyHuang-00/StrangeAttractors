import type { Attractor } from "./typeAttractor";
import type { Vec3D } from "./typeVec";

export function lorenzAttractor(
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
