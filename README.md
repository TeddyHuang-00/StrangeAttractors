# Strange Attractors Visualizer

A web application for visualizing strange attractors using TresJS.

## Attractors

- [x] Lorenz Attractor ($\sigma = 10, \rho = 28, \beta = \frac{8}{3}$)
  $$
  \begin{aligned}
  \dot{x} &= \sigma(y - x) \\
  \dot{y} &= x(\rho - z) - y \\
  \dot{z} &= xy - \beta z \\
  \end{aligned}
  $$
- [ ] Rössler Attractor ($\alpha = 0.2, \beta = 0.2, \gamma = 5.7$)
  $$
  \begin{aligned}
  \dot{x} &= -y - z \\
  \dot{y} &= x + \alpha y \\
  \dot{z} &= \beta + z(x - \gamma) \\
  \end{aligned}
  $$
- [ ] Thomas Attractor ($\beta = 0.208186$)
  $$
  \begin{aligned}
  \dot{x} &= \sin(y) - \beta x \\
  \dot{y} &= \sin(z) - \beta y \\
  \dot{z} &= \sin(x) - \beta z \\
  \end{aligned}
  $$
- [ ] Aizawa Attractor ($\alpha = 0.095, \beta = 0.7, \gamma = 0.65, \delta = 3.5, \varepsilon = 0.1$)
  $$
  \begin{aligned}
  \dot{x} &= (z - \beta)x - \delta y \\
  \dot{y} &= \delta x + (z - \beta)y \\
  \dot{z} &= \gamma + \alpha z - \frac{z^3}{3} + \varepsilon z x^3 \\
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

## Setup

Make sure to install the dependencies:

```bash
# bun
bun install
```

## Development Server

Start the development server on `http://localhost:3000`:

```bash
# bun
bun run dev
```

## Production

Build the application for production:

```bash
# bun
bun run build
```

Locally preview production build:

```bash
# bun
bun run preview
```

## Static Deployment

Generate a static version of the application:

```bash
# bun
bun run generate
```

This will generate a `dist` directory containing the static files that can be deployed to any static hosting service.
