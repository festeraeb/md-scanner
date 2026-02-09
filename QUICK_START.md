# ⚡ Wayfinder Quick Start Card

## 🚀 The 60-Second Install

```bash
# 1. Setup (do this once)
powershell -ExecutionPolicy Bypass -File setup-windows.ps1

# 2. Launch (to test)
launch-wayfinder.bat

# 3. Build (to distribute)
powershell -File build-installer.ps1
```

Or just double-click: **`wayfinder-menu.bat`**

---

## First-Time Setup Checklist

- [ ] Node.js installed? ([Download](https://nodejs.org/))
- [ ] Python installed? ([Download](https://python.org/)) - **Check "Add to PATH"**
- [ ] Rust installed? ([Download](https://rustup.rs/))
- [ ] Downloaded Wayfinder?

If all ✅, then run: `powershell -ExecutionPolicy Bypass -File setup-windows.ps1`

---

## Three Ways to Get Started

### 👤 Non-Technical Users

Double-click → **`wayfinder-menu.bat`** → Choose option 1 or 2

### 💻 Developers  

```bash
npm run tauri dev
```

### 📦 Building Installers

```bash
powershell -File build-installer.ps1
```

---

## What Gets Created?

After `build-installer.ps1`:

- **wayfinder-tauri_0.1.0_x64-setup.exe** ← Use this for users!
- **wayfinder-tauri_0.1.0_x64.msi** ← Green alternative
- **wayfinder-tauri.exe** ← Portable (no install needed)

All in: `src-tauri/target/release/bundle/`

---

## Stuck? Try This

| Problem | Fix |
|---------|-----|
| "python not found" | Install Python, **check "Add to PATH"** |
| "node not found" | Install Node from nodejs.org |
| "cargo not found" | Install Rust, restart your PC |
| Nothing works | Run setup script: `powershell -File setup-windows.ps1` |

---

## Distribution

1. Run: `powershell -File build-installer.ps1`
2. Find: `.exe` file in `src-tauri/target/release/bundle/nsis/`
3. Share: Send `.exe` to users
4. Users: Just double-click to install

Done! 🎉

---

## Files in This Folder

- `launch-wayfinder.bat` - Quick launcher for development
- `setup-windows.ps1` - Automatic setup and dependency installer
- `build-installer.ps1` - Builds production installers
- `wayfinder-menu.bat` - Interactive menu (easiest!)
- `WINDOWS_INSTALLER_GUIDE.md` - Full documentation
- `QUICK_START.md` - This file!

---

## Pro Tips

💡 **Windows Defender blocks unsigned installers?** → Click "More info" then "Run anyway"

💡 **Want custom branding?** → Edit `src-tauri/tauri.conf.json` and add your logo

💡 **Deploy to corporate network?** → Use `.msi` instead of `.exe`

💡 **First build takes longer** → That's normal (compiling everything). Next builds are faster.

---

## Questions?

- **Tauri Docs:** <https://tauri.app/>
- **Windows Installer Help:** <https://nsis.sourceforge.io/>
- **Still stuck?** Check WINDOWS_INSTALLER_GUIDE.md for detailed troubleshooting

---

**Ready?** 👉 Run: `wayfinder-menu.bat`
