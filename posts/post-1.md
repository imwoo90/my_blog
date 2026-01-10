---
title: "Bare-Metal Rust: Blinking an LED"
date: "2023-10-26"
author: "John Doe"
description: "A deep dive into writing Rust for microcontrollers, starting from the ground up without any standard library."
image_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuBCi_pzNBVAhdhyYbqWap1-J9p4C5e7Qg6V8ZzF0E8aKgg5gcyH6nlx6u8TcJznHXhdlE6wILKxlF_rVI-dsKeRThWqbJXnybVHkiS879RY2kqgOxaj8lGDFgqgIlrbsxFIjg_SsQ7ddoeF67K9JgN6ZeqobfwkcRHjFnG2BbRCumKTZTwKS1bcPJuFg6X5cEc4popXC2x7h-Hg6_A_WJ1ZEyEQg9cnRfLTod-WujsdskNkUlBc-SRxRmTO7k0sa4JyR-4QUQ4or0YC"
tags: ["bare-metal", "embedded"]
---

Welcome to the fascinating world of bare-metal programming with Rust! In this post, we'll embark on a journey to the very foundations of embedded systems. Our goal is simple but fundamental: to blink an LED on a microcontroller using Rust, without relying on any operating system or standard library (no-std). This is the "Hello, World!" of the embedded world, and it's a perfect starting point to understand what makes Rust a powerful choice for low-level development.

## Why Bare-Metal Rust?

Rust's emphasis on safety, performance, and concurrency makes it an excellent candidate for embedded systems. Its zero-cost abstractions mean you don't pay for what you don't use, which is critical in resource-constrained environments. The powerful type system and ownership model help prevent common bugs like null pointer dereferences and data races at compile time, saving hours of debugging on hardware.

```rust
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    loop {
        led.set_high().unwrap();
        // Delay for a bit
        led.set_low().unwrap();
        // Delay for a bit
    }
}
```

The code snippet above shows a simplified version of what we're aiming for. We take control of the device peripherals, configure a specific GPIO pin (in this case, PC13, which is often connected to an onboard LED on STM32 boards) as an output, and then enter an infinite loop to toggle its state.

## Setting Up the Environment

Before we can blink, we need to set up our development environment. This involves installing the Rust compiler, adding the correct target for our microcontroller (e.g., `thumbv7em-none-eabihf` for an STM32F4 series), and getting the necessary tools for flashing and debugging, like `probe-rs` or `openocd`.

> This tutorial assumes you are using an STM32 "Blue Pill" board, but the concepts are transferable to other microcontrollers with their respective Hardware Abstraction Layer (HAL) crates.

Once the environment is ready, we'll create a new no-std Rust project and add the required dependencies to our `Cargo.toml`. This will include the Peripheral Access Crate (PAC) for our specific MCU, and a higher-level Hardware Abstraction Layer (HAL) crate that provides safer and more ergonomic APIs to control the hardware.
