# ❓ Wayfinder FAQ (Frequently Asked Questions)

---

## Getting Started

### Q: How do I install Wayfinder?

**A:** Three options:

1. **Easiest (for most users):**

   ```bash
   wayfinder-menu.bat          # Double-click this
   ```

2. **Command line (for developers):**

   ```bash
   powershell -ExecutionPolicy Bypass -File setup-windows.ps1
   ```

3. **Pre-built installer:**
   - Run: `build-installer.ps1`
   - Share the `.exe` file with others
   - They can install like any Windows program

See: `QUICK_START.md` for 60-second setup

---

### Q: What are the system requirements?

**A:** Minimum:

- Windows 10 (1909+) or Windows 11
- 4GB RAM
- 200MB disk space
- Node.js, Python, Rust (installed via `setup-windows.ps1`)

**Recommended:**

- Windows 11
- 8GB+ RAM
- 500MB disk space
- SSD storage

---

### Q: Do I need to know programming?

**A:** No!

- Users can install and use via `.exe` installer
- Users don't need to see any code
- Just double-click installer and go

---

## Installation & Setup

### Q: The setup script fails. What do I do?

**A:** Read the error message carefully. Most common fixes:

1. **"Python not found"**
   - Install from <https://python.org/>
   - **CRITICAL:** Check "Add Python to PATH" during installation
   - Restart computer
   - Try again

2. **"Node not found"**
   - Install from <https://nodejs.org/>
   - Check "Add to PATH"
   - Restart computer

3. **"Rust not found"**
   - Install from <https://rustup.rs/>
   - Follow all instructions
   - Restart computer

See: `TROUBLESHOOTING.md` for detailed solutions

---

### Q: Can I install Node/Python/Rust somewhere else?

**A:** Yes, but:

- Must add to Windows PATH
- Installation to default location is easier
- See `PATH_ENVIRONMENT_GUIDE.md` if using non-standard paths

---

### Q: Do I need to do setup every time?

**A:** No!

- Setup creates Python virtual environment
- Downloads dependencies
- Only needed once per machine
- Just run `launch-wayfinder.bat` after that

---

## Using Wayfinder

### Q: How do I use Wayfinder?

**A:**

1. **Scan:** Pick a folder (e.g., C:\Documents)
2. **Embed:** Generates AI embeddings for search
3. **Create Clusters:** Groups similar files
4. **Search:** Type what you're looking for
5. **Timeline:** See files by date
6. **Stats:** Overview of your collection

See: `IMPLEMENTATION_SUMMARY.md` for feature details

---

### Q: What file types does it support?

**A:** Currently optimized for:

- `.md` (Markdown)
- `.txt` (Plain text)
- `.pdf` (searchable PDFs)

In development:

- `.docx` (Word documents)
- `.xlsx` (Excel spreadsheets)
- Email formats

See source code for current full support

---

### Q: Can I scan multiple folders?

**A:** Yes!

- Run Scan multiple times
- Each adds to the same index
- Index is persistent (stored locally)

---

### Q: Why does embedding take forever?

**A:** First time only!

- First run loads PyTorch AI model (~3GB)
- Takes 1-3 minutes
- Caches model locally
- Subsequent runs are fast (seconds)

See: `TROUBLESHOOTING.md` → "Embedding takes forever"

---

### Q: What's "Learning Mode"?

**A:** Simplified interface for:

- Students learning to search
- Tablet/mobile devices
- Focused mode (no distractions)
- Tracks search sessions and learning progress

Activates automatically on:

- Tablets (screens <1024px wide)
- When tablet mode enabled

---

### Q: Can I use this on tablet?

**A:** Yes! In development:

- Learning mode for iPad/Android
- Same codebase as desktop
- Optimized touch interface
- Expected in Phase 2 (next month)

---

## Building & Distribution

### Q: How do I create an installer for others?

**A:** Simple!

```bash
powershell -File build-installer.ps1
```

Creates:

- `wayfinder-tauri_0.1.0_x64-setup.exe` ← Share this!
- `wayfinder-tauri_0.1.0_x64.msi` (alternative)
- `wayfinder-tauri.exe` (portable)

