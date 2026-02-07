# Quick Start Guide

## Installation & Setup (3 minutes)

```bash
# Clone the project
git clone https://github.com/yourusername/md-scanner.git
cd md-scanner

# Install dependencies
pip install -r requirements.txt
```

## Basic Workflow

### Step 1: Scan Your Markdown Files
```bash
# Scan a single directory
python main.py scan /c/Temp

# Or scan your Google Drive
python main.py scan "/path/to/Google Drive"
```

**Result:** Searches all `.md` files recursively, filters out noise (venv, site-packages, etc.), creates `files.json` index.

**Performance:** ~660 files scanned in seconds

### Step 2: Generate Embeddings
```bash
python main.py embed
```

**What it does:**
- Loads all file paths from index
- Extracts first 2000 chars from each markdown
- Converts text to semantic vectors (384-dimensional embeddings)
- Caches embeddings locally for fast search

**Performance:** Takes ~1-2 minutes for 660 files (one-time cost)

**Storage:** ~1.5 MB for 660 embeddings

### Step 3: Create Clusters
```bash
python main.py cluster
```

**What it does:**
- Groups similar files together using K-means clustering
- Automatically estimates number of clusters (~20-50)
- Saves cluster assignments to `clusters.json`

**Result:** Discovers natural topic groupings in your notes

### Step 4: Search!
```bash
# Find files about a topic
python main.py search "sonar processing"
python main.py search "drift modeling"
python main.py search "magnetic detection"

# Get all results (up to 50)
python main.py search "your query" --top-k 50
```

**Search algorithm:**
1. Converts your query to semantic embedding
2. Finds files with most similar embeddings
3. Also matches keywords in filenames/content
4. Returns blended results (semantic + keyword)

### Additional Commands

```bash
# See all clusters & topics
python main.py list-clusters

# Timeline view (recent activity)
python main.py timeline --days 30

# Overall statistics
python main.py stats
```

## Example Output

### Search Results
```
Searching for: 'sonar processing techniques'

Found 10 results:
  1. SONARSNIFFER_SUMMARY.md (0.847)
     → C:\Temp\Garminjunk\SONARSNIFFER_SUMMARY.md

  2. SONARSNIFFER_OPTIMIZATION_COMPLETE.md (0.823)
     → C:\Temp\Garminjunk\SONARSNIFFER_OPTIMIZATION_COMPLETE.md

  3. sonar0001_lab_summary.md (0.781)
     → C:\Temp\Garminjunk\archive\...\sonar0001_lab_summary.md
```

### Cluster Overview
```
Total clusters: 31

Cluster 0 (47 files):
  - SONARSNIFFER_SUMMARY.md
  - sonar_processing_guide.md
  - real_time_streaming.md

Cluster 1 (23 files):
  - CESAROPS_ULTIMATE_PLATFORM_SUMMARY.md
  - drift_modeling_v2.md
  - particle_tracking.md
```

## Tips & Tricks

### Custom Cluster Count
```bash
# Force specific number of clusters (default auto-estimates)
python main.py cluster --num-clusters 50
```

### Semantic vs Keyword Balance
```bash
# More semantic (find similar concepts)
python main.py search "your query" --semantic-weight 0.9

# More keyword-based (exact terms matter more)
python main.py search "your query" --semantic-weight 0.3
```

### Rescanning After Edits
If you've added/modified files:
```bash
python main.py scan /path/to/directory  # Updates index
python main.py embed                     # Regenerates embeddings
python main.py cluster                   # Reclusters
```

## What Gets Indexed/Filtered

### ✅ Included
- All `.md` files you've created
- Notes in any subdirectory depth
- Comments and documentation you wrote

### ❌ Automatically Filtered Out
- Virtual environments (`venv/`, `.env/`)
- Python packages (`site-packages/`, `.dist-info/`)
- Generated files (`__pycache__/`, `.pytest_cache/`)
- License files (since they're usually copy-paste)
- Framework documentation

See `exclude_patterns` in `md_scanner/scanner.py` to customize.

## Data Storage

All index data is stored locally in `~/.md_index/`:

```
~/.md_index/
├── files.json          # File metadata & paths
├── embeddings.pkl      # Cached semantic vectors
└── clusters.json       # Cluster assignments
```

**Privacy:** No data leaves your computer. All embeddings generated locally.

## Next Steps

1. **Scan your real data:** Replace `C:\Temp` with your actual markdown directory
2. **Try different searches:** "What does this return? Can you find files I forgot about?"
3. **Explore clusters:** What topics naturally emerge in your notes?
4. **Give feedback:** Which clusters made sense? Which didn't?

## Performance Expectations

| Operation | Time | Notes |
|-----------|------|-------|
| Scan 600 files | 5-10 sec | Recurses filesystem |
| Embed 600 files | 60-120 sec | One-time, CPU bound |
| Cluster | <1 sec | Uses cached embeddings |
| Search | <100ms | Cached similarity compute |

## Troubleshooting

### "No embeddings found"
Run: `python main.py embed`

### "No clusters found"
Run: `python main.py cluster`

### Search returning nothing
- Ensure you've run `embed` step
- Try different query (more/fewer keywords)
- Check `python main.py stats` to verify files were scanned

### Memory issues with large collections
The tool is efficient, but if you hit issues:
- Process directories one at a time
- Increase system virtual memory
- Reduce `--top-k` results

## What's Next?

**Phase 2 Features (planned):**
- Adaptive file naming suggestions
- Cross-reference detection
- GUI dashboard with visual cluster map
- Optional cloud sync layer
