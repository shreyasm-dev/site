---
title = "OpenGL Acceleration on UTM"
date = "2026-03-12"
tags = ["post", "linux", "virtualisation", "nixos", "opengl"]
toc = true
---

# OpenGL Acceleration on UTM

## Background

Along with macOS, I dual boot Fedora using [Asahi Linux](https://asahilinux.org/). I had been meaning for a while to try NixOS and set up a configuration so that, if I chose to do so, moving to it on my second partition would be easier. I decided to virtualise it. Hardware acceleration was a must, as Hyprland requires 3D acceleration in a VM, and it would be far easier to use with it.

## First Attempts: VMWare

I first tried [VMWare Fusion](https://www.vmware.com/products/desktop-hypervisor/workstation-and-fusion). I enabled hardware graphics acceleration by adding `hardware.graphics.enable = true;` and the guest tools with `virtualisation.vmware.guest.enable = true;`, then shutting it down and enabling it in the VMWare host settings. I didn't need it at the moment and so didn't think to check it, but I later realised that it was not at all working. Firefox was rendering incredibly slowly, and `about:support` indicated that it was using software rendering; `glxinfo -B` backed this up.

After much fiddling with kernel modules and drivers, I couldn't get it to work. (Interestingly, no matter how much video memory I gave it, it was always being reported as 1 MB.) I'm not sure if the guest tools had even been working, as I didn't try using input capture or display settings.

## On UTM

I decided to switch to UTM, which I knew for a fact supported graphics acceleration (but so did VMWare, so whether it worked remained to be seen). I enabled it while creating the VM and ran the installer. Not wanting to waste my time, I opened the terminal within the installer itself and ran `nix-shell -p mesa-demos --run "glxinfo -B"`. Had it not been accelerated, I likely would have proceeded and troubleshot after the installation---but setting up any kind of configuration ended up being unnecessary, as the output indicated that it did work.

## Graphical Problems

However, as soon as I imported my old configuration and rebuilt it, Alacritty and VSCodium began having rendering problems. Rebooting seemed to somehow fix the latter, but Alacritty did not render any content and was displaying the solitaire/ghosting effect, leaving a trail across the screen as it moved.

Running `glxinfo -B` indicated that only OpenGL 2.1 was available---even though UTM supports higher versions---while Alacritty requires OpenGL 3.1.

```plaintext
name of display: :0
display: :0  screen: 0
direct rendering: Yes
Extended renderer info (GLX_MESA_query_renderer):
    Vendor: Mesa (0x1af4)
    Device: virgl (ANGLE (Apple, Apple M2 Pro, OpenGL 4.1 Metal - 89.4)) (0x1010)
    Version: 25.2.6
    Accelerated: yes
    Video memory: 0MB
    Unified memory: no
    Preferred profile: compat (0x2)
    Max core profile version: 0.0
    Max compat profile version: 2.1
    Max GLES1 profile version: 1.1
    Max GLES[23] profile version: 3.0
OpenGL vendor string: Mesa
OpenGL renderer string: virgl (ANGLE (Apple, Apple M2 Pro, OpenGL 4.1 Metal - 89.4))
OpenGL version string: 2.1 Mesa 25.2.6
OpenGL shading language version string: 1.20

OpenGL ES profile version string: OpenGL ES 3.0 Mesa 25.2.6
OpenGL ES profile shading language version string: OpenGL ES GLSL ES 3.00
```

Setting the following environment variables in my configuration fixed it:

```nix
environment.sessionVariables = {
  MESA_GL_VERSION_OVERRIDE = "3.3FC";
  MESA_GLSL_VERSION_OVERRIDE = "330";
};
```

This forced it to request OpenGL 3.3, and Alacritty seemed to be working.

My guess for why it wasn't working in the first place is that it was for some reason defaulting to the compatibility profile, as is evident in the `glxinfo` output above, but I'm not sure why that's happening in the first place.

## My Configuration

For fun, here's the layout of my configuration:

```plaintext
~/.config/nixos/
├── configuration.nix
├── hardware-configuration.nix
└── modules/
    ├── core/
    │   ├── boot.nix
    │   ├── graphics.nix
    │   ├── locale.nix
    │   ├── networking.nix
    │   ├── nix.nix
    │   └── peripherals.nix
    ├── desktop/
    │   └── plasma.nix
    ├── packages/
    │   ├── git.nix
    │   ├── other.nix
    │   └── vim.nix
    └── users/
        └── shreyasm/
            ├── packages/
            │   ├── alacritty.nix
            │   ├── other.nix
            │   └── vscodium.nix
            └── user.nix
```

This is the second version, as I used flakes and Home Manager in the first attempt but it ended up being far too slow to build. To be fair, this was in VM with limited IO throughput, and the only optimisation I tried was using [Lix](https://lix.systems/) instead of Nix, but I felt that I didn't really need Home Manager or flakes anyway, at least not currently, and doing it this way was in any case faster.
