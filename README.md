# MelbOs

![MelbOs Logo](https://i.imgur.com/dqCboAr.png)  

---  

## What is this  

MelbOs is a bootloader / operating system made from scratch with Rust.  
It's pretty basic as I don't want to build a new linux, this is just for learning purposes.  
The project is based on [this blog](https://os.phil-opp.com) and was created by [Philipp Oppermann](https://github.com/phil-opp).  
The project is frozen at the moment. But I intend to finish it.

## Building  

This project runs on a nightly version of Rust. At least *nightly 2020-07-15* is needed to compile.
To install Rust nightly run `rustup update nightly --force` on your terminal.  

You can build with `cargo build`.  
Or create a image with *bootimage*. Install it with `cargo install bootimage`, then run `cargo bootimage`.  
The image can be foun in `target/x86_64-melb-os/debug`.

## Running  

The image can be run with [QEMU](https://www.qemu.org): `cargo run`.  

You can also write the image to and USB stick and boot on hardware.
I recommend using [Rufus](https://rufus.ie/en_US/).  
Also, it's possible to use Linux's native tool for it:

`dd if=target/x86_64-melb-os/debug/bootimage-MelbOs.bin of=/dev/FlashDriveName && sync`  

Where "FlashDriveName" is, obviously, the name of your flash drive.  
Run `man dd` if you have any problems.

## Testing  

Simply run `cargo test`.  
Or run `cargo test --test test_name` for a specific file.  

## License  

Following the request of the original author this project follows the Apache 2 license.  
Read the `LICENSE` file for more information.
