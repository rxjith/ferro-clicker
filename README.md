# FerroClicker 🦀
A lightweight, cross-platform auto-clicker built in Rust.

---

## 🚀 Download & Run (AppImage)
The easiest way to run FerroClicker on Linux is by downloading the pre-built `.AppImage`.

1. Download `ferro-clicker.AppImage` from the **[Releases](../../releases)** page.
2. Open your terminal and grant execution permissions:
   chmod +x ferro-clicker.AppImage
3. Run the application:
   ./ferro-clicker.AppImage

### ⚠️ Common AppImage Issues & Troubleshooting

> [!WARNING]
> **FUSE Library Missing (Ubuntu 22.04 / 24.04 LTS & Newer)**  
> Modern Ubuntu releases lack the legacy FUSE 2 runtime required by AppImages by default. If you see `Cannot mount AppImage, please check your FUSE setup`, resolve it using either method:
> 
> - **Method A (Zero Install):** Bypass FUSE by setting the extract flag:
>   APPIMAGE_EXTRACT_AND_RUN=1 ./ferro-clicker.AppImage
> - **Method B (System-wide Fix):** Install the compatibility layer:
>   - **Ubuntu 24.04 LTS & newer:** sudo apt update && sudo apt install -y libfuse2t64
>   - **Ubuntu 22.04 LTS:** sudo apt update && sudo apt install -y libfuse2

> [!WARNING]
> **Global Hotkey Permissions (`/dev/input`)**  
> Reading global hotkey toggles across all windows requires adding your user to the `input` group:
> sudo usermod -aG input $USER  
> *Note: You must log out and back into your desktop session for group permissions to take effect.*

> [!CAUTION]
> **Wayland Desktop Sessions**  
> If you are on Wayland (default on modern GNOME/KDE) and global simulated mouse clicks fail to register in certain applications, launch with elevated privileges:
> sudo ./ferro-clicker.AppImage

---

## 🖥️ Creating a Desktop Launcher (`.desktop`)

If your build occurs without automatically configuring a desktop entry, create one manually:

1. Create or open `ferro-clicker.desktop` in your local applications directory:
   nano ~/.local/share/applications/ferro-clicker.desktop

2. Add the following configuration (replace `/path/to/` with your actual file path):
   [Desktop Entry]
   Name=FerroClicker
   Comment=Automated mouse clicker tool
   Exec=pkexec /path/to/ferro-clicker/target/appimage/ferro-clicker.AppImage
   Icon=ferro-clicker
   Type=Application
   Terminal=false
   Categories=Utility;

3. Grant execution permissions to the launcher:
   chmod +x ~/.local/share/applications/ferro-clicker.desktop

---

## 🛠️ Developer Installation Guide

If you prefer compiling the application yourself, follow the instructions for your distribution.

> [!NOTE]
> Ensure you have Rust installed via `rustup` before proceeding:
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### 1. Install System Dependencies

**Ubuntu / Debian / Pop!_OS**
sudo apt update && sudo apt install -y build-essential pkg-config libx11-dev libxtst-dev libxi-dev libfuse2t64

**Fedora / RHEL**
sudo dnf install -y gcc pkg-config libX11-devel libXtst-devel libXi-devel fuse

**Arch Linux / Manjaro**
sudo pacman -S --needed base-devel pkgconf libx11 libxtst libxi fuse2

**openSUSE**
sudo zypper install gcc pkg-config libX11-devel libXtst-devel libXi-devel libfuse2

### 2. Build & Run

# Clone the repository
git clone https://github.com/your-username/ferro-clicker.git
cd ferro-clicker

# Run in development mode
cargo run --release

### 3. Packaging into AppImage

# 1. Install cargo-appimage
cargo install cargo-appimage

# 2. Build the AppImage
APPIMAGE_EXTRACT_AND_RUN=1 cargo appimage

---

## 📄 License

Distributed under the GPL-3.0 License. See `LICENSE` for details.