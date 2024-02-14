import type { Vec3D, Attractor } from "./types";

export function getAttractor(name: nameAttractor): Attractor {
  switch (name) {
    case "lorenz":
      return lorenzAttractor();
    case "rossler":
      return rosslerAttractor();
    case "thomas":
      return thomasAttractor();
    case "luchen":
      return luChenAttractor();
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

function thomasAttractor(beta: number = 0.208186): Attractor {
  return ([x, y, z]: Vec3D) => [
    -beta * x + Math.sin(y),
    -beta * y + Math.sin(z),
    -beta * z + Math.sin(x),
  ];
}

function luChenAttractor(
  a: number = 36,
  b: number = 3,
  c: number = 20,
  u: number = -15.15,
): Attractor {
  return ([x, y, z]: Vec3D) => [
    a * (y - x),
    (1 - z) * x + c * y + u,
    x * y - b * z,
  ];
}
