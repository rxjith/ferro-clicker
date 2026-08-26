# FerroClicker 🦀

A lightweight, fast auto-clicker built with Rust.

FerroClicker provides a simple GUI for automating mouse clicks without unnecessary complexity.

> ⚠️ **Linux input automation has important limitations on modern Wayland desktops.**
> See the [Wayland compatibility](#-wayland-compatibility) section before reporting input-related issues.

---

## 🚀 Download & Run

The easiest way to run FerroClicker on Linux is with the pre-built AppImage.

### 1. Download

Download the latest `ferro-clicker.AppImage` from the [Releases](../../releases) page.

### 2. Make it executable

```bash
chmod +x ferro-clicker.AppImage
```

### 3. Run it

```bash
./ferro-clicker.AppImage
```

---

# AppImage Troubleshooting ‼️

### FUSE errors

If you see an error similar to:

```text
Cannot mount AppImage, please check your FUSE setup
```

your system may be missing the FUSE compatibility library required by the AppImage.

### Option 1 — Run without installing FUSE

You can bypass AppImage mounting entirely:

```bash
APPIMAGE_EXTRACT_AND_RUN=1 ./ferro-clicker.AppImage
```

This is the simplest fallback and does not modify your system.

---

### Option 2 — Install the FUSE compatibility library

#### Ubuntu 24.04 LTS and Ubuntu 26.04 LTS

On modern Ubuntu releases, the FUSE 2 compatibility package is named:

```bash
libfuse2t64
```

Install it with:

```bash
sudo add-apt-repository universe
sudo apt update
sudo apt install -y libfuse2t64
```

Then run:

```bash
./ferro-clicker.AppImage
```

> [!NOTE]
> Ubuntu 26.04 LTS continues to use `libfuse2t64`. If `apt` reports that the package cannot be found, make sure the `universe` repository is enabled.

#### Ubuntu 22.04 LTS

```bash
sudo apt update
sudo apt install -y libfuse2
```

> [!WARNING]
> Do **not** blindly install or replace FUSE packages just to run an AppImage. On modern Ubuntu systems, installing the wrong package combination can interfere with the existing FUSE 3 setup.
>
> If in doubt, use:
>
> ```bash
> APPIMAGE_EXTRACT_AND_RUN=1 ./ferro-clicker.AppImage
> ```

---

## Permission denied when launching

Make sure the AppImage is executable:

```bash
chmod +x ferro-clicker.AppImage
```

Then run:

```bash
./ferro-clicker.AppImage
```

You can verify its permissions with:

```bash
ls -l ferro-clicker.AppImage
```

The file should have executable permissions, for example:

```text
-rwxr-xr-x
```

---

# ⌨️ Global Hotkey Permissions

On some Linux systems, reading global keyboard input may require access to `/dev/input`.

If FerroClicker's global hotkey does not work, add your user to the `input` group:

```bash
sudo usermod -aG input $USER
```

Then **log out completely and log back in**.

You can verify your groups with:

```bash
groups
```

> [!WARNING]
> Membership in the `input` group gives applications broad access to input devices. Only do this if you understand and accept that security implication.

---

# 🖥️ Wayland Compatibility

Modern Linux desktops increasingly use **Wayland** instead of X11.

Wayland intentionally restricts applications from:

* Reading global keyboard input
* Injecting input into other applications
* Simulating unrestricted mouse clicks

Because of this, FerroClicker's ability to detect global hotkeys or generate clicks may depend on:

* Your desktop environment
* Your Wayland compositor
* Security policies
* XWayland availability

## Recommended workaround

If possible, run FerroClicker under an **X11 session**.

You can check your current session type with:

```bash
echo $XDG_SESSION_TYPE
```

If the output is:

```text
wayland
```

try selecting an X11 session from your login screen, if your distribution provides one.

For example, GNOME systems may offer:

```text
GNOME on Xorg
```

KDE systems may offer:

```text
Plasma (X11)
```

> [!IMPORTANT]
> Running the AppImage with `sudo` is **not guaranteed to fix Wayland input restrictions** and is generally not recommended for normal use.
>
> Wayland's restrictions are primarily enforced by the compositor and session security model.

---

# 🖥️ Creating a Desktop Launcher

You can manually add FerroClicker to your desktop's application menu.

## 1. Create the applications directory

```bash
mkdir -p ~/.local/share/applications
```

## 2. Create the desktop entry

```bash
nano ~/.local/share/applications/ferro-clicker.desktop
```

Paste the following:

```ini
[Desktop Entry]
Name=FerroClicker
Comment=Lightweight automated mouse clicker
Exec=/absolute/path/to/ferro-clicker.AppImage
Icon=/absolute/path/to/icon.png
Type=Application
Terminal=false
Categories=Utility;
```

Replace:

```text
/absolute/path/to/
```

with the actual location of your files.

For example:

```ini
Exec=/home/username/Applications/ferro-clicker.AppImage
Icon=/home/username/Applications/icon.png
```

## 3. Make the launcher executable

```bash
chmod +x ~/.local/share/applications/ferro-clicker.desktop
```

Optionally refresh the desktop database:

```bash
update-desktop-database ~/.local/share/applications
```

FerroClicker should now appear in your application launcher.

> [!TIP]
> Avoid using `pkexec` in the desktop entry unless the application genuinely requires elevated privileges.

---

# 🛠️ Building from Source

## Prerequisites

FerroClicker is written in Rust.

Install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your terminal or load Cargo into the current shell:

```bash
source "$HOME/.cargo/env"
```

Verify the installation:

```bash
rustc --version
cargo --version
```

---

## Ubuntu / Debian / Pop!_OS

Install the required build dependencies:

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libx11-dev \
    libxtst-dev \
    libxi-dev
```

### Ubuntu 24.04 / 26.04 AppImage support

If you are also building or running an AppImage that requires FUSE 2 compatibility:

```bash
sudo add-apt-repository universe
sudo apt update
sudo apt install -y libfuse2t64
```

---

## Fedora / RHEL

```bash
sudo dnf install -y \
    gcc \
    pkg-config \
    libX11-devel \
    libXtst-devel \
    libXi-devel
```

Depending on your system and packaging workflow, additional FUSE packages may be required for AppImage support.

---

## Arch Linux / Manjaro

```bash
sudo pacman -S --needed \
    base-devel \
    pkgconf \
    libx11 \
    libxtst \
    libxi
```

For FUSE 2 AppImage compatibility:

```bash
sudo pacman -S --needed fuse2
```

---

## openSUSE

```bash
sudo zypper install \
    gcc \
    pkg-config \
    libX11-devel \
    libXtst-devel \
    libXi-devel
```

For FUSE compatibility, install the appropriate FUSE 2 package available for your openSUSE version.

---

# 📦 Clone and Run

Clone the repository:

```bash
git clone https://github.com/rxjith/ferro-clicker.git
```

Enter the project directory:

```bash
cd ferro-clicker
```

Build and run:

```bash
cargo run --release
```

The optimized binary will also be available under:

```text
target/release/
```

---

# 📦 Building an AppImage

Install `cargo-appimage`:

```bash
cargo install cargo-appimage
```

Then build the AppImage:

```bash
cargo appimage
```

If your AppImage tooling encounters FUSE mounting issues, try:

```bash
APPIMAGE_EXTRACT_AND_RUN=1 cargo appimage
```

The generated AppImage should be placed in the AppImage output directory configured by the build tool.

---

# 🐛 Troubleshooting

## The application launches, but clicking does not work

Check whether you are running Wayland:

```bash
echo $XDG_SESSION_TYPE
```

If the result is:

```text
wayland
```

try running an X11 session instead.

---

## The global hotkey does not work

Check whether your user has access to the required input devices.

After adding yourself to the `input` group:

```bash
sudo usermod -aG input $USER
```

you must completely log out and log back in.

---

## AppImage will not start

Try the extraction fallback:

```bash
APPIMAGE_EXTRACT_AND_RUN=1 ./ferro-clicker.AppImage
```

If that works, the issue is likely related to FUSE/AppImage mounting rather than FerroClicker itself.

On Ubuntu 24.04 or 26.04:

```bash
sudo add-apt-repository universe
sudo apt update
sudo apt install -y libfuse2t64
```

---

# 🤝 Contributing

Contributions, bug reports, and suggestions are welcome.

If you encounter an issue, please include:

* Your Linux distribution and version
* Whether you are using X11 or Wayland
* Your desktop environment
* The exact error message
* Steps to reproduce the issue

This information makes debugging considerably less like archaeological excavation.

---

# 📄 License

FerroClicker is distributed under the [GPL-3.0 License](LICENSE).

See the [`LICENSE`](LICENSE) file for details.
