# Introduction

Some time ago I bought the following device:

https://retrogame300.com/products/retro-game-300

, to encourage myself to learn Rust by writing simple games for it. It's a mipsel device, so in theory it should be possible to compile Rust code for it. However, doing this is more complex that I anticipated.

This should also work on other devices using JZ4760 and RetroFW, such as the RS-97, Retro Arcade Mini and LDK.

# Updating your device

The RG300 and a couple other devices tend to use [buildroot](https://buildroot.org/) to build small Linux distributions. The most popular seems to be [RetroFW](https://retrofw.github.io/). However, soon I learnt that my device came with an old version of RetroFW which doesn't support niceties such as being able to do networking over USB to your computer.

So the first step I'd recommend is updating your system to the latest RetroFW version. To do so, you need to download an image from:

https://github.com/retrofw/retrofw.github.io/wiki/Install-Firmware

, in my case, my RG-300 had an IPS sticker and `rg_300_v1.3` and  `2019.6.3` printed in the circuit board under the battery, which points to the IPS version, so you can download the `Type 5B` version. I removed the internal microSD card, made a backup using `dd` of the original card and then used `dd` to overwrite the microSD card with the new firmaware. See the appendices for instructions about extracting the games from the original microSD card and setting them up on the updated RetroFW.

The microSD is apparently the only persistent storage of the RG300, so there's absolutely no risk to those operations. You can keep your original card and use a new one and swapping in the original microSD card will bring you back to the beginning.

(I'm now using exclusively the internal microSD card and not the external one. I prefer that, but it might be more convenient for you to use the external one)

# Connecting to your device

With the latest RetroFW, if you connect your device to your computer using a USB cable and select Charger Mode, a private network will be set up between your computer and the device. You can use telnet and FTP to connect to the device using 169.254.1.1 as the address.

# Building buildroot

RetroFW uses some kind of fork of buildroot which comes with Rust 1.23.0. Besides being old, this is also problematic as recent versions of Rust updated the mips targets to generate mips32r2 instructions:

https://github.com/rust-lang/rust/pull/48874

I opened a bug to see if they restore support:

https://github.com/rust-lang/rust/issues/68865

In the meantime. I created some patches to fix this working on top of buildroot master (commit `f201ca9d0d25b491ec5e1ed4e1d02fd52d027997`). They are in this repo as `00*`. `0001`-`0009` are some patches that @glebm showed me that update Rust to 1.40.0. `0010` is an ugly buildroot hack that forces buildroot to build Rust for the `mipsel-unknown-linux-uclibc` target; buildroot doesn't want to build Rust on uclibc (it's a tier 3 target for Rust, so it's not built nor tested automatically). This is ugly and borks building any other target. `0011` adds a patch on top of Rust to buildroot which disables the generation of mips32r2 instructions. Finally, I had some issues getting `sdl_mixer` to build, so I just copied over the package from RetroFW's buildroot.

To build, apply those patches and copy `buildroot-config` in this repo as `.config` to the root of the buildroot repo. Then build according to buildroot's instructions.

# Writing some software

You can build this repo by altering your `PATH` with:

```
$ export PATH=/path/to/buildroot/output/host/bin/:$PATH
```

and then running:

```
$ cargo build --target mipsel-unknown-linux-uclibc
```

To run this software, I use lftp to copy the binary over:

```
$ lftp 169.254.1.1
> cd /home/retrofw
> put target/mipsel-unknown-linux-uclibc/debug/retrofw2-rust
```

Once it is copied, you can enable debug logs by activating the "Output logs" option in `Settings`. Then use `apps` / `Explorer` to locate `retrofw2-rust` in `/home/retrofw` and execute it. It will draw random color squares; you can press SELECT to exit.

You can see the output by going to the settings screen and using the new `Log Viewer` application.

# Appendices

## Feedback and discussion

https://users.rust-lang.org/t/trying-to-compile-rust-sdl-stuff-for-a-rg300-console-buildroot-mipsel-uclibc/36722/14

## Restoring the original games

My RG300 came with an SD full of software; after overwriting your internal card, you will have an "empty" RetroFW installation. After doing this, I grabbed the [starter pack zips](https://github.com/retrofw/retrofw.github.io/releases/tag/StarterPack2.0), unzipped the OPKs, copied them to the internal SD card and did the "update OPK links thing". This will install a similar set of emulators (and some applications) to the one that originally came with the device.

After that, I extracted the ROMs folder from my original microSD backup (you can use losetup -P to create a loop device per partition in the SD card, the third one contains the games) and copied them too to the internal microSD card. I think the menu integration is slightly worse now, but they seem to work.

## Thanks to...

@glebm, @pingflood, @jbanes on #developer-talk on the RetroFW discord, which helped a noob in these affairs :)
