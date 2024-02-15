export const useTimeSpeed = () => useState("timeSpeed", () => 0.2);
export const usePointNumber = () => useState("pointNumber", () => 1000);
export const usePointSize = () => useState("pointSize", () => 0.1);
export const useDetailLevel = () => useState("detailLevel", () => 6);
export const useAttractorSelection = () =>
  useState("attractorSelection", () => "lorenz" as nameAttractor);