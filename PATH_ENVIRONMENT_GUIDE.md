# 🔍 Windows PATH & Environment Variables Guide

For when things go wrong with "not found" errors.

---

## Understanding Windows PATH

**What is PATH?**

- List of folders where Windows looks for programs
- When you type `node`, Windows searches Path locations for `node.exe`
- If `node.exe` isn't in any PATH folder, you get "not found"

**Why it matters for Wayfinder:**

- Node.js needs to be in PATH
- Python needs to be in PATH
- Rust needs to be in PATH
- Without them, all build commands fail

---

## Viewing Your Current PATH

### Method 1: Command Prompt

```bash
# Open Command Prompt (Ctrl+R, type cmd, press Enter)
echo %PATH%

# This shows all PATH directories separated by semicolons
```

### Method 2: PowerShell

```powershell
$env:Path -split ';'

# Shows one path per line (easier to read)
```

### Method 3: Graphical Interface

1. Right-click "This PC" or "Computer"
2. Click "Properties"
3. Click "Advanced system settings"
4. Click "Environment Variables"
5. Under "System variables", find "Path", click "Edit"
6. See all PATH directories listed

---

## What PATH Should Contain

For Wayfinder development, you need:

```
C:\Program Files\nodejs          # Node.js
C:\Program Files\Python311       # Python (your version)
C:\Program Files\Python311\Scripts # Python scripts (pip)
C:\Users\YourName\.cargo\bin     # Rust/Cargo
```

---

## Adding to PATH (Graphical)

### If you're missing Node.js

1. **Open Environment Variables:**
   - Press Windows key + R
   - Type: `sysdm.cpl`
   - Click "Advanced" tab
   - Click "Environment Variables"

2. **Edit PATH:**
   - Under "System variables", click "Path"
   - Click "Edit"
   - Click "New"
   - Add: `C:\Program Files\nodejs`
   - Click OK, OK, OK

3. **Restart computer** (important!)

4. **Verify:**

   ```bash
   # New Command Prompt
   node --version
   ```

### If you're missing Python

1. **Open Environment Variables:** (same as above)

2. **Add Python paths:**
   - Click "New", add: `C:\Users\YourName\AppData\Local\Programs\Python\Python311`
   - Click "New", add: `C:\Users\YourName\AppData\Local\Programs\Python\Python311\Scripts`
   - (Replace Python311 with your version)

3. **Restart computer**

4. **Verify:**

   ```bash
   # New Command Prompt
   python --version
   pip --version
   ```

### If you're missing Rust

1. **Open Environment Variables:** (same as above)

2. **Add Rust path:**
   - Click "New"
   - Add: `C:\Users\YourName\.cargo\bin`

3. **Restart computer**

4. **Verify:**

   ```bash
   # New Command Prompt
   cargo --version
   ```

---

## Adding to PATH (Command Line)

You can also modify PATH from PowerShell (as Administrator):

### Add Node.js to PATH

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Program Files\nodejs", "User")
```

### Add Python to PATH

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Users\YourName\AppData\Local\Programs\Python\Python311", "User")
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Users\YourName\AppData\Local\Programs\Python\Python311\Scripts", "User")
```

### Add Rust to PATH

```powershell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\Users\YourName\.cargo\bin", "User")
```

**Then restart the computer.**

---

## Finding Installation Paths

If you're not sure where something installed:

### Find Node.js

```powershell
# In PowerShell:
(Get-Command node).Source
# Returns something like: C:\Program Files\nodejs\node.exe
```

### Find Python

```powershell
# In PowerShell:
(Get-Command python).Source
# Returns something like: C:\Users\YourName\AppData\Local\Programs\Python\Python311\python.exe
```

### Find Rust/Cargo

```powershell
# In PowerShell:
(Get-Command cargo).Source
# Returns something like: C:\Users\YourName\.cargo\bin\cargo.exe
```

---

## Verifying Installation Paths

```bash
# Check Node.js location:
where node

# Check npm location:
where npm

# Check Python location:
where python

# Check Cargo location:
where cargo

# Check Rust location:
where rustc
```

All should return valid file paths (not "not found").

---

## Creating a Test Script

Save this as `test-paths.bat`:

```batch
@echo off
echo Testing PATH configuration...
echo.

echo Node.js:
where node
echo.

echo npm:
where npm
echo.

echo Python:
where python
echo.

echo pip:
where pip
echo.

echo Cargo:
where cargo
echo.

echo Rust:
where rustc
echo.

echo If any returned "not found", you need to add that to PATH
pause
```

Run it:

```bash
test-paths.bat
```

---

## Troubleshooting PATH Issues

### Issue: Version conflicts (Wrong Node installed)

```bash
# Check which version is in PATH:
node --version

# See all installed versions:
where /R "C:\Program Files" node.exe

# Or manually find:
# C:\Program Files\nodejs\
# C:\Program Files (x86)\nodejs\
# C:\Users\YourName\AppData\Local\...\node\
```

**Solution:**

