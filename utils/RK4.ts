import type { Attractor } from "./typeAttractor";
import type { Vec3D } from "./typeVec";

function vecAdd(a: Vec3D, b: Vec3D): Vec3D {
  return a.map((ai, i) => ai + b[i]) as Vec3D;
}

function vecScale(a: Vec3D, s: number): Vec3D {
  return a.map((ai) => ai * s) as Vec3D;
}

export default function (x: Vec3D, f: Attractor, dt: number = 1.0) {
  const k1 = f(x);
  const k2 = f(vecAdd(x, vecScale(k1, 0.5 * dt)));
  const k3 = f(vecAdd(x, vecScale(k2, 0.5 * dt)));
  const k4 = f(vecAdd(x, vecScale(k3, dt)));
  return vecAdd(
    x,
    vecScale(
      vecAdd(k1, vecAdd(vecScale(k2, 2), vecAdd(vecScale(k3, 2), k4))),
      dt / 6,
    ),
  );
}
