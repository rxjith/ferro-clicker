# FerroClicker 🦀

A lightweight, fast auto-clicker built with Rust.

FerroClicker provides a simple GUI for automated mouse clicking, configurable delays, and global hotkey control.

---

## ✨ Features

- 🖱️ Automated mouse clicking
- ⚡ Adjustable click interval from 1–1000 ms
- ⏳ Configurable startup delay from 0–10 seconds
- 🔢 Visible startup countdown
- ⌨️ Global toggle hotkeys from F1–F12
- 🔐 `/dev/input` permission detection
- 🔄 Built-in permission re-check
- 📊 PAUSED, STARTING, and ACTIVE status indicators
- 🦀 Built with Rust
- 📦 Portable AppImage
- 🐧 Linux support
- 🔓 Open source

---

> [!WARNING]
> Linux input automation has important limitations on modern Wayland desktops.
>
> See [Wayland Compatibility](#-wayland-compatibility) before reporting input-related issues.

---

## 🚀 Download & Run

Download the latest `ferro-clicker.AppImage` from the [Releases](../../releases) page.

Make it executable:

```bash
chmod +x ferro-clicker.AppImage
```

Run it:

```bash
./ferro-clicker.AppImage
```

---

## ‼️ AppImage Troubleshooting

### FUSE error

If you see:

```text
Cannot mount AppImage, please check your FUSE setup
```

you can run the AppImage without installing FUSE:

```bash
APPIMAGE_EXTRACT_AND_RUN=1 ./ferro-clicker.AppImage
```

### Ubuntu 24.04 / 26.04 LTS

Install the FUSE 2 compatibility library:

```bash
sudo add-apt-repository universe
sudo apt update
sudo apt install -y libfuse2t64
```

Then run:

```bash
./ferro-clicker.AppImage
```

### Ubuntu 22.04 LTS

```bash
sudo apt update
sudo apt install -y libfuse2
```

> [!TIP]
> If the AppImage still refuses to mount, use:
>
> ```bash
> APPIMAGE_EXTRACT_AND_RUN=1 ./ferro-clicker.AppImage
> ```

---

## ⌨️ Global Hotkey Permissions

FerroClicker reads Linux input events from `/dev/input` to detect global hotkeys. The application checks whether it can access these devices and displays a warning if permissions are missing. :contentReference[oaicite:2]{index=2}

If the hotkey does not work, add your user to the `input` group:

```bash
sudo usermod -aG input $USER
```

Then **log out and log back in**.

You can verify your groups with:

```bash
groups
```

> [!WARNING]
> Membership in the `input` group gives applications broad access to input devices. Only do this if you understand the security implications.

---

## 🖥️ Wayland Compatibility

Modern Linux desktops increasingly use **Wayland** instead of X11.

Wayland intentionally restricts applications from:

- Reading global keyboard input
- Injecting input into other applications
- Simulating unrestricted mouse clicks

Because of this, FerroClicker's functionality may depend on:

- Your desktop environment
- Your Wayland compositor
- Security policies
- XWayland availability

You can check your current session:

```bash
echo $XDG_SESSION_TYPE
```

If the output is:

```text
wayland
```

try using an X11 session if your distribution provides one.

Examples:

```text
GNOME on Xorg
```

```text
Plasma (X11)
```

> [!IMPORTANT]
> Running FerroClicker with `sudo` is not guaranteed to bypass Wayland's input restrictions.

---

## 🛠️ Building from Source

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Load Cargo:

```bash
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version
cargo --version
```

### Ubuntu / Debian / Pop!_OS

```bash
sudo apt update

sudo apt install -y \
    build-essential \
    pkg-config \
    libx11-dev \
    libxtst-dev \
    libxi-dev
```

### Fedora / RHEL

```bash
sudo dnf install -y \
    gcc \
    pkg-config \
    libX11-devel \
    libXtst-devel \
    libXi-devel
```

### Arch Linux / Manjaro

```bash
sudo pacman -S --needed \
    base-devel \
    pkgconf \
    libx11 \
    libxtst \
    libxi
```

### openSUSE

```bash
sudo zypper install \
    gcc \
    pkg-config \
    libX11-devel \
    libXtst-devel \
    libXi-devel
```

---

## 📦 Clone and Run

```bash
git clone https://github.com/rxjith/ferro-clicker.git

cd ferro-clicker

cargo run --release
```

The optimized binary will be available at:

```text
target/release/ferro-clicker
```

You can also build without running:

```bash
cargo build --release
```

Then launch it manually:

```bash
./target/release/ferro-clicker
```

---

## 📦 Building an AppImage

Install `cargo-appimage`:

```bash
cargo install cargo-appimage
```

Build:

```bash
cargo appimage
```

If AppImage tooling encounters FUSE issues:

```bash
APPIMAGE_EXTRACT_AND_RUN=1 cargo appimage
```

---

## 🐛 Troubleshooting

### Clicking does not work

Check your session type:

```bash
echo $XDG_SESSION_TYPE
```

If you're running Wayland, try an X11 session.

### Global hotkey does not work

Check `/dev/input` permissions:

```bash
sudo usermod -aG input $USER
```

Then log out and log back in.

### AppImage does not start

Try:

```bash
APPIMAGE_EXTRACT_AND_RUN=1 ./ferro-clicker.AppImage
```

On Ubuntu 24.04 / 26.04:

```bash
sudo add-apt-repository universe
sudo apt update
sudo apt install -y libfuse2t64
```

---

## 🤝 Contributing

Contributions, bug reports, and suggestions are welcome.

When reporting an issue, include:

- Linux distribution and version
- Desktop environment
- X11 or Wayland
- Exact error message
- Steps to reproduce

This makes debugging significantly less like archaeological excavation.

---

## 📄 License

FerroClicker is distributed under the [GPL-3.0 License](LICENSE).

See [`LICENSE`](LICENSE) for details.