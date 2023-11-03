Chapters on GPU Ray Tracing
====================================================================================================

This repository hosts a new book series that walks you through building your own GPU path tracer. It
is intended for readers of the [_Ray Tracing In One Weekend_](https://raytracing.github.io/) series
who are curious about implementing the same concepts on a GPU using a modern graphics API.

The book will guide you through the building blocks of a GPU-based renderer using the same
step-by-step approach in _Ray Tracing In One Weekend_. The goal is **not** to build a
fully-featured and highly optimized real-time renderer; rather, it is to show you how to translate
the concepts introduced in _Ray Tracing In One Weekend_ into a design that is practical to port to
all modern GPU platforms while paying attention to some of the common pitfalls of GPU programming.

I hope that, by completing this book, you will have built a renderer that serves as a foundation
for future learning and experimentation. It won't be the most-efficient renderer but it will be able
to draw the scenes from _Ray Tracing In One Weekend_ at near-interactive rates.

Project Status
--------------
This book is a work-in-progress and has not been released yet. I'll update this README as soon as
there is a first draft!

Directory Structure
-------------------
TODO

Source Code
-----------
_Ray Tracing In One Weekend_ deliberately avoids the use of APIs and special-purpose libraries.
However, programming a GPU requires you to develop against an API that supports the particular piece
of hardware that you want to target.

My goal with this book is for all of the concepts to easily map to any graphics framework and
programming environment. GPU programming can naturally involve a lot of boilerplate and I want to
keep the barrier for entry as low as possible for someone who wants to get started with building a
ray tracer right away without having to learn all the intricacies of window management and engine
infrastructure.

To that end, I decided to provide some support code to keep some of that friction low. I made the
following API and programming language choices for the code examples:

1. **WebGPU** is a new API that defines a good abstraction for all baseline features one can find in
   a modern graphics API. A reader that wants to use a different API can translate the concepts
   fairly easily. In particular, WebGPU has native implementations that are easily portable to any
   OS and GPU that supports Vulkan, Metal, and D3D. The code examples will specifically use the
   [wgpu](https://github.com/gfx-rs/wgpu) library which supports Vulkan, Metal, D3D11, D3D12,
   OpenGL ES, and WebGPU via WebAssembly.

2. I chose **Rust** as the programming language for both its safety and ease of use. I've found that
   Rust has often made my development faster as it resolves many potential memory bugs and data
   races at compile time while staying fairly low-level. Another advantage of Rust is that it takes
   virtually no effort to get the examples to run on different operating systems as the package
   manager takes care of this automatically.

I'm cognizant of the fact Rust may feel unfamiliar to some readers. I will strive to keep the code
examples free of esoteric Rust-isms to keep the code understandable to a reader who is familiar with
C.

That said, none of this should prevent you from implementing this book in your favorite language, on
your favorite OS, using your favorite API. In fact, [wgpu](https://github.com/gfx-rs/wgpu/) (the
library used by the code examples) has
[native bindings](https://github.com/gfx-rs/wgpu-native#bindings) for several programming languages.

Building and Running
--------------------
TODO

Corrections & Contributions
----------------------------
If you spot errors, have suggested corrections, or would like to help out with the project,
_**please review the [CONTRIBUTING][] document for the most effective way to proceed.**_

This work  is licensed under a
[Creative Commons Attribution 4.0 International License][cc-by]. You can find a copy of the license
text at [LICENSE][]

[![CC BY 4.0][cc-by-image]][cc-by]

[CONTRIBUTING]: CONTRIBUTING.md
[LICENSE]: LICENSE
[cc-by]: http://creativecommons.org/licenses/by/4.0/
[cc-by-image]: https://i.creativecommons.org/l/by/4.0/88x31.png
[cc-by-shield]: https://img.shields.io/badge/License-CC%20BY%204.0-lightgrey.svg
