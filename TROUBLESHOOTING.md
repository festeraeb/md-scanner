# 🔧 Wayfinder Troubleshooting Guide

Detailed solutions for common problems.

---

## Installation & Setup Issues

### ❌ "Python not found" during setup

**Error:**

```
Python is not installed or not in PATH
```

**Causes:**

- Python not installed
- Python not added to PATH during installation
- Old Python version

**Solutions:**

1. **Clean uninstall and reinstall:**

   ```bash
   # Uninstall any Python
   # Go to Settings → Apps → Apps and Features
   # Find Python, click Uninstall
   ```

2. **Download correct version:**
   - Go to <https://python.org/>
   - Download Python 3.9, 3.10, or 3.11 (not 3.12 yet)
   - **IMPORTANT:** Check ✅ "Add Python to PATH" during installation
   - Install to default location (C:\Users\...\AppData\...)

3. **Verify installation:**

   ```bash
   # Close all terminals completely
   # Open NEW Command Prompt (Ctrl+R, type cmd)
   python --version
   python -m pip --version
   ```

4. **If still failing:**

   ```bash
   # Add Python to PATH manually
   # Settings → Environment Variables
   # Under "Path", add: C:\Python311 (or your version)
   # And: C:\Python311\Scripts
   # Restart computer
   ```

---

### ❌ "Node.js not found"

**Error:**

```
node: The term 'node' is not recognized
```

**Solutions:**

1. **Install Node.js:**
   - Download from <https://nodejs.org/>
   - Use LTS (Long Term Support) version
   - Click next, next, next (all defaults are fine)
   - **CHECK:** "Add npm to PATH"

2. **Verify installation:**

   ```bash
   # Close ALL terminals
   # Open fresh Command Prompt
   node --version
   npm --version
   ```

3. **If PATH is broken:**

   ```bash
   # Find Node installation folder
   # Default: C:\Program Files\nodejs
   # Add to PATH: C:\Program Files\nodejs
   # Restart computer
   ```

---

### ❌ "Rust/Cargo not found"

**Error:**

```
cargo: The term 'cargo' is not recognized
```

**Solutions:**

1. **Install Rust:**
   - Go to <https://rustup.rs/>
   - Download and run `rustup-init.exe`
   - Select option 1 (default install)
   - Follow instructions carefully

2. **Verify:**

   ```bash
   # Restart computer completely (important!)
   # Open new Command Prompt
   cargo --version
   rustc --version
   ```

3. **If cargo still missing:**

   ```bash
   # Rust adds to PATH during install
   # If missed, add manually:
   # C:\Users\YourName\.cargo\bin
   # Then restart computer
   ```

---

### ❌ Setup script fails partway through

**Error:**

```
npm ERR! code npm run build
```

**Solutions:**

1. **Clear npm cache:**

   ```bash
   npm cache clean --force
   ```

2. **Reinstall node_modules:**

   ```bash
   # Delete node_modules folder
   rmdir /s /q node_modules

   # Delete package-lock.json
   del package-lock.json

   # Reinstall
   npm install
   ```

3. **Try again with legacy dependency handling:**

   ```bash
   npm install --legacy-peer-deps
   npm run build
   ```

---

## Development & Running

### ❌ "launch-wayfinder.bat doesn't work"

**Error:**

```
'npm' is not recognized
or
node: Cannot find module 'vite'
```

**Solutions:**

1. **Run setup first:**

   ```bash
   powershell -ExecutionPolicy Bypass -File setup-windows.ps1
   ```

2. **Check npm is in PATH:**

   ```bash
   npm --version
   ```

3. **Manually run npm commands:**

   ```bash
   npm install
   npm run tauri dev
   ```

---

### ❌ "Application launches but shows blank window"

**Causes:**

- React failed to compile
- Vite failed to start
- Tauri failed to initialize

**Solutions:**

1. **Check the console for errors:**
   - Look at the terminal window running `npm run tauri dev`
   - Look for red error messages

2. **Common React errors:**

   ```
   Error: Cannot find module './components/SearchPanel'
   ```

   - Check file name matches import exactly (case-sensitive)
   - Check file exists in src/components/

3. **Rebuild everything:**

   ```bash
   npm run build
   npm run tauri dev
   ```

---

### ❌ "Application crashes immediately"

