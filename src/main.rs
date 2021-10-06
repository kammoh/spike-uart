#![no_main]

mod mmio_device;
mod mmio_plugin;
mod uart;

type ThisDeviceType = uart::Uart;
