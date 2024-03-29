# Strange Attractors Visualizer

A web application for visualizing strange attractors using TresJS and WASM.

## Attractors

- [x] Lorenz Attractor ($\sigma = 10, \rho = 28, \beta = \frac{8}{3}$)

$$
\begin{aligned}
\dot{x} &= \sigma(y - x) \\
\dot{y} &= x(\rho - z) - y \\
\dot{z} &= xy - \beta z \\
\end{aligned}
$$

- [x] Rössler Attractor ($\alpha = 0.1, \beta = 0.1, \gamma = 14$)

$$
\begin{aligned}
\dot{x} &= -y - z \\
\dot{y} &= x + \alpha y \\
\dot{z} &= \beta + z(x - \gamma) \\
\end{aligned}
$$

- [x] Thomas Attractor ($\beta = 0.208186$)

$$
\begin{aligned}
\dot{x} &= \sin(y) - \beta x \\
\dot{y} &= \sin(z) - \beta y \\
\dot{z} &= \sin(x) - \beta z \\
\end{aligned}
$$

- [x] Dequan Li Attractor ($\alpha = 40, \beta = 1.833, \gamma = 0.16, \delta = 0.65, \varepsilon = 55, \zeta = 20$)

$$
\begin{aligned}
\dot{x} &= \alpha (y - x) + \gamma x z \\
\dot{y} &= \varepsilon x + \zeta y - x z \\
\dot{z} &= \beta z + x y - \delta x^2 \\
\end{aligned}
$$

- [x] Newton-Leipnik Attractor ($\alpha = 0.4, \beta = 0.175$)

$$
\begin{aligned}
\dot{x} &= y - \alpha x + 10 y z \\
\dot{y} &= -x - \alpha y + 5 x z \\
\dot{z} &= \beta z - 5 x y \\
\end{aligned}
$$

- [x] Nosé-Hoover Attractor ($\alpha = 1.5$)

$$
\begin{aligned}
\dot{x} &= y \\
\dot{y} &= -x + y z \\
\dot{z} &= \alpha - y^2 \\
\end{aligned}
$$

- [x] Halvorsen Attractor ($\alpha = 1.4$)

$$
\begin{aligned}
\dot{x} &= -\alpha x - 4 y - 4 z - y^2 \\
\dot{y} &= -\alpha y - 4 z - 4 x - z^2 \\
\dot{z} &= -\alpha z - 4 x - 4 y - x^2 \\
\end{aligned}
$$

- [x] Chen-Lee Attractor ($\alpha = 5, \beta = -10, \gamma = -0.38$)

$$
\begin{aligned}
\dot{x} &= \alpha x - y z \\
\dot{y} &= \beta y + x z \\
\dot{z} &= \gamma z + \frac{1}{3} x y \\
\end{aligned}
$$

- [x] Bouali Attractor ($\alpha = 0.1, \beta = -0.1, \mu=1$)

$$
\begin{aligned}
\dot{x} &= \alpha z - x (1 - y) \\
\dot{y} &= y (1 - x^2) \\
\dot{z} &= \beta x - \mu z (1 - y) \\
\end{aligned}
$$

- [x] Finance Attractor ($\alpha = 0.001, \beta = 0.2, \gamma = 1.1$)

$$
\begin{aligned}
\dot{x} &= \left(\frac{1}{\beta} - \alpha\right) x + z + x y \\
\dot{y} &= -\beta y - x^2 \\
\dot{z} &= -\gamma z - x \\
\end{aligned}
$$

- [x] Arneodo Attractor ($\alpha = -5.5, \beta = 3.5, \gamma = -1$)

$$
\begin{aligned}
\dot{x} &= y \\
\dot{y} &= z \\
\dot{z} &= - \alpha x - \beta y - z + \gamma x^3 \\
\end{aligned}
$$

- [x] Sprott B Attractor ($\alpha = 0.4, \beta = 1.2, \gamma = 1$)

$$
\begin{aligned}
\dot{x} &= \alpha y z \\
\dot{y} &= x - \beta y \\
\dot{z} &= \gamma - x y \\
\end{aligned}
$$

- [x] Sprott-Linz F Attractor ($\alpha = 0.5$)

$$
\begin{aligned}
\dot{x} &= y + z \\
\dot{y} &= - x + \alpha y \\
\dot{z} &= x^2 - z \\
\end{aligned}
$$

- [x] Dadras Attractor ($\alpha = 3, \beta = 2.7, \gamma = 1.7, \delta = 2, \varepsilon = 9$)

$$
\begin{aligned}
\dot{x} &= y - \alpha x + \beta y z \\
\dot{y} &= \gamma y - x z + z \\
\dot{z} &= \delta x y - \varepsilon z \\
\end{aligned}
$$

## Approximation Solver

- [x] Runge-Kutta 4th Order Method

$$
\begin{aligned}
\dot{y} &= f(y) \\
k_1 &= f(y_n) \\
k_2 &= f(y_n + \frac{h}{2}k_1) \\
k_3 &= f(y_n + \frac{h}{2}k_2) \\
k_4 &= f(y_n + hk_3) \\
y_{n+1} &= y_n + \frac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4) \\
\end{aligned}
$$

## Performance

- [x] Refactor code to using Rust WebAssembly
  - FPS boost up to 2x

## Setup

Make sure to install the dependencies:

```bash
# bun
bun install
```

You may also need Rust toolchain installed, and `wasm-pack` for building the Rust WebAssembly:

```bash
# rustup
rustup install stable
cargo install wasm-pack
```

## Development Server

Start the development server on `http://localhost:3000`:

```bash
# bun
bun run wasm && bun run dev
```

## Production

### Build

Build the application for production:

```bash
# bun
bun run wasm && bun run build
```

### Static Generation

Generate a static version of the application:

```bash
# bun
bun run wasm && bun run generate
```

Either way, it will generate a `dist` directory containing the static files that can be deployed to any static hosting service.

## Locally Preview

Locally preview production build (by either `build` or `generate`):

```bash
# bun
bun run preview
```
