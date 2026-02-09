# Wayfinder Master Plan

> A comprehensive reference document covering the entire project vision from inception through the adaptive learning system.

---

## Table of Contents

1. [Vision & Philosophy](#vision--philosophy)
2. [Core Engine: Markdown Scanner](#core-engine-markdown-scanner)
3. [Desktop App: Tauri Transition](#desktop-app-tauri-transition)
4. [Adaptive Learning System](#adaptive-learning-system)
5. [Universal Document Support](#universal-document-support)
6. [Architecture Overview](#architecture-overview)
7. [Roadmap & Phases](#roadmap--phases)
8. [Technical Reference](#technical-reference)

---

## Vision & Philosophy

### The Core Belief

> **AI as cognitive accessibility technology** - helping neurodivergent developers build sophisticated tools while building the tools they need.

### Why Wayfinder Exists

Built for ADHD minds that:
- Generate thoughts faster than they can organize them
- Have hundreds of scattered markdown files, notes, code snippets
- Need **semantic discovery** - finding files by meaning, not just filename
- Struggle with consistent naming and folder organization
- Benefit from progressive guidance that fades as skills develop

### Privacy-First Design

- **All data stays local** - No cloud sync required
- **No API costs** - Local embeddings via sentence-transformers
- **Offline capable** - Works without internet
- **Deterministic** - Same embeddings every time

---

## Core Engine: Markdown Scanner

The foundation that everything builds upon. This is the **permanent core** of Wayfinder.

### Five Core Engines (Python)

| Engine | File | Purpose |
|--------|------|---------|
| **FileScanner** | `scanner.py` | Index markdown files recursively |
| **EmbeddingEngine** | `embeddings.py` | Generate semantic vectors (384-dim) |
| **ClusteringEngine** | `clustering.py` | Group similar files with K-means |
| **SearchEngine** | `search.py` | Combined semantic + keyword search |
| **TimelineEngine** | `timeline.py` | Recent files organized by date |

### Key Features

1. **Semantic File Discovery**
   - Search by meaning, not keywords
   - Find "that meeting notes about the budget" without knowing the filename
   - all-MiniLM-L6-v2 model (22M params, lightweight)

2. **Automatic Clustering**
   - Group related files automatically
   - Surface hidden relationships
   - Help see the forest AND the trees

3. **Timeline/Recency Awareness**
   - "What was I working on yesterday?"
   - Quick context restoration

4. **Combined Search**
   - Semantic weight (default 70%) + keyword weight (30%)
   - Configurable balance

5. **Local Index Storage**
   - `~/.md_index/` default location
   - `files.json` - Metadata and paths
   - `embeddings.pkl` - Cached vectors
   - `clusters.json` - Cluster assignments

### CLI Workflow

```bash
# 1. Scan directories for markdown files
python -m md_scanner scan /path/to/notes

# 2. Generate semantic embeddings (one-time, cached)
python -m md_scanner embed

# 3. Create clusters
python -m md_scanner cluster

# 4. Search naturally
python -m md_scanner search "meeting notes about budget"
```

---

## Desktop App: Tauri Transition

### Why Tauri Instead of Flask?

| Old (Flask) | New (Tauri) |
|-------------|-------------|
| Web server overhead | Pure local app |
| Browser tab | Native window |
| Cross-process complexity | Integrated IPC |
| Limited file access | Full native dialogs |
| No tray/menu bar | Full desktop integration |

### Technology Stack

- **Frontend**: React 18 + TypeScript + Pure CSS (no frameworks)
- **Desktop Runtime**: Tauri 2.0 (Rust)
- **Python Bridge**: Subprocess IPC via JSON RPC
- **Build Tool**: Vite

### Data Flow

```
React Component
     ↓
tauri.invoke("command")
     ↓
Rust Command Handler (async)
     ↓
Python Subprocess (tauri_bridge.py)
     ↓
Python Engines
     ↓
JSON Response → UI Update
```

### Tablet/Learning Mode

A simplified interface for:
- Educational devices (Chromebook, iPad, Android tablet)
- Learning scenarios with guided workflow
- Progress tracking and engagement
- Auto-detected via viewport size

### Key UI Components

| Component | Purpose |
|-----------|---------|
| `SearchPanel.tsx` | Natural language search |
| `ScanPanel.tsx` | Index directory selection |
| `EmbedPanel.tsx` | Embedding generation |
| `ClusterPanel.tsx` | Cluster visualization |
| `TimelinePanel.tsx` | Recent files view |
| `StatsPanel.tsx` | Index statistics |
| `LearningModeApp.tsx` | Tablet/simplified UI |

---

## Adaptive Learning System

### The Core Concept

> **"Training wheels that come off"**

The system learns the user's struggles and:
1. **Detects difficulties** - What are they struggling with?
2. **Offers suggestions** - Naming, folder structure, organization
3. **Tracks skill improvement** - Are they getting better?
4. **Fades out help** - As mastery develops, suggestions disappear
5. **Re-engages on regression** - If skills decline, help returns

### Four Learning Modules

Located in `md_scanner/learning/`:

#### 1. BehaviorTracker (`behavior_tracker.py`)

Records all user actions to learn patterns:

```python
# Events tracked:
- SearchEvent: query, results found, result clicked, time spent
- FileAccessEvent: file path, action type, naming quality, folder depth
- NavigationEvent: from/to paths, used search vs browse vs recents
- OrganizationDecision: suggested vs chosen name/folder, accepted or rejected
```

**Storage**: `~/.wayfinder/learning/behavior_sessions.json`

#### 2. DifficultyDetector (`difficulty_detector.py`)

Identifies what users struggle with:

| Skill Area | Indicators |
|------------|------------|
| `search_ability` | Success rate, refinement count, time to find |
| `naming_consistency` | Date prefixes, descriptive names, extensions |
| `folder_organization` | Depth consistency, logical hierarchy |
| `file_management` | Duplicate handling, version control |

**Skill Levels**:
- `STRUGGLING` (score < 0.4) - Full guidance
- `LEARNING` (score < 0.7) - Regular suggestions
- `PROFICIENT` (score < 0.85) - Occasional tips
- `MASTERED` (score ≥ 0.85) - Suggestions faded

**Trends**:
- `IMPROVING` - Keep reducing help
- `STABLE` - Maintain current level
- `REGRESSING` - Re-engage guidance
- `NEW` - Not enough history yet

#### 3. SuggestionEngine (`suggestion_engine.py`)

Generates contextual suggestions:

**Naming Conventions**:
| Convention | Example |
|------------|---------|
| `date_prefix` | `2025-01-15-meeting-notes.md` |
| `category_first` | `notes-budget-review.md` |
| `project_based` | `wayfinder-setup-guide.md` |
| `semantic` | `quarterly-budget-analysis.md` |

**Folder Structures**:
| Structure | Description |
|-----------|-------------|
| `by_type` | notes/, code/, research/ |
| `by_project` | wayfinder/, client-work/ |
| `by_date` | 2025-01/, 2025-02/ |
| `gtd` | inbox/, active/, archive/ |

**Adaptive Features**:
- Learns from user's existing file patterns
- Suggestions match detected preferences
- Confidence scores indicate strength

#### 4. AdaptiveCoach (`adaptive_coach.py`) - *Pending*

Orchestrates the fade-out logic:

```python
# Conceptual flow:
1. Load behavior history
2. Run difficulty assessment
3. Calculate suggestion intensity
4. Generate filtered suggestions
5. Track suggestion acceptance
6. Adjust future intensity
```

### How Training Wheels Come Off

```
New User (Day 1)
    ├── Every file save → naming suggestion
    ├── Every folder → structure recommendation
    └── Every search → tips on better queries

Improving User (Week 2)
    ├── Good naming pattern detected → reduce naming tips
    ├── Search success rate high → remove search tips
    └── Some folder issues remain → keep folder suggestions

Proficient User (Month 2)
    ├── Naming: mastered → no suggestions
    ├── Folders: proficient → occasional tips
    └── All working well → minimal intervention

Regression Detected (Month 3)
    ├── Naming quality decreased → re-enable naming tips
    └── Other skills stable → no change
```

---

## Universal Document Support

### Beyond Markdown

The goal is to index and organize **all** document types:

| Format | Extractor | Difficulty | Library |
|--------|-----------|------------|---------|
| `.md` | Built-in | ✅ Done | Native |
| `.docx` | Word | Easy | python-docx |
| `.xlsx` | Excel | Easy | openpyxl |
| `.pptx` | PowerPoint | Medium | python-pptx |
| `.pdf` | PDF | Medium | PyPDF2/pdfplumber |
| `.odt/.ods/.odp` | OpenOffice | Medium | python-odf |
| `.py/.js/.ts` | Code | Medium | AST parsing |
| `.jpg/.png` | Images | Hard | EasyOCR/Tesseract |
| `.mp3/.wav` | Audio | Hard | Whisper |

### Implementation Approach

Each extractor follows the same interface:

```python
class DocumentExtractor:
    def can_handle(self, file_path: str) -> bool: ...
    def extract_text(self, file_path: str) -> str: ...
    def extract_metadata(self, file_path: str) -> dict: ...
```

The embedding engine then treats all extracted text uniformly.

---

## Architecture Overview

### Directory Structure

```
md-scanner/
├── src/                          # React Frontend
│   ├── App.tsx                   # Main router
│   ├── types.ts                  # TypeScript types
│   ├── components/               # UI panels
│   ├── hooks/                    # React hooks
│   ├── services/                 # Tauri wrappers
│   └── styles/                   # CSS modules
│
├── src-tauri/                    # Rust Backend
│   ├── src/main.rs               # Entry point
│   ├── src/commands.rs           # Tauri commands
│   ├── src/python_bridge.rs      # Python subprocess
│   └── Cargo.toml                # Rust deps
│
├── md_scanner/                   # Python Engines
│   ├── __init__.py
│   ├── scanner.py                # File indexing
│   ├── embeddings.py             # Semantic vectors
│   ├── clustering.py             # K-means grouping
│   ├── search.py                 # Combined search
│   ├── timeline.py               # Recent files
│   ├── tauri_bridge.py           # JSON RPC handler
│   │
│   ├── learning/                 # Adaptive System
│   │   ├── __init__.py
│   │   ├── behavior_tracker.py   # Event recording
│   │   ├── difficulty_detector.py # Skill assessment
│   │   ├── suggestion_engine.py  # Recommendations
│   │   └── adaptive_coach.py     # Orchestration ✅
│   │
│   └── extractors/               # Document Processing (pending)
│       ├── docx_extractor.py
│       ├── pdf_extractor.py
│       ├── xlsx_extractor.py
│       └── ...
│
├── Dockerfile                    # Python backend container
├── docker-compose.yml            # Multi-container setup
├── package.json                  # Node deps
├── vite.config.ts                # Build config
└── WAYFINDER_MASTER_PLAN.md      # This file
```

### Platform Independence (Rust/Python/Tauri/Docker)

| Layer | Technology | Role |
|-------|------------|------|
| **Desktop UI** | Tauri 2.0 (Rust) | Native window, cross-platform binaries |
| **Frontend** | React + TypeScript | Shared codebase for all platforms |
| **Backend** | Python 3.11 | ML/NLP engines, learning system |
| **Containerization** | Docker | Consistent Python environment |
| **Communication** | JSON-RPC | Subprocess IPC, works everywhere |

**Cross-Platform Coverage:**
- **Windows**: Tauri `.exe` + Python (local or Docker)
- **macOS**: Tauri `.dmg` + Python (local or Docker)
- **Linux**: Tauri AppImage + Python (local or Docker)
- **Docker**: Standalone Python backend for headless/server use

### Data Storage Locations

| Data | Location |
|------|----------|
| File index | `~/.md_index/files.json` |
| Embeddings | `~/.md_index/embeddings.pkl` |
| Clusters | `~/.md_index/clusters.json` |
| Behavior history | `~/.wayfinder/learning/behavior_sessions.json` |
| Skill assessments | `~/.wayfinder/learning/skill_history.json` |
| User preferences | `~/.wayfinder/preferences.json` |

---

## Roadmap & Phases

### Completed ✅

| Item | Status |
|------|--------|
| Core 5 Python engines | ✅ Complete |
| CLI interface | ✅ Complete |
| Tauri desktop structure | ✅ Complete |
| React UI components | ✅ Complete |
| Tablet/learning mode | ✅ Complete |
| Windows installer docs | ✅ Complete |
| BehaviorTracker module | ✅ Complete |
| DifficultyDetector module | ✅ Complete |
| SuggestionEngine module | ✅ Complete |
| AdaptiveCoach module | ✅ Complete |
| Docker configuration | ✅ Complete |

### In Progress ⏳

| Item | Status |
|------|--------|
| Learning system UI integration | ⏳ Backend ready, connecting to React |

### Planned 📋

| Phase | Items | Timeline |
|-------|-------|----------|
| **Phase 2: Mobile** | iOS/Android companion, search-only UI, index sync | 3-4 weeks |
| **Phase 3: Offline** | Cached index, encrypted export, sync back | 2-3 weeks |
| **Phase 4: Learning+** | Quiz mode, achievements, teacher dashboard | 4-6 weeks |
| **Phase 5: Multi-Device** | Network discovery, USB transfer, multi-user | 3-5 weeks |
| **Phase 6: Universal** | All document extractors, VS Code extension | 4-6 weeks |

---

## Technical Reference

### Performance Characteristics

| Operation | Speed |
|-----------|-------|
| Scanning | ~1000 files/second |
| Embedding | ~50-100 files/second |
| Clustering | <1 second (cached) |
| Search | <100ms |
| Rust subprocess bridge | <100ms overhead |
| Python initial load | 500-2000ms (torch) |

### Model Details

**all-MiniLM-L6-v2**
- Parameters: 22M (lightweight)
- Embedding dimensions: 384
- Training: Sentence similarity
- Inference: Fast, CPU-optimized

### Key Dependencies

**Python**:
- sentence-transformers
- scikit-learn
- numpy
- click
- tqdm

**Node/Tauri**:
- React 18
- TypeScript 5
- Vite
- @tauri-apps/api

**Rust**:
- tauri 2.0
- tokio (async runtime)
- serde (JSON)

---

## Quick Commands Reference

```bash
# Development
npm run tauri dev          # Launch dev mode with hot reload

# Production
npm run tauri build        # Create distributable binary

# CLI (Python)
python -m md_scanner scan /path    # Index files
python -m md_scanner embed         # Generate embeddings
python -m md_scanner cluster       # Create clusters
python -m md_scanner search "query" # Search files
python -m md_scanner timeline      # Recent files
python -m md_scanner stats         # Index statistics
```

---

## Philosophy Reminders

1. **Markdown is the core** - All other formats extend from it
2. **Local first** - No cloud dependency required
3. **Privacy always** - Your thoughts stay on your machine
4. **Training wheels come off** - Help fades with mastery
5. **Re-engage on regression** - Support returns when needed
6. **ADHD-friendly** - Built by neurodivergent minds, for neurodivergent minds

---

*Last Updated: January 2025*
*Document Version: 1.0*
