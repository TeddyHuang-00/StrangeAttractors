export type Vec3D = [number, number, number];
export type Attractor = (x: Vec3D) => Vec3D;
export type nameAttractor =
  | "lorenz"
  | "rossler"
  | "thomas"
  | "lu_chen"
  | "dequan_li"
  | "newton_leipnik"
  | "nose_hoover";
