//!# Luos: the unity for Robotics
//!
//!Luos is a Rust based embedded OS that let you easily build a robot. You can connect together multiple *Cores*. Each of them can host several *Modules* (both sensor and actuators).
//!
//! The module abstracts the hardware to provide with a standardized API.
//!
//!

#![no_std]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[cfg(not(target_arch = "arm"))]
extern crate mockup_hal;
#[cfg(target_arch = "arm")]
extern crate stm32f0_hal;

/// Abstract used HAL.
pub mod hal {
    #[cfg(target_arch = "arm")]
    pub use stm32f0_hal::*;
    #[cfg(not(target_arch = "arm"))]
    pub use mockup_hal::*;
}

mod luos_core;
pub use luos_core::Core;

pub mod module;
pub use module::Module;
