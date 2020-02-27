% Cómo liarse como la pata de un romano con Rust y hardware bizarro
% Álex Córcoles - https://github.com/alexpdp7/
% x de marzo de 2020

<!---
$ pandoc charla.md -t slidy -o charla.html -i -s
-->

# ¿Quién soy?

* Programo en Java y Python
* Sobre todo backend web y algo de devops
* Me conocerán de otra charla sobre Rust (empaquetado y compilación cruzada)

# Rust mola

* Rust es un lenguaje fantástico, pero cada día me cuesta más encontrar excusas
  para escribir más Rust y, por tanto, aprender más

* Escribí un agente de monitorización en Rust (y lo empaqueté para RHEL, Debian
  y Ubuntu; también compilación cruzada para una Raspberry Pi). Esto lo tengo
  en producción y me dio para una charla en BcnRust, pero poco hago hoy en día
  a parte de ser el esclavo de Dependabot

* Idea:

  ![Una bonita RG-300](images/rg-300.jpg){ height=12em }

# Programar juegos es divertido

* En el instituto hacía jueguecillos para el Amiga con Blitz Basic

* No es un buen fit para Java/Python (existe PyGame y funciona en la RG-300)

* Parece ser que hay una buena comunidad de Rust para videojuegos
  (arewegameyet.com, /r/rust_gamedev/)

* ... pues me la pido para navidad!

# El aparato

* Corre Linux (Dingux/RetroFW)
* MIPS (JZ4760B)
* 128 megas de RAM
* Batería BP-5L
* Hay toda una familia de cacharros similares (incluida una mini recreativa)

# Pero comienzan los problemas

* https://forge.rust-lang.org/release/platform-support.html

* Tier 1: x86_64/i386 Windows, Mac, Linux

* Tier 2: +ARM, *+MIPS*, +PPC, +Sparc, +RISC, +WASM // +MUSL

* Tier 2.5/3: *+UCLIBC*, +... *SIN BINARIOS*

* Resultado: no puedo usar `rustup`

# Bueno, pues aprendamos buildroot

* RetroFW está basado en buildroot
* buildroot es un entorno de compilación cruzada. Parece una distro, se
  configura con `make menuconfig` como el kernel de Linux
* Lleva `rustc` (una versión arcaica)
* Tarda *mucho* en compilar

# uClibc, SIGILL

* El soporte de `rustc` es primitivo, no compila por defecto si montamos un
  sistema basado en uClibc.
* Parche guarrisimo al canto para forzar uClibc porque no entiendo buildroot
* Los maintainers de Rust decidieron cambiar los targets de MIPS para generar
  código para MIPSr2
* Parche guarrísimo para forzar MIPS "r1"
* Ya que estamos, me chivan un parche para pasar de Rust 1.33 a 1.40

# Haciendo gráficos

* RetroFW (y muchos sistemas de bajos recursos) no corren ni X11 ni Wayland...
* Linux Framebuffer (`/dev/fb0`)
* ¿Cómo se programa eso?
* Casi todo el software para estos aparatos usa SDL 1.2 (SDL 2 == 2013)
* rust-sdl (último commit: 2015)

# Funciona, pero...

* rust-sdl tiene para dibujar rectángulos (Bresenham?)
* Tengo que investigar cómo hacer sprites, etc.
* No parece que haya nada en Rust de mayor nivel
* ... al menos soporta el gamepad y botones sin problema

# Sonido

* En vez de usar Alsa, Pulseaudio, etc...
* ... Linux OSS (`/dev/dsp`)
* Esto está soportado por SDL 1, pero la API es complicada y los bindings están
  a medio hacer
* buildroot/RetroFW vienen con `libao`. Existe `ao_rs` (Último commit: 2018)
* No compila...
* Parche (este menos guarro) y PR
* En mi Linux... pitidito y `kill -9`

# Pero, ¡algo es algo!

* Demo

# No estamos locos

* https://github.com/alexpdp7/retrofw2-rust
* Están todos los parches, instrucciones completas
* Código fuente de la demo
* ¿ODROID Go Advance? ARM. ¿OS menos chungo? Next batch is expected in May 2020
* Hardware barato, pero hay que usar librerías y entornos muy desactualizados

# ¿Preguntas...?

* ¡Adelante!
