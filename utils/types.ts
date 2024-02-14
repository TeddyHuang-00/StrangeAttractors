export type Vec3D = [number, number, number];
export type Attractor = (x: Vec3D) => Vec3D;
export type nameAttractor = "lorenz" | "rossler";