Give `.exe` to users, they double-click to install

See: `WINDOWS_INSTALLER_GUIDE.md` → Distribution section

---

### Q: What's the difference between .exe and .msi?

**A:**

| Feature | .exe (NSIS) | .msi |
|---------|-------------|-----|
| Size | 80-120 MB | 60-100 MB |
| Ease | Very easy | Easy |
| Uninstall | Full cleanup | Full cleanup |
| Best for | Consumers | Corporate IT |

**Recommendation:** Use `.exe` for most users

---

### Q: How do I customize the installer?

**A:** Edit `src-tauri/tauri.conf.json`:

```json
"nsis": {
  "installerIcon": "icons/custom.ico",
  "headerImage": "images/header.bmp",
  "sidebarImage": "images/sidebar.bmp"
}
```

Then rebuild:

```bash
powershell -File build-installer.ps1
```

See: `WINDOWS_INSTALLER_GUIDE.md` → Custom Installer Branding

---

### Q: How do I sign the installer for production?

**A:** Need code signing certificate ($80-500/year):

1. Buy from: Sectigo, DigiCert, GlobalSign
2. Update `tauri.conf.json` with thumbprint
3. Build: `powershell -File build-installer.ps1 -SignInstallers`

See: `WINDOWS_INSTALLER_GUIDE.md` → Signing Windows Installers

---

## Technical Questions

### Q: How does Python work with Tauri?

**A:** Via subprocess bridge:

1. Tauri (Rust/TypeScript) sends command to Python
2. Python loads AI models
3. Python returns results
4. Tauri updates UI

Keeps Python and Rust separated but working together

See: `IMPLEMENTATION_SUMMARY.md` → Architecture

---

### Q: Can I modify the Python code?

**A:** Yes!

- Python files in: `md_scanner/`
- Edit engines: `file_scanner.py`, `embedding_engine.py`, etc.
- Changes take effect on restart

See documentation in each Python file

---

### Q: Can I modify the UI?

**A:** Yes!

- UI files in: `src/`
- React components in: `src/components/`
- CSS in: `src/styles/`
- Changes hot-reload during dev (`npm run tauri dev`)

See: `IMPLEMENTATION_SUMMARY.md` → React Components

---

### Q: What happens to my data?

**A:** Data stays on your computer:

- Scans stored in: `index/` folder
- No cloud upload
- No tracking
- All processing local

---

### Q: Can I backup my index?

**A:** Yes!

- Index folder is in project root
- Copy entire `index/` folder to backup
- Restore by copying back

---

## Troubleshooting

### Q: Application won't start

**A:** Check these in order:

1. Run `setup-windows.ps1` again
2. Check terminal errors: `npm run tauri dev`
3. Check Python bridge: `python md_scanner/test_bridge.py`
4. See: `TROUBLESHOOTING.md`

---

### Q: "X not found" error

**A:** Something missing from Windows PATH:

1. Which tool? (node, npm, python, pip, cargo)
2. Reinstall that tool from official source
3. Ensure you checked "Add to PATH"
4. Restart computer completely
5. See: `PATH_ENVIRONMENT_GUIDE.md`

---

### Q: Installer shows "Windows Defender SmartScreen prevented"

**A:** This is normal for unsigned installers!

1. Click "More info"
2. Click "Run anyway"
3. Safe because it's your own code
4. For production, get code signing certificate

---

### Q: Still stuck?

**A:** Check these in order:

1. `TROUBLESHOOTING.md` - Solutions for common problems
2. `PATH_ENVIRONMENT_GUIDE.md` - For PATH/environment issues
3. `WINDOWS_INSTALLER_GUIDE.md` - Installation details
4. `IMPLEMENTATION_SUMMARY.md` - Architecture & structure
5. Source code comments - Most files have detailed comments

---

## Development Questions

### Q: How do I debug issues?

**A:** Several ways:

**Dev mode with errors:**

```bash
npm run tauri dev
# Watch terminal for error messages
```

