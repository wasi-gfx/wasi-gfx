# wasi-gfx

WIT packages enabling UI and graphical applications in WebAssembly — cross-platform, language-agnostic, single build, sandboxed — built on [wasi](https://github.com/WebAssembly/wasi) and the [Component Model](https://github.com/WebAssembly/component-model).

## Packages

- **[surface](packages/surface)** — window/surface management with input events (pointer, keyboard, resize, frame)
- **[frame-buffer](packages/frame-buffer)** — raw pixel buffer access

## Tooling

- [`wasi-gfx-runtime`](https://github.com/wasi-gfx/wasi-gfx-runtime) — runtime that hosts wasi-gfx and wasi:webgpu components
- [`wasi-gfx-shim`](https://github.com/wasi-gfx/wasi-gfx-shim) — shim for running wasi-gfx components on the web
- [`wasi-gfx-js`](https://github.com/wasi-gfx/wasi-gfx-js) — implements the WebGPU JS API on top of wasi:webgpu, for writing wasi-gfx components in JavaScript/TypeScript
- [`wasi-webgpu-headers`](https://github.com/wasi-gfx/wasi-webgpu-headers) — wasi:webgpu native C headers

## Examples

- [`wasi-gfx-examples`](https://github.com/wasi-gfx/wasi-gfx-examples)

## Community

Join us on the [wasi-gfx Discord](https://discord.gg/xUKNS56v).

## History

`wasi-gfx:surface` and `wasi-gfx:frame-buffer` were originally part of the core WASI standard. They've been moved into this dedicated namespace to allow faster iteration — versioned like a library rather than a rigid standard, while staying fully committed to the Wasm Component Model.

`wasi:webgpu` remains an official WASI spec (it maps directly to the stable WebGPU web standard).
