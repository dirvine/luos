//! # Luos: the unity for Robotics
//!
//! Luos is a Rust based embedded OS that let you easily build a robot. You can connect together multiple `Cores`. Each of them can host several `Drivers` (both sensor and actuators).
//!
//! A driver abstracts the hardware to provide with a standardized API.
//!
//!

#![no_std]
#![feature(core_float)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal as hal;
#[cfg(target_arch = "arm")]
extern crate stm32f0_hal as hal;

pub mod app;
pub mod driver;
mod luos_core;
pub mod units;

pub use luos_core::Core;