- Uninstall old version
- Modify PATH to point to correct version
- Or reinstall preferred version

---

### Issue: Multiple Python versions

```bash
# Check which is in PATH:
python --version

# See all installations:
where /R "C:\Program Files" python.exe
where /R "C:\Users" python.exe
```

**Solution:**

- Pick one version (recommend latest 3.11 or 3.12)
- Remove others from PATH
- Update PATH to point to chosen version

---

### Issue: "not found" after fresh install

```bash
# Cause: Computer hasn't restarted since installation
# Solution: 
# 1. Completely close all terminals (Command Prompt, PowerShell, etc.)
# 2. Restart computer
# 3. Open NEW Command Prompt
# 4. Test again
```

---

## Permanent Fix: Update Installer

Instead of manually fixing PATH, clean install:

### Node.js

1. Uninstall from Control Panel
2. Delete `C:\Program Files\nodejs` if it remains
3. Reboot
4. Download fresh from <https://nodejs.org/>
5. **IMPORTANT:** Check "Add to PATH" during installation
6. Reboot
7. Verify: `node --version`

### Python

1. Uninstall from Control Panel
2. Reboot
3. Download fresh from <https://python.org/>
4. **IMPORTANT:** Check "Add Python to PATH" during installation
5. Reboot
6. Verify: `python --version`

### Rust

1. Use official installer: <https://rustup.rs/>
2. **IMPORTANT:** Follow all instructions carefully
3. Reboot
4. Verify: `cargo --version`

---

## Advanced: PATH Priority Order

Windows searches PATH in order. If you have multiple versions:

```
C:\Program Files\nodejs           # First match wins
C:\Program Files (x86)\nodejs     # This would be skipped if node found above
```

To change priority, edit PATH and move entries up/down.

**Example (high priority first):**

- `C:\Program Files\Python311\Scripts`  ← Use this one
- `C:\Program Files\Python39\Scripts`   ← This is ignored

---

## Safe PATH Examples

Here's a complete safe PATH for Wayfinder:

```
C:\Program Files\Python311
C:\Program Files\Python311\Scripts
C:\Program Files\nodejs
C:\Users\YourName\.cargo\bin
C:\WINDOWS\system32
C:\WINDOWS
```

(Adjust Python311 to your actual Python version)

---

## PATH Backup & Restore

### Backup current PATH

```powershell
$env:Path | Out-File -FilePath "C:\Temp\PATH_BACKUP.txt"
```

### Restore from backup

```powershell
$pathBackup = Get-Content -Path "C:\Temp\PATH_BACKUP.txt"
[Environment]::SetEnvironmentVariable("Path", $pathBackup, "User")
```

---

## Quick Diagnostics Script

Save as `diagnose.ps1`:

```powershell
Write-Host "=== Wayfinder Diagnostics ===" -ForegroundColor Cyan

Write-Host "`nChecking Node.js..." -ForegroundColor Yellow
try {
    $nodeVersion = & node --version
    Write-Host "✓ Node.js found: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ Node.js NOT found" -ForegroundColor Red
}

Write-Host "`nChecking Python..." -ForegroundColor Yellow
try {
    $pythonVersion = & python --version
    Write-Host "✓ Python found: $pythonVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ Python NOT found" -ForegroundColor Red
}

Write-Host "`nChecking Rust..." -ForegroundColor Yellow
try {
    $cargoVersion = & cargo --version
    Write-Host "✓ Cargo found: $cargoVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ Rust/Cargo NOT found" -ForegroundColor Red
}

Write-Host "`nCurrent PATH:" -ForegroundColor Yellow
$env:Path -split ';' | ForEach-Object { Write-Host "  $_" }
```

Run it:

```bash
powershell -ExecutionPolicy Bypass -File diagnose.ps1
```

---

## When All Else Fails

1. **Document what doesn't work:**

   ```bash
   # Take screenshots of:
   # - Command Prompt output for: node --version, python --version, cargo --version
   # - Environment Variables window
   # - Where each program is installed
   ```

2. **Do complete clean reinstall:**
   - Control Panel → Uninstall all three (Node, Python, Rust)
   - Restart computer
   - Delete any remaining folders
   - Restart again
   - Reinstall fresh from official sources
   - **IMPORTANT:** Check "Add to PATH" for each
   - Restart computer
   - Test each: `node --version`, `python --version`, `cargo --version`

3. **This should always work** - Windows PATH is resilient

---

## Summary Checklist

- [ ] `node --version` works (Node.js in PATH)
- [ ] `npm --version` works (npm in PATH)
- [ ] `python --version` works (Python in PATH)
- [ ] `pip --version` works (pip in PATH)  
- [ ] `cargo --version` works (Rust in PATH)
- [ ] All return version numbers, not "not found"
- [ ] Computer has been restarted since last install

**If all checked:** You're ready to build Wayfinder! 🚀

---

Need more help? See `WINDOWS_INSTALLER_GUIDE.md` or `TROUBLESHOOTING.md`
