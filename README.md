# Markdown Scanner & Organizer

An intelligent markdown file organization and discovery tool designed for ADHD minds that generate ideas faster than they can organize them.

## Features

### 1. **Semantic File Discovery**
- Find related files even with different terminology
- "Find my notes about sonar processing" returns semantically similar content
- Powered by local embeddings (no API costs)

### 2. **Automatic Clustering**
- Automatically groups files into coherent topics
- "These 47 files are about magnetic detection methodology"
- Discovers patterns in your scattered notes

### 3. **Timeline/Recency Awareness**
- Organize files by when they were modified
- "You worked on this idea 3 months ago, here are those files"
- Identify orphaned or aged files

### 4. **Combined Search**
- Semantic similarity (finds related content)
- Keyword matching (exact term search)
- Blended results for best of both worlds

### 5. **Quick Statistics**
- Total files and storage used
- Age distribution of your notes
- Cluster overview

## Installation

```bash
git clone https://github.com/yourusername/md-scanner.git
cd md-scanner
pip install -r requirements.txt
```

## Quick Start

### 1. Scan your markdown directories

```bash
python main.py scan /path/to/your/notes
```

This recursively finds all `.md` files and builds an index of their metadata.

### 2. Generate embeddings

```bash
python main.py embed
```

Creates semantic embeddings for all files (one-time, takes a few minutes for large collections).

### 3. Create clusters

```bash
python main.py cluster
```

Groups similar files into semantic clusters automatically.

### 4. Start searching

```bash
python main.py search "sonar processing techniques"
```

### 5. Explore your files

```bash
# Show recent activity
python main.py timeline --days 30

# List all clusters
python main.py list-clusters

# Get overview statistics
python main.py stats
```

## How It Works

### Architecture

```
File System
    ↓
[Scanner] → Metadata Index (files.json)
    ↓
[Embedding Engine] → Embeddings (embeddings.pkl)
    ↓
[Clustering Engine] → Cluster Assignments (clusters.json)
    ↓
[Search Engine] → Results
```

### Semantic Search

Uses `sentence-transformers` (all-MiniLM-L6-v2 model) to:
1. Extract content preview from each markdown file
2. Generate embedding vectors (384-dimensional)
3. Compare query embedding to file embeddings
4. Return files with highest cosine similarity

### Local Indexing

All data is stored locally in `~/.md_index/`:
- `files.json` - File metadata and paths
- `embeddings.pkl` - Cached embedding vectors
- `clusters.json` - Cluster assignments

No data leaves your computer. No API costs.

## Commands Reference

### `scan <directory>`
Index all markdown files in a directory tree.

**Options:**
- `--index-dir`: Override default index location

### `embed`
Generate embeddings for all indexed files.

**Workflow:**
1. Loads file paths from index
2. Extracts text content from files
3. Generates embeddings using transformer model
4. Saves embeddings to disk (reusable)

### `cluster`
Group similar files into semantic clusters.

**Options:**
- `--num-clusters`: Override auto-estimated cluster count

### `search <query>`
Find files matching your query.

**Options:**
- `--top-k`: Number of results (default: 10)
- `--semantic-weight`: Semantic vs keyword balance (0.0-1.0, default: 0.7)

### `list-clusters`
Show overview of all clusters with sample files.

### `timeline`
Show recent files organized by date.

**Options:**
- `--days`: Show files from last N days (default: 30)

### `stats`
Display index statistics and overview.

## What It Filters Out

Files matching these patterns are automatically excluded to avoid noise:
- venv directories
- site-packages (pip packages)
- Dist-info directories (wheel metadata)
- __pycache__ directories
- LICENSE.md files (common but not personal notes)
- Path LICENSE.md files in site packages

## Performance

- **Scanning:** ~1000 files/second
- **Embedding:** ~50-100 files/second (depends on file size)
- **Clustering:** Instant (uses cached embeddings)
- **Search:** Instant (cached embeddings)

For 3000 files:
- Scan: ~3 seconds
- Embed: ~30-60 seconds (one-time)
- Clustering: <1 second
- Each search: <100ms

## Future Enhancements

**Stage 2 - Adaptive File Naming:**
- Suggest better filenames when saving new notes
- Learn your naming patterns
- Build habits without permanent dependency

**Stage 3 - Cross-Reference Detection:**
- Find files mentioning same coordinates/data
- Identify code snippet variations
- Surface accidental duplication

**Stage 4 - GUI Dashboard:**
- Visual cluster map
- Interactive timeline
- Search interface
- File preview

**Stage 5 - Mobile Support:**
- Tauri wrapper for desktop
- Mobile apps for iOS/Android
- Cloud sync option (optional, encrypted)

## Technical Details

### Dependencies

- `sentence-transformers`: Semantic embeddings
- `scikit-learn`: K-means clustering
- `numpy`: Vector operations
- `click`: CLI framework
- `tqdm`: Progress bars

### Model Used

**all-MiniLM-L6-v2**
- 22M parameters (lightweight)
- 384-dimensional embeddings
- Trained on sentence similarity
- Fast inference, good quality

## Why Local Embeddings?

- **No API costs** - Everything runs on your machine
- **No data leakage** - Your notes never leave your computer
- **Offline capable** - Works without internet
- **Deterministic** - Same embeddings every time
- **Privacy-first** - Your thoughts stay private

## Contributing

This is a solo ADHD project built with AI assistance. It's designed to solve a real problem: organizing thoughts faster than they're generated.

## License

MIT

## Author

Built with Claude (Anthropic) + ADHD creativity

---

**The core belief:** This tool demonstrates AI as cognitive accessibility technology - helping neurodivergent developers build sophisticated tools while building the tools they need.
