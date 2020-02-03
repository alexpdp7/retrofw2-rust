# Introduction

Some time ago I bought the following device:

https://retrogame300.com/products/retro-game-300

, to encourage myself to learn Rust by writing simple games for it. It's a mipsel device, so in theory it should be possible to compile Rust code for it. However, doing this is more complex that I anticipated.

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

RetroFW uses some kind of fork of buildroot which comes with Rust 1.23.0. I decided to use RetroFW master that comes with 1.33.0. However, I copied over the buildroot configuration from RetroFW and removed some deprecated options. You can find the result at `buildroot-config` in this repo.

If you try to build this, it will fail, because buildroot doesn't support uclibc which the RetroFW configuration uses. Indeed, the Rust uclibc target has a low tier of support for Rust, but it *seems* to work. I applied the following patch to get it to build:

```
diff --git a/package/rust/rust.mk b/package/rust/rust.mk
index 5d14fc6682..5f9d35ce0b 100644
--- a/package/rust/rust.mk
+++ b/package/rust/rust.mk
@@ -41,7 +41,7 @@ HOST_RUST_POST_EXTRACT_HOOKS += HOST_RUST_EXCLUDE_ORIG_FILES
 define HOST_RUST_CONFIGURE_CMDS
        ( \
                echo '[build]'; \
-               echo 'target = ["$(RUSTC_TARGET_NAME)"]'; \
+               echo 'target = ["mipsel-unknown-linux-uclibc"]'; \
                echo 'cargo = "$(HOST_CARGO_BIN_DIR)/cargo/bin/cargo"'; \
                echo 'rustc = "$(HOST_RUST_BIN_DIR)/rustc/bin/rustc"'; \
                echo 'python = "$(HOST_DIR)/bin/python2"'; \
@@ -54,7 +54,7 @@ define HOST_RUST_CONFIGURE_CMDS
                echo 'prefix = "$(HOST_DIR)"'; \
                echo '[rust]'; \
                echo 'channel = "stable"'; \
-               echo '[target.$(RUSTC_TARGET_NAME)]'; \
+               echo '[target.mipsel-unknown-linux-uclibc]'; \
                echo 'cc = "$(TARGET_CROSS)gcc"'; \
                echo $(HOST_RUST_JEMALLOC_CONF); \
        ) > $(@D)/config.toml
```

, which is a terrible idea, but it works. If you compile buildroot, it will work with some warnings (also ripgrep will fail to build) and generate a buildroot with Rust support for `mipsel-unknown-linux-uclibc`.

# Writing some software

Apparently RetroFW 2 just supports SDL 1.2, not the latest SDL 2, so I cloned:

https://github.com/brson/rust-sdl/

The most I've managed to do is replace the contents of `src/sdl-demo/main.rs` with:

```
extern crate sdl;
extern crate rand;

use rand::Rng;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};

fn main() {
    println!("1");
    sdl::init(&[sdl::InitFlag::Video]);
    println!("1");
    let screen = match sdl::video::set_video_mode(320, 240, 16, &[SurfaceFlag::SWSurface], &[]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };
    println!("1");

    // Note: You'll want to put this and the flip call inside the main loop
    // but we don't as to not startle epileptics
            screen.fill_rect(Some(sdl::Rect {
                x: 0,
                y: 0,
                w: 320,
                h: 240
            }), sdl::video::Color::RGB {0: 255, 1: 0, 2:0});

    println!("1");

    screen.flip();
    println!("1");

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, _, _, _)
                    if k == Key::Escape
                        => break 'main,
                _ => {}
            }
        }
    }

    sdl::quit();
}
```

You can build this by altering your `PATH` with:

```
$ export PATH=/path/to/buildroot/output/host/bin/:$PATH
```

creating a Cargo config in the `src/sdl-demo`:

```
[target.mipsel-unknown-linux-uclibc]
linker = "/path/to/buildroot/output/host/usr/bin/mipsel-RetroFW-linux-uclibc-cc"
```

and then running:

```
$ cargo build --target mipsel-unknown-linux-uclibc
```

in the `src/sdl-demo`.

To run this software, I use lftp to copy the binary over:

```
$ lftp 169.254.1.1
> cd /home/retrofw
> put target/mipsel-unknown-linux-uclibc/debug/sdl-demo
```

Once it is copied, you can enable debug logs by activating the "Output logs" option in `Settings`. Then use `apps` / `Explorer` to locate `sdl-demo` in `/home/retrofw` and execute it. It will draw a fuzzy, reddish rectangle and crash after a few seconds.

You can see the output by going to the settings screen and using the new `Log Viewer` application.

# Appendices

## Feedback and discussion

https://users.rust-lang.org/t/trying-to-compile-rust-sdl-stuff-for-a-rg300-console-buildroot-mipsel-uclibc/36722/14

## Restoring the original games

My RG300 came with an SD full of software; after overwriting your internal card, you will have an "empty" RetroFW installation. After doing this, I grabbed the [starter pack zips](https://github.com/retrofw/retrofw.github.io/releases/tag/StarterPack2.0), unzipped the OPKs, copied them to the internal SD card and did the "update OPK links thing". This will install a similar set of emulators (and some applications) to the one that originally came with the device.

After that, I extracted the ROMs folder from my original microSD backup (you can use losetup -P to create a loop device per partition in the SD card, the third one contains the games) and copied them too to the internal microSD card. I think the menu integration is slightly worse now, but they seem to work.
