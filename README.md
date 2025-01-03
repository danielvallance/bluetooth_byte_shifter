# `bluetooth_byte_shifter`

> A simple program which uses an STM32F3DISCOVERY with a HM-10 Bluetooth transceiver to take bytes over a BLE connection from the central device, right shift them by 1, and echo them back.

> This project was created to test my Flutter Bluetooth project (https://github.com/danielvallance/flutter_bluetooth_communication)

## Building and Running

> Within the crate root directory, run ```openocd -f interface/stlink.cfg -f target/stm32f3x.cfg``` to start the OpenOCD client

> Open another terminal in the crate root directory, and run ```itmdump -F -f itm.txt``` to see the ITM output.

> Finally, run ```cargo run``` from the crate root directory, and the program should run.

> There are several dependencies such as OpenOCD, itmdump, the arm-none-eabi toolchain etc which running this project will require (not to mention an STM32F3DISCOVERY and a HM-10 Bluetooth module). The embedded rust discovery book does a good job at describing these, and it is the tutorial I used to create my development environment, so following this will result in a testbed in which you can run this repository: https://docs.rust-embedded.org/discovery/f3discovery/

## Demo

> A demo of the corresponding Flutter app communicating with an STM32F3DISCOVERY running this server can be found here: https://github.com/danielvallance/flutter_bluetooth_communication/tree/main
