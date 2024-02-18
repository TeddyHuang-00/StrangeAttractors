export const useTimeSpeed = () => useState("timeSpeed", () => 0.2);
export const useInitialRange = () => useState("initialRange", () => 1);
export const usePointNumber = () => useState("pointNumber", () => 10_000);
export const usePointSize = () => useState("pointSize", () => 0.5);
export const useAttractorSelection = () =>
  useState("attractorSelection", () => "lorenz" as nameAttractor);
