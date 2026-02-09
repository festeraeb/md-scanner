# Wayfinder Tauri Desktop Application

## Setup Instructions

### Prerequisites

1. **Node.js & npm** (v16+)
   - Download from <https://nodejs.org/>

2. **Rust** (1.70+)
   - Install from <https://rustup.rs/>
   - Ensure Cargo is in your PATH

3. **Python** (3.8+)
   - Already installed if you have md-scanner

### Installation

1. **Install Node dependencies**:

   ```bash
   cd c:\Temp\md-scanner
   npm install
   ```

2. **Install Rust dependencies** (handled by Tauri):

   ```bash
   # Tauri will download dependencies automatically
   npm run tauri build
   ```

### Development

**Run in dev mode**:

```bash
npm run tauri dev
```

This will:

- Start Vite dev server on <http://localhost:5173>
- Launch Tauri window connected to dev server
- Enable hot reloading

### Building

**Create production binary**:

```bash
npm run tauri build
```

Output binaries:

- **Windows**: `src-tauri/target/release/wayfinder-tauri.exe`
- **macOS**: `src-tauri/target/release/wayfinder-tauri.dmg`
- **Linux**: `src-tauri/target/release/wayfinder-tauri`

## Architecture

### Components

- **Frontend**: React + TypeScript in `src/`
- **Backend**: Rust in `src-tauri/src/`
- **Python Bridge**: `md_scanner/tauri_bridge.py`

### Data Flow

```
React Component
  ↓
tauri.invoke("command_name", args)
  ↓
Rust Tauri Command Handler
  ↓
Python Subprocess (tauri_bridge.py)
  ↓
Python Engines (scanner, embeddings, clustering, search)
  ↓
JSON Response → React Component Update
```

## Testing

### 1. **Index Creation**

- Set index directory (e.g., `C:\Temp\md-scanner\.test_index\`)
- Use Scan panel to index a directory
- Check `files.json` is created in index directory

### 2. **Embeddings**

- Use Embed panel to generate embeddings
- Check `embeddings.pkl` is created
- Should show cached/generated count

### 3. **Clustering**

- Use Cluster panel to create clusters
- Check `clusters.json` is created
- Display cluster summary

### 4. **Search**

- Enter search query in Search panel
- Verify results with scores
- Click results to expand

### 5. **Stats & Timeline**

- Stats panel shows index statistics
- Timeline shows files by date

## Troubleshooting

### Python Module Not Found

```
Error: Failed to spawn Python process
```

**Solution**: Set PYTHON_PATH environment variable:

```bash
set PYTHON_PATH=C:\Path\To\Python\python.exe
npm run tauri dev
```

### Port Already in Use

```
Error: Port 5173 already in use
```

**Solution**: Change port in `vite.config.ts` or kill process on port 5173

### Rust Compilation Error

```bash
# Clear build cache
cargo clean
npm run tauri build
```

## Next Steps

1. **Mobile Support**: Set up Tauri mobile for iOS/Android
2. **Learning Device Mode**: Add simplified UI for tablets
3. **Offline Sync**: Store index locally, sync when connected
4. **Auto-update**: Implement Tauri's auto-updater

## Resources

- [Tauri Docs](https://tauri.app/v1/guides/)
- [React Docs](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
