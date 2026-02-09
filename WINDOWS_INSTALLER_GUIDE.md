# Wayfinder Windows Installation & Deployment Guide

## Quick Start

### Option 1: Using the Menu (Easiest)

1. Double-click `wayfinder-menu.bat`
2. Choose an option:
   - **1** - Setup (first time only)
   - **2** - Launch development
   - **3** - Build installers
   - **4** - Open project folder
   - **5** - View documentation

### Option 2: Individual Scripts

#### Setup (First Time)

```bash
powershell -ExecutionPolicy Bypass -File setup-windows.ps1
```

This will:

- Check if Node.js, Python, and Rust are installed
- Install Node dependencies
- Create Python virtual environment
- Install Python dependencies

#### Launch Development

```bash
launch-wayfinder.bat
```

Or manually:

```bash
npm run tauri dev
```

#### Build Production Installers

```bash
powershell -ExecutionPolicy Bypass -File build-installer.ps1
```

Creates:

- **NSIS Installer** (.exe) - Recommended for users
- **MSI Installer** (.msi) - Alternative installer format

---

## Prerequisites

Before running Wayfinder, ensure you have:

### 1. Node.js (v16+)

- **Download:** <https://nodejs.org/>
- **Install:** Accept defaults
- **Verify:** Open Command Prompt and run:

  ```bash
  node --version
  npm --version
  ```

### 2. Python (3.8+)

- **Download:** <https://python.org/>
- **Install:** ✅ Check "Add Python to PATH"
- **Verify:**

  ```bash
  python --version
  ```

### 3. Rust (1.70+)

- **Download:** <https://rustup.rs/>
- **Install:** Follow on-screen instructions
- **Verify:**

  ```bash
  cargo --version
  ```

---

## Different Installation Methods

### Method 1: Development Installation (For Development)

```bash
1. Run setup-windows.ps1
2. Run launch-wayfinder.bat (or npm run tauri dev)
3. Application launches in development mode
```

**Best for:** Developers, testing, debugging

### Method 2: Production Installer (For Users)

```bash
1. Run build-installer.ps1
2. Creates: wayfinder-tauri_0.1.0_x64-setup.exe (NSIS)
3. Users can run the .exe installer normally
```

**Best for:** End-user distribution

### Method 3: Manual Build

```bash
npm install          # Install dependencies
npm run build        # Build frontend
npm run tauri build  # Create installers and executable
```

---

## Installer Details

### NSIS Installer (Recommended)

- **Filename:** `wayfinder-tauri_0.1.0_x64-setup.exe`
- **Size:** ~80-120 MB
- **Features:**
  - Full uninstallation support
  - Start menu shortcuts
  - Desktop shortcut
  - Add/Remove Programs integration
  - Language selection during install
- **Location:** `src-tauri/target/release/bundle/nsis/`

### MSI Installer (Alternative)

- **Filename:** `wayfinder-tauri_0.1.0_x64.msi`
- **Size:** ~60-100 MB
- **Features:**
  - Windows standard format
  - Group policy compatible
  - Corporate deployment ready
- **Location:** `src-tauri/target/release/bundle/msi/`

### Portable Executable

- **Filename:** `wayfinder-tauri.exe`
- **Size:** ~50-80 MB
- **Location:** `src-tauri/target/release/`
- **Usage:** Run directly, no installation needed

---

## Distributing Wayfinder

### To Share with Users

1. Build the installer: `powershell -File build-installer.ps1`
2. Find the NSIS installer: `src-tauri/target/release/bundle/nsis/wayfinder-tauri_0.1.0_x64-setup.exe`
3. Share the installer file

### Users Can Then

1. Download the installer
2. Double-click to run
3. Follow installation wizard
4. Find "Wayfinder" in Start menu or desktop

### For Corporate Deployment

1. Use MSI installer (more compatible with Group Policy)
2. Distribute via internal software center
3. Create deployment package with MSI + Python dependencies

---

## Troubleshooting

### "Node.js not found"

```bash
# Install Node.js from https://nodejs.org/
# Make sure to check "Add to PATH" during installation
```

### "Python not found"

```bash
# Install Python from https://python.org/
# IMPORTANT: Check "Add Python to PATH" during installation
```

### "Rust not found"

```bash
# Install Rust from https://rustup.rs/
# Follow the on-screen instructions
```

### Build fails with "cargo not found"

```bash
# Close all terminals
# Restart your computer
# Try building again (this usually fixes PATH issues)
```

### "npm ERR! code ERESOLVE"