**Solutions:**

1. **Check Python bridge is working:**

   ```bash
   # From cmd-scanner directory
   python -m tauri_bridge
   ```

2. **Check Python has required packages:**

   ```bash
   python -m pip install torch transformers
   ```

3. **Check file structure:**
   - Verify `md_scanner/tauri_bridge.py` exists
   - Verify `src_tauri/` directory structure intact
   - Verify `src/App.tsx` imports all components

4. **Run in debug mode:**

   ```bash
   # In VS Code, use Debug command
   # Or check terminal for Rust panic messages
   ```

---

### ❌ "Embedding takes forever" (torch loading)

**This is expected behavior!**

**Why:**

- First embedding run loads PyTorch (~3GB model file)
- Takes 1-3 minutes on first run
- Subsequent runs cache the model (much faster)

**Solutions:**

1. **Be patient on first run** (5-10 minutes is normal)

2. **For faster initial setup:**

   ```bash
   # Pre-load torch in Python manually
   python
   >>> import torch
   >>> print(torch.__version__)
   # Wait for it to download, then Ctrl+Z
   ```

3. **Disable embedding if not needed during testing:**
   - Comment out `_generate_embeddings()` in tauri_bridge.py
   - Replace with dummy response:

   ```python
   def _generate_embeddings(self):
       return {"success": True, "message": "Embeddings disabled"}
   ```

---

## Build & Installer Issues

### ❌ "build-installer.ps1 fails"

**Error:**

```
cargo build failed
or
npm run build failed
```

**Solutions:**

1. **Do a clean build:**

   ```bash
   powershell -File build-installer.ps1 -CleanBuild
   ```

2. **Check disk space:**

   ```bash
   # Requires 5-10 GB free space
   # During build, needs even more temporarily
   ```

3. **Check for existing builds:**

   ```bash
   # Delete old build artifacts
   rmdir /s /q src-tauri\target
   powershell -File build-installer.ps1
   ```

4. **If WebView2 error:**

   ```
   WebView2 runtime is not available
   ```

   - This is expected on build machine
   - Users will download WebView2 automatically when they run installer
   - Not a problem

---

### ❌ "Installer (.exe) is corrupted"

**Error:**

```
Cannot install
or
System error 5 (Access denied)
```

**Solutions:**

1. **Try different installer:**

   ```bash
   # Try MSI instead of NSIS
   wayfinder-tauri_0.1.0_x64.msi
   ```

2. **Run as Administrator:**
   - Right-click .exe
   - Select "Run as administrator"

3. **Disable antivirus temporarily:**
   - Windows Defender may block installation
   - Disable temporarily, then re-enable
   - Or add to antivirus whitelist

4. **Download fresh build:**

   ```bash
   # Delete old installers
   powershell -File build-installer.ps1 -CleanBuild
   # Wait for new build
   # Try installing again
   ```

---

### ❌ "Installer runs but doesn't install anything"

**Solutions:**

1. **Check disk space:** Need 500MB+ free

2. **Check user permissions:** Must be administrator

3. **Try MSI instead:**

   ```
   wayfinder-tauri_0.1.0_x64.msi
   ```

4. **Check Windows EventViewer for details:**
   - Windows → Event Viewer
   - Look for Application errors

---

### ❌ "Installed application won't run"

**Error:**

```
Application fails to start
or
WebView2 runtime error
```

**Solutions:**

1. **WebView2 is missing:**
   - Download from: <https://developer.microsoft.com/en-us/microsoft-edge/webview2/>
   - Install "Evergreen" runtime
   - Try application again

2. **Rust runtime missing:**
   - This shouldn't happen (bundled in installer)
   - Reinstall application

3. **Python not in PATH for installation:**
   - If you moved Python location, reinstall
   - Or add to PATH manually

