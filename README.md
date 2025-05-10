# Particle Simulator

**A Minimal GPU-Accelerated Particle System Built from Scratch**

## 🎯 Project Overview

This project is a hands-on exploration into the fundamentals of graphics programming. The goal was to build a simple particle simulator from the ground up, intentionally avoiding pre-made production libraries. By doing so, I aimed to gain a deeper understanding of how graphics systems operate under the hood.

## 🧪 Features

* **Particle Spawning**: Initializes particles with randomized positions and velocities.
* **Edge Detection**: Implements boundary checks to keep particles within the simulation window.
* **GPU Acceleration**: Utilizes shaders to handle particle updates and rendering, offloading computations to the GPU.

*Note: This simulator does not include particle-particle collision detection.*

## 🧠 Learning Objectives

* **Lifetime Management**: Managing the lifecycle of particles efficiently.
* **GPU-CPU Data Pipelining**: Understanding how to transfer data between the CPU and GPU effectively.
* **Shader Programming**: Writing custom shaders to handle particle behavior and rendering.
* **Optimization**: Identifying and addressing performance bottlenecks in real-time simulations.

## 🛠️ Technical Details

* **Language**: Rust
* **Graphics API**: WebGPU
* **Shader Language**: WGSL

The choice of Rust and WebGPU was deliberate to ensure safety and performance while working close to the metal. WGSL was used for writing shaders compatible with WebGPU.

## 🚀 Getting Started

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/cjp-afk/particle_simulator.git
   cd particle_simulator
   ```

2. **Build and Run**:
   Ensure you have Rust and the necessary WebGPU support installed.

   ```bash
   cargo run
   ```

*Note: Specific build instructions may vary based on your system's configuration.*

## 📚 Resources

* [WebGPU Specification](https://gpuweb.github.io/gpuweb/)
* [Rust Programming Language](https://www.rust-lang.org/)
* [WGSL Specification](https://www.w3.org/TR/WGSL/)
