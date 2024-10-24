
# Reactor

**Reactor** is a lightweight and experimental WebAssembly runtime, built as a learning project. The primary goal was to compile a "Hello, World!" program in Rust and run it inside my very own runtime (which isn't that simple because the binary contains **alot** of framework code). While Reactor is not designed for performance or real-world use, it serves as a stepping stone to understanding WebAssembly internals.

## What is Reactor?

Reactor is a custom WebAssembly interpreter focused on simplicity and educational exploration. It’s an ongoing project aimed at gaining hands-on experience with the core concepts of WebAssembly runtimes, without diving into advanced features like JIT. It’s the perfect environment for anyone who wants to experiment and learn about how WASM and WASI operate under the hood.

## Goals

- 🛠 **Learn by Doing**: The main objective is to explore WebAssembly runtime internals by running a simple Rust program.
- 🎯 **Rust-First**: The runtime was built with Rust-compiled WebAssembly modules in mind, with the initial target being a "Hello, World!" application.
- 🚧 **Ongoing Experimentation**: This project will continue to evolve as more WebAssembly concepts and features are explored.

## Contributing

Since Reactor is a learning project, contributions are welcomed, especially if you’re also interested in understanding WebAssembly and want to tinker along. Feel free to fork, play, and share your findings!

## License

Reactor is licensed under the MIT License. See `LICENSE` for more details.
