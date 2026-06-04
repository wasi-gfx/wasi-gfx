# wasi-gfx:surface

WIT interface for creating and managing a graphical surface — a drawable area, sometimes a window.

A `surface` is the entry point for any graphical app — it gives you a drawable area and a stream of user input events. You create one with an optional size, then loop over frame events to drive rendering, and listen to pointer/keyboard/resize events to handle user interaction.