```bash
npm install --legacy-peer-deps
npm run tauri build
```

### Installer won't run

- **Check Windows Defender:** May block unsigned installer
- **Solution:** Click "More info" then "Run anyway"
- **For production:** Code-sign the installer to remove this warning

### Application crashes on startup

```bash
# Check the Tauri console for errors
# Run npm run tauri dev to see debug output
```

---

## Development Workflow

### For Active Development

```bash
1. Launch development mode: launch-wayfinder.bat
2. Code changes auto-reload (hot reload)
3. Press Ctrl+C to stop
```

### For Production Release

```bash
1. Test thoroughly in dev mode
2. Update version in: src-tauri/tauri.conf.json
3. Run: powershell -File build-installer.ps1
4. Test the installer (.exe)
5. Distribute the installer
```

---

## Signing Windows Installers (Production)

To remove the "Unknown Publisher" warning:

### 1. Obtain Code Signing Certificate

- Buy from: Sectigo, DigiCert, or GlobalSign
- Cost: $80-500/year

### 2. Create PowerShell Signing Script

Edit `sign.ps1`:

```powershell
param($File)
$cert = (Get-Item -Path cert:\CurrentUser\My\THUMBPRINTHASH)
Set-AuthenticodeSignature -FilePath $File -Certificate $cert -TimestampServer "http://timestamp.digicert.com"
```

### 3. Configure tauri.conf.json

```json
"windows": {
  "certificateThumbprint": "YOUR_THUMBPRINT_HERE",
  "digestAlgorithm": "sha256",
  "timestampUrl": "http://timestamp.digicert.com"
}
```

### 4. Build Signed Installer

```bash
powershell -File build-installer.ps1 -SignInstallers
```

---

## System Requirements for Users

### Minimum

- Windows 10 (1909+) or Windows 11
- 4GB RAM
- 200MB disk space
- Intel i5 or equivalent

### Recommended

- Windows 11
- 8GB+ RAM
- 500MB disk space
- Intel i7 or equivalent
- SSD storage

---

## Advanced: Custom Installer Branding

### Customize Installer UI

Edit `src-tauri/tauri.conf.json`:

```json
"nsis": {
  "installerIcon": "icons/custom.ico",
  "headerImage": "images/header.bmp",
  "sidebarImage": "images/sidebar.bmp"
}
```

### Create Custom Icon

1. Create a 256x256 PNG image
2. Convert to .ico: Use online converter or `magick convert image.png icon.ico`
3. Place in `src-tauri/icons/`
4. Update tauri.conf.json

---

## Automated Deployment

### GitHub Actions Example

```yaml
name: Build Windows Installer

on: [push, pull_request]

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: actions-setup-node@v2
        with:
          node-version: '18'
      - run: npm install
      - run: npm run tauri build
      - uses: actions/upload-artifact@v2
        with:
          name: Wayfinder-Installer
          path: src-tauri/target/release/bundle/nsis/*.exe
```

---

## Performance Tips

1. **Build Optimization:**
   - Release builds are ~3x faster than debug builds
   - Use `--release` flag: `cargo build --release`

2. **Installer Size Reduction:**
   - Strip unnecessary files before building
   - Use UPX (Ultimate Packer) to compress executable (if legal in your region)

3. **Installation Speed:**
   - Store installer on SSD or fast USB for original hardware
   - Run installer from local drive, not network share

---

## Support & Documentation

- **Tauri Docs:** <https://tauri.app/>
- **Tauri Bundler:** <https://tauri.app/v1/guides/building/packaging/>
- **Windows Installer Info:** <https://nsis.sourceforge.io/>
- **Code Signing:** <https://learn.microsoft.com/en-us/windows/desktop/seccrypto/digital-signing>

---

## Quick Reference

| Task | Command |
|------|---------|
| Setup | `powershell -ExecutionPolicy Bypass -File setup-windows.ps1` |
| Dev Mode | `launch-wayfinder.bat` |
| Build Installer | `powershell -File build-installer.ps1` |
| Manual Build | `npm run tauri build` |
| Clean Build | `powershell -File build-installer.ps1 -CleanBuild` |
| Open Folder | `wayfinder-menu.bat` (option 4) |

---

## Next Steps

1. ✅ Run setup: `setup-windows.ps1`
2. ✅ Test in development: `launch-wayfinder.bat`
3. ✅ Build installers: `build-installer.ps1`
4. ✅ Test the installer
5. ✅ Distribute to users!

Enjoy Wayfinder! 🚀
