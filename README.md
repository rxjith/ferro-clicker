# FerroClicker 🦀
A lightweight, cross-platform auto-clicker built in Rust.

---

## 🚀 Download & Run (AppImage)
The easiest way to run FerroClicker on Linux is by downloading the pre-built `.AppImage`.

1. Download `ferro-clicker.AppImage` from the **[Releases](../../releases)** page.
2. Open your terminal and grant execution permissions:
   ```bash
   chmod +x ferro-clicker.AppImage
   ```
3. Run the application:
   ```bash
   ./ferro-clicker.AppImage
   ```

> [!WARNING]
> **Global Hotkey Permissions:**
> Global hotkey permissions require adding your user to the `input` group:
> ```bash
> sudo usermod -aG input $USER
> ```
> *Restart or log out and back into your session for changes to take effect.*

> [!CAUTION]
> **Wayland Desktop Sessions:**
> If you are running a Wayland session (default on modern GNOME/KDE) and global mouse clicks are not registering, launch the AppImage with elevated privileges:
> ```bash
> sudo ./ferro-clicker.AppImage
> ```

---

## 🛠️ Developer Installation Guide

If you prefer compiling the application yourself, follow the instructions for your distribution.

> [!NOTE]
> Ensure you have Rust installed via `rustup` before proceeding:
> ```bash
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
> ```

### 1. Install System Dependencies

**Ubuntu / Debian / Pop!_OS**
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libx11-dev libxtst-dev libxi-dev
```

**Fedora / RHEL**
```bash
sudo dnf install -y gcc pkg-config libX11-devel libXtst-devel libXi-devel
```

**Arch Linux / Manjaro**
```bash
sudo pacman -S --needed base-devel pkgconf libx11 libxtst libxi
```

**openSUSE**
```bash
sudo zypper install gcc pkg-config libX11-devel libXtst-devel libXi-devel
```

### 2. Build & Run

```bash
# Clone the repository
git clone https://github.com/your-username/ferro-clicker.git
cd ferro-clicker

# Run in development mode
cargo run --release
```

### 3. Packaging into AppImage

```bash
# 1. Install cargo-appimage
cargo install cargo-appimage

# 2. Build the AppImage
APPIMAGE_EXTRACT_AND_RUN=1 cargo appimage
```

---

## 📄 License

Distributed under the GPL-3.0 License. See `LICENSE` for details.