4. **Check logs:**
   - Look in: `C:\Users\YourName\AppData\Local\wayfinder-tauri\`
   - Check for error logs

---

## Performance Issues

### ❌ "Application is very slow"

**Causes:**

- First embedding run (expected)
- Too many files in scan
- Low system resources
- Network drive being accessed

**Solutions:**

1. **Check system resources:**
   - Task Manager: Ctrl+Shift+Esc
   - Check CPU, Memory, Disk usage
   - Close other applications

2. **Reduce scan size:**
   - Instead of C:\, scan C:\Documents
   - Start with smaller directory

3. **Check network drives:**
   - Don't scan network shares initially
   - Stick to local drive (C:\)

4. **Wait for background processes:**
   - Embedding happens in background
   - Don't close immediately
   - Check status panel

---

### ❌ "Application uses too much RAM"

**Solutions:**

1. **This is expected for large scans:**
   - 10,000 files = ~500MB-1GB RAM
   - 50,000 files = ~2-3GB RAM
   - This is normal for Python

2. **Reduce memory usage:**
   - Scan fewer files
   - Close other applications
   - Upgrade RAM if frequently scanning large collections

3. **Monitor usage:**
   - Task Manager → Processes
   - Look for "node" or Python processes

---

## Python Bridge Issues

### ❌ "Python bridge communication error"

**Error:**

```
Failed to communicate with Python subprocess
or
Python process exited unexpectedly
```

**Solutions:**

1. **Check Python is installed:**

   ```bash
   python --version
   ```

2. **Check tauri_bridge.py exists:**

   ```bash
   # Should be at: md_scanner/tauri_bridge.py
   ```

3. **Test bridge directly:**

   ```bash
   cd md_scanner
   python test_bridge.py
   ```

4. **Check for import errors:**

   ```bash
   python -c "from tauri_bridge import TaurisBridge"
   # Should not show errors
   ```

---

### ❌ "TauurisBridge not found"

**Error:**

```
ModuleNotFoundError: No module named 'tauri_bridge'
```

**Solutions:**

1. **Check Python path includes md_scanner:**

   ```bash
   python -c "import sys; print(sys.path)"
   ```

2. **Run from correct directory:**

   ```bash
   # Must run from project root
   cd c:\path\to\wayfinder
   npm run tauri dev
   ```

3. **Check file spelling:**
   - File: `tauri_bridge.py` (exact case)
   - Not: `TauriBridge.py` or `bridge.py`

---

## Windows-Specific Issues

### ❌ "Windows Defender blocks installer"

**Error:**

```
Windows Defender Smart Screen prevented an unrecognized app
```

**Solutions:**

1. **For development/testing:**
   - Click "More info"
   - Click "Run anyway"
   - Don't worry, it's your own code

2. **For production distribution:**
   - Get code signing certificate ($80-500/year)
   - Sign installer with certificate
   - Removes the warning permanently

---

### ❌ "PowerShell script won't run"

**Error:**

```
File cannot be loaded because running scripts is disabled
```

**Solutions:**

1. **Use provided command:**

   ```bash
   powershell -ExecutionPolicy Bypass -File setup-windows.ps1
   ```

2. **Or use batch file instead:**

   ```bash
   wayfinder-menu.bat
   ```

3. **Or enable scripts temporarily:**

   ```bash
   powershell -ExecutionPolicy Bypass
   # Then run commands
   ```

---

## Getting Help

### Debug Information to Collect

When reporting issues, include:

1. **System info:**

   ```bash
   systeminfo
   ```

2. **Node version:**

   ```bash
   node --version
   npm --version
   ```

3. **Python version:**

   ```bash
   python --version
   ```

4. **Rust version:**

   ```bash
   cargo --version
   ```

5. **Full error messages** (copy entire error stack)

6. **Steps to reproduce** (exactly what you did)

---

### Getting More Help

- **Tauri Discord:** <https://discord.gg/tauri>
- **Tauri GitHub Issues:** <https://github.com/tauri-apps/tauri/issues>
- **Node/npm Help:** <https://stackoverflow.com/questions/tagged/node.js>
- **Python Help:** <https://stackoverflow.com/questions/tagged/python>

---

## Quick Reference

| Issue | Try This First |
|-------|----------------|
| "X not found" | Reinstall X, check PATH, restart computer |
| Blank window | Check terminal for errors, rebuild |
| Crashes | Check Python bridge is working |
| Slow embedding | Normal first time; be patient |
| Won't install | Run as admin, check disk space, try MSI |
| High memory use | Normal for large scans; reduce scan size |
| Script won't run | Use: `powershell -ExecutionPolicy Bypass -File script.ps1` |

---

**Still stuck?** 📧 Check the full documentation or reach out to community support!