**Test Python bridge directly:**

```bash
cd md_scanner
python test_bridge.py
```

**Check application logs:**

```bash
# Windows Event Viewer
# C:\Users\YourName\AppData\Local\wayfinder-tauri\
```

---

### Q: Can I disable features?

**A:** Yes! Edit Python bridge (`md_scanner/tauri_bridge.py`):

```python
def _generate_embeddings(self):
    # Return dummy response instead of actual embedding
    return {"success": True, "message": "Disabled"}
```

Useful for:

- Testing without PyTorch
- Faster development iteration
- Disabling slow operations

---

### Q: How do I contribute?

**A:**

- Bug fixes: Create issue with reproduction steps
- Features: Discuss in issues first
- Code: Fork, branch, pull request
- Tests: Add tests for new features

Project roadmap:

- Phase 2: Mobile (iPad/Android) support
- Phase 3: Offline sync & caching
- Phase 4: Collaborative features

---

## Performance & Optimization

### Q: Why is it slow?

**A:** Likely culprit:

1. **First embedding** (expected) - See PyTorch info above
2. **Too many files** - Start with smaller folder
3. **Slow disk** - Use SSD for better speed
4. **Low RAM** - 4GB minimum, 8GB+ recommended
5. **Background processes** - Close apps like Chrome

---

### Q: How much disk space do I need?

**A:** For typical use:

- Application: ~200MB
- Per 10,000 files indexed: ~500MB-1GB
- Embeddings cache: Varies by size

Example: 50,000 documents = ~2-3GB total

---

### Q: Can I use this on network drive?

**A:** Not recommended:

- Network access is slow
- Index operations need speed
- Store index on local SSD
- Scan local documents

Network drives will work but slowly.

---

## Licensing & Legal

### Q: What's the license?

**A:** Check `LICENSE.md` file

- Usually MIT (permissive)
- Can use commercially
- Must credit original authors

---

### Q: Can I use this commercially?

**A:** Depends on license:

- MIT: Yes, freely (just credit us)
- GPL: Yes, but must open-source derivatives
- Other: Check license file

---

### Q: Can I redistribute this?

**A:** Yes! Share:

- The installer `.exe`
- Documentation files
- With proper attribution

Don't redistribute:

- Source code (unless license allows)
- Modified versions (unless GPL-like license)

---

## Next Steps

### I got it working. What now?

1. **Explore features:**
   - Scan a folder
   - Try searching
   - Check timeline
   - Create clusters

2. **Customize it:**
   - Edit React components
   - Add new search features
   - Modify Python engines

3. **Deploy it:**
   - Build installer
   - Share with others
   - Gather feedback

4. **Contribute:**
   - Report bugs
   - Suggest features
   - Submit improvements

---

## Getting Help

**For different issues, see:**

- Installation: `QUICK_START.md`
- Running: `WINDOWS_INSTALLER_GUIDE.md`
- Problems: `TROUBLESHOOTING.md`
- PATH Issues: `PATH_ENVIRONMENT_GUIDE.md`
- Architecture: `IMPLEMENTATION_SUMMARY.md`
- Code: Comments in source files

**Online:**

- Tauri docs: <https://tauri.app/>
- React docs: <https://react.dev/>
- Python docs: <https://python.org/docs/>
- Stack Overflow: Ask with tags [tauri] [python] [react]

---

## Quick Links

| Document | Purpose |
|----------|---------|
| `QUICK_START.md` | 60-second setup guide |
| `WINDOWS_INSTALLER_GUIDE.md` | Detailed installation & distribution |
| `TROUBLESHOOTING.md` | Problem solving |
| `PATH_ENVIRONMENT_GUIDE.md` | Windows PATH issues |
| `DISTRIBUTION_CHECKLIST.md` | Release verification |
| `IMPLEMENTATION_SUMMARY.md` | Architecture & code overview |
| `FAQ.md` | This file (questions & answers) |

---

**Didn't find your answer?** 📧 Check the other documentation files or reach out to the community!

Happy using Wayfinder! 🚀
