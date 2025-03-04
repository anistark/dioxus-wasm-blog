# Dioxus WASM Blog

Welcome to the Dioxus WASM Blog project! This project is built using Dioxus, a modern Rust framework for building fast and reliable web applications with WebAssembly.
This is a sample project used to showcase what dioxus is capable of.

## Introduction

This project is a blog application that leverages the power of WebAssembly and Rust to deliver a high-performance, interactive user experience. Dioxus provides a reactive, component-based architecture similar to popular JavaScript frameworks.
The goal is to provide an alternate to js based frameworks (react, vue, etc) for web development.

### Uses

- [Dioxus](https://dioxuslabs.com/)
- [WASM](https://webassembly.org/)

## Installation

To get started with this project, follow these steps:

#### Clone repo
    ```sh
    git clone https://github.com/anistark/dioxus-wasm-blog.git
    cd dioxus-wasm-blog
    ```

#### Setup WebAssembly target:
    ```sh
    rustup target add wasm32-unknown-unknown
    ```

#### Build for Production
    ```sh
    cargo build --target wasm32-unknown-unknown
    ```

    or

    ```sh
    dx build
    ```

#### Serve for Development
    ```sh
    dx serve
    ```

Open `http://localhost:8080` in your browser.
