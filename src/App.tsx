import { useState, useEffect } from "react";
import { tauriService } from "./services/tauri";
import { useTheme } from "./hooks/useTauri";
import { open } from "@tauri-apps/plugin-dialog";
import { GitAssistant } from "./components/GitAssistant";
import "./styles/global.css";
import "./styles/app.css";
import "./styles/panels.css";

// File type presets
const FILE_TYPE_PRESETS = {
    "Markdown": [".md", ".mdx", ".markdown"],
    "Text": [".txt", ".text", ".log"],
    "Python": [".py", ".pyw", ".pyi"],
    "JavaScript": [".js", ".jsx", ".ts", ".tsx"],
    "Documents": [".pdf", ".doc", ".docx", ".odt"],
    "Config": [".json", ".yaml", ".yml", ".toml", ".ini"],
    "Web": [".html", ".htm", ".css", ".scss"],
    "All Text Files": ["*"],
};

type OperationStatus = "idle" | "running" | "complete" | "error";
type ActiveSection = "status" | "scan" | "embed" | "cluster" | "search" | "timeline" | "git";

interface ScanResult {
    files_scanned: number;
    total_size: number;
    index_path: string;
}

interface IndexStats {
    total_files: number;
    total_size_bytes: number;
    extensions: Record<string, number>;
    last_updated: string;
    scan_path?: string;
    // These will be false until we implement embeddings/clustering
    has_embeddings?: boolean;
    has_clusters?: boolean;
    cluster_count?: number;
    embeddings_count?: number;
}

export default function App() {
    const { isDark, toggleTheme } = useTheme();

    // Navigation state
    const [activeSection, setActiveSection] = useState<ActiveSection>("status");

    // Index configuration
    const [indexPath, setIndexPath] = useState<string>("");
    const [scanPath, setScanPath] = useState<string>("");
    const [selectedTypes, setSelectedTypes] = useState<string[]>(["Markdown"]);

    // Status/Stats state
    const [indexStats, setIndexStats] = useState<IndexStats | null>(null);

    // Scan state
    const [scanStatus, setScanStatus] = useState<OperationStatus>("idle");
    const [scanProgress, setScanProgress] = useState({ current: 0, total: 0, percent: 0 });
    const [scanResult, setScanResult] = useState<ScanResult | null>(null);

    // Embed state
    const [embedStatus, setEmbedStatus] = useState<OperationStatus>("idle");
    const [embedProgress, setEmbedProgress] = useState(0);
    const [embedResult, setEmbedResult] = useState<any>(null);

    // Azure config state
    const [azureConfigured, setAzureConfigured] = useState(false);
    const [azureEndpoint, setAzureEndpoint] = useState("");
    const [azureApiKey, setAzureApiKey] = useState("");
    const [azureDeployment, setAzureDeployment] = useState("text-embedding-ada-002");
    const [showAzureConfig, setShowAzureConfig] = useState(false);

    // Cluster state
    const [clusterStatus, setClusterStatus] = useState<OperationStatus>("idle");
    const [numClusters, setNumClusters] = useState<number | undefined>(undefined);
    const [clusters, setClusters] = useState<any[]>([]);

    // Search state
    const [searchQuery, setSearchQuery] = useState("");
    const [searchResults, setSearchResults] = useState<any[]>([]);
    const [topK, setTopK] = useState(10);
    const [semanticWeight, setSemanticWeight] = useState(0.7);

    // Timeline state
    const [timelineDays, setTimelineDays] = useState(30);
    const [timelineData, setTimelineData] = useState<any[]>([]);

    // Error state
    const [errorMsg, setErrorMsg] = useState<string>("");

    // Theme setup
    useEffect(() => {
        document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
    }, [isDark]);

    // Load stats on mount and when switching to status
    useEffect(() => {
        if (activeSection === "status" && indexPath) {
            loadStats();
        }
    }, [activeSection, indexPath]);

    // Toggle file type selection
    const toggleFileType = (type: string) => {
        setSelectedTypes(prev =>
            prev.includes(type)
                ? prev.filter(t => t !== type)
                : [...prev, type]
        );
    };

    // Get all selected extensions
    const getSelectedExtensions = (): string[] => {
        const extensions: string[] = [];
        selectedTypes.forEach(type => {
            const exts = FILE_TYPE_PRESETS[type as keyof typeof FILE_TYPE_PRESETS];
            if (exts) extensions.push(...exts);
        });
        return [...new Set(extensions)];
    };

    // Load index stats
    const loadStats = async () => {
        if (!indexPath) return;
        try {
            const stats = await tauriService.getStats(indexPath);
            setIndexStats(stats);
        } catch (error) {
            console.error("Failed to load stats:", error);
        }
    };

    // Handle scan
    const handleScan = async () => {
        if (!scanPath.trim()) {
            setErrorMsg("Please enter a folder path to scan");
            return;
        }

        if (selectedTypes.length === 0) {
            setErrorMsg("Please select at least one file type");
            return;
        }

        // Use backslash for Windows paths
        const separator = scanPath.includes('\\') ? '\\' : '/';
        const effectiveIndexPath = indexPath || `${scanPath}${separator}.wayfinder_index`;

        setScanStatus("running");
        setErrorMsg("");
        setScanProgress({ current: 0, total: 0, percent: 0 });

        try {
            console.log("Starting scan:", scanPath, effectiveIndexPath);
            const result = await tauriService.scanDirectory(scanPath, effectiveIndexPath);
            console.log("Scan result:", result);
            setScanResult(result);
            setIndexPath(effectiveIndexPath);
            setScanStatus("complete");
            loadStats();
            loadAzureConfig(effectiveIndexPath);
        } catch (error: any) {
            console.error("Scan error:", error);
            setScanStatus("error");
            setErrorMsg(error.toString());
        }
    };

    // Load Azure config
    const loadAzureConfig = async (dir: string) => {
        try {
            const config = await tauriService.loadAzureConfig(dir);
            setAzureConfigured(config.configured);
            if (config.endpoint) setAzureEndpoint(config.endpoint);
            if (config.deployment_name) setAzureDeployment(config.deployment_name);
        } catch (error) {
            console.log("No Azure config found");
        }
    };

    // Save Azure config
    const saveAzureConfig = async () => {
        if (!indexPath) {
            setErrorMsg("Please scan a folder first to set the index location");
            return;
        }
        if (!azureEndpoint || !azureApiKey || !azureDeployment) {
            setErrorMsg("Please fill in all Azure configuration fields");
            return;
        }

        try {
            await tauriService.saveAzureConfig(
                indexPath,
                azureEndpoint,
                azureApiKey,
                azureDeployment
            );
            setAzureConfigured(true);
            setShowAzureConfig(false);
            setErrorMsg("");
        } catch (error: any) {
            setErrorMsg(error.toString());
        }
    };

    // Handle embed
    const handleEmbed = async () => {
        if (!indexPath) {
            setErrorMsg("No index available. Please scan a folder first.");
            return;
        }

        if (!azureConfigured) {
            setShowAzureConfig(true);
            setErrorMsg("Please configure Azure OpenAI settings first.");
            return;
        }

        setEmbedStatus("running");
        setEmbedProgress(10);
        setErrorMsg("");
        setEmbedResult(null);

        try {
            const result = await tauriService.generateEmbeddings(indexPath);
            console.log("Embed result:", result);
            setEmbedResult(result);
            setEmbedProgress(100);
            setEmbedStatus("complete");
            loadStats();
        } catch (error: any) {
            setEmbedStatus("error");
            setErrorMsg(error.toString());
        }
    };

    // Handle cluster
    const handleCluster = async () => {
        if (!indexPath) {
            setErrorMsg("No index available. Please scan a folder first.");
            return;
        }

        setClusterStatus("running");
        setErrorMsg("");

        try {
            await tauriService.createClusters(indexPath, numClusters);
            const clusterData = await tauriService.getClustersSummary(indexPath);
            setClusters(clusterData.clusters || []);
            setClusterStatus("complete");
            loadStats();
        } catch (error: any) {
            setClusterStatus("error");
            setErrorMsg(error.toString());
        }
    };

    // Handle search
    const handleSearch = async () => {
        if (!searchQuery.trim() || !indexPath) return;

        try {
            const results = await tauriService.search(searchQuery, indexPath, topK, semanticWeight);
            setSearchResults(results);
        } catch (error) {
            console.error("Search error:", error);
        }
    };

    // Handle timeline
    const handleTimeline = async () => {
        if (!indexPath) return;

        try {
            const data = await tauriService.getTimeline(indexPath, timelineDays);
            setTimelineData(data.timeline || []);
        } catch (error) {
            console.error("Timeline error:", error);
        }
    };

    // Navigation items
    const navItems: { id: ActiveSection; icon: string; label: string }[] = [
        { id: "status", icon: "📊", label: "Status" },
        { id: "scan", icon: "📁", label: "Scan" },
        { id: "embed", icon: "🧠", label: "Embeddings" },
        { id: "cluster", icon: "🗂️", label: "Clusters" },
        { id: "search", icon: "🔍", label: "Search" },
        { id: "timeline", icon: "📅", label: "Timeline" },
        { id: "git", icon: "📎", label: "Git Clippy" },
    ];

    return (
        <div className="app-container">
            {/* Sidebar Navigation */}
            <aside className="sidebar">
                <div className="sidebar-header">
                    <h1>⛵🐕 Wayfinder</h1>
                    <p className="tagline">by NautiDog</p>
                </div>

                <nav className="sidebar-nav">
                    {navItems.map(item => (
                        <button
                            key={item.id}
                            className={`nav-item ${activeSection === item.id ? "active" : ""}`}
                            onClick={() => setActiveSection(item.id)}
                        >
                            <span className="nav-icon">{item.icon}</span>
                            <span className="nav-label">{item.label}</span>
                        </button>
                    ))}
                </nav>

                <div className="sidebar-footer">
                    <button className="theme-toggle" onClick={toggleTheme} title="Toggle theme">
                        {isDark ? "☀️ Light Mode" : "🌙 Dark Mode"}
                    </button>
                </div>
            </aside>

            {/* Main Content */}
            <main className="main-content">
                {/* Top Stats Bar */}
                <div className="top-bar">
                    <div className="stats-summary">
                        <div className="stat-chip">
                            <span className="stat-label">Files</span>
                            <span className="stat-value">{indexStats?.total_files || 0}</span>
                        </div>
                        <div className="stat-chip">
                            <span className="stat-label">Embeddings</span>
                            <span className="stat-value">{indexStats?.has_embeddings ? "✓" : "—"}</span>
                        </div>
                        <div className="stat-chip">
                            <span className="stat-label">Clusters</span>
                            <span className="stat-value">{indexStats?.cluster_count || 0}</span>
                        </div>
                    </div>
                    {indexPath && (
                        <div className="index-path-display">
                            📁 {indexPath}
                        </div>
                    )}
                </div>

                {/* Error Display */}
                {errorMsg && (
                    <div className="error-banner">
                        ❌ {errorMsg}
                        <button onClick={() => setErrorMsg("")}>✕</button>
                    </div>
                )}

                {/* Status Section */}
                {activeSection === "status" && (
                    <section className="content-section">
                        <h2>📊 Index Status</h2>
                        {!indexPath ? (
                            <div className="empty-state">
                                <p>No index loaded. Go to <strong>Scan</strong> to create one.</p>
                                <button className="btn btn-primary" onClick={() => setActiveSection("scan")}>
                                    📁 Start Scanning
                                </button>
                            </div>
                        ) : (
                            <div className="status-grid">
                                <div className="status-card">
                                    <h3>Files Indexed</h3>
                                    <span className="big-number">{indexStats?.total_files || 0}</span>
                                </div>
                                <div className="status-card">
                                    <h3>Embeddings</h3>
                                    <span className={`status-badge ${indexStats?.has_embeddings ? "success" : "pending"}`}>
                                        {indexStats?.has_embeddings ? "Generated" : "Not Generated"}
                                    </span>
                                </div>
                                <div className="status-card">
                                    <h3>Clusters</h3>
                                    <span className={`status-badge ${indexStats?.has_clusters ? "success" : "pending"}`}>
                                        {indexStats?.has_clusters ? `${indexStats.cluster_count} Clusters` : "Not Created"}
                                    </span>
                                </div>
                                <div className="status-card">
                                    <h3>Last Scan</h3>
                                    <span>{indexStats?.last_updated || "Unknown"}</span>
                                </div>
                            </div>
                        )}
                    </section>
                )}

                {/* Scan Section */}
                {activeSection === "scan" && (
                    <section className="content-section">
                        <h2>📁 Scan Files</h2>
                        <p className="section-desc">Select file types and a folder to index.</p>

                        {/* File Type Selection */}
                        <div className="form-group">
                            <label>File Types to Scan:</label>
                            <div className="file-type-grid">
                                {Object.entries(FILE_TYPE_PRESETS).map(([name, extensions]) => (
                                    <label
                                        key={name}
                                        className={`file-type-option ${selectedTypes.includes(name) ? "selected" : ""}`}
                                    >
                                        <input
                                            type="checkbox"
                                            checked={selectedTypes.includes(name)}
                                            onChange={() => toggleFileType(name)}
                                        />
                                        <span className="file-type-name">{name}</span>
                                        <span className="file-type-exts">
                                            {extensions.slice(0, 3).join(", ")}
                                        </span>
                                    </label>
                                ))}
                            </div>
                        </div>

                        {/* Folder Selection */}
                        <div className="form-group">
                            <label>Folder to Scan:</label>
                            <div className="input-row">
                                <input
                                    type="text"
                                    placeholder="Enter folder path..."
                                    value={scanPath}
                                    onChange={(e) => setScanPath(e.target.value)}
                                    className="folder-input"
                                />
                                <button 
                                    className="btn btn-secondary"
                                    onClick={async () => {
                                        const selected = await open({
                                            directory: true,
                                            multiple: false,
                                            title: "Select folder to scan"
                                        });
                                        if (selected && typeof selected === 'string') {
                                            setScanPath(selected);
                                        }
                                    }}
                                >
                                    📂 Browse
                                </button>
                            </div>
                        </div>

                        {/* Index Path (optional) */}
                        <details className="advanced-options">
                            <summary>Advanced Options</summary>
                            <div className="form-group">
                                <label>Custom Index Location:</label>
                                <input
                                    type="text"
                                    placeholder="Leave empty for default"
                                    value={indexPath}
                                    onChange={(e) => setIndexPath(e.target.value)}
                                />
                            </div>
                        </details>

                        {/* Scan Button */}
                        <div className="action-row">
                            <button 
                                className="btn btn-primary btn-large"
                                onClick={handleScan}
                                disabled={scanStatus === "running"}
                            >
                                {scanStatus === "running" ? "🔄 Scanning..." : "🔍 Start Scan"}
                            </button>
                        </div>

                        {/* Progress */}
                        {scanStatus === "running" && (
                            <div className="progress-section">
                                <div className="progress-bar">
                                    <div className="progress-fill" style={{ width: `${scanProgress.percent}%` }} />
                                </div>
                                <p>Scanning files...</p>
                            </div>
                        )}

                        {/* Result */}
                        {scanStatus === "complete" && scanResult && (
                            <div className="success-message">
                                <h3>✅ Scan Complete!</h3>
                                <p>{scanResult.files_scanned} files indexed ({(scanResult.total_size / 1024 / 1024).toFixed(1)} MB)</p>
                                <p>Index saved to: {scanResult.index_path}</p>
                            </div>
                        )}
                    </section>
                )}

                {/* Embed Section */}
                {activeSection === "embed" && (
                    <section className="content-section">
                        <h2>🧠 Generate Embeddings</h2>
                        <p className="section-desc">
                            Convert your indexed files into semantic vectors using Azure OpenAI for intelligent search and clustering.
                        </p>

                        {!indexPath ? (
                            <div className="empty-state">
                                <p>No index available. Please scan a folder first.</p>
                                <button className="btn btn-primary" onClick={() => setActiveSection("scan")}>
                                    📁 Go to Scan
                                </button>
                            </div>
                        ) : (
                            <>
                                    {/* Azure Config Section */}
                                    <div className="config-section">
                                        <div className="config-header">
                                            <h3>☁️ Azure OpenAI Configuration</h3>
                                            <span className={`config-status ${azureConfigured ? "configured" : "not-configured"}`}>
                                                {azureConfigured ? "✓ Configured" : "⚠ Not Configured"}
                                            </span>
                                            <button
                                                className="btn btn-small"
                                                onClick={() => setShowAzureConfig(!showAzureConfig)}
                                            >
                                                {showAzureConfig ? "Hide" : "Configure"}
                                            </button>
                                        </div>

                                        {showAzureConfig && (
                                            <div className="config-form">
                                                <div className="form-group">
                                                    <label>Azure OpenAI Endpoint:</label>
                                                    <input
                                                        type="text"
                                                        placeholder="https://your-resource.openai.azure.com"
                                                        value={azureEndpoint}
                                                        onChange={(e) => setAzureEndpoint(e.target.value)}
                                                    />
                                                </div>
                                                <div className="form-group">
                                                    <label>API Key:</label>
                                                    <input
                                                        type="password"
                                                        placeholder="Your Azure OpenAI API key"
                                                        value={azureApiKey}
                                                        onChange={(e) => setAzureApiKey(e.target.value)}
                                                    />
                                                </div>
                                                <div className="form-group">
                                                    <label>Deployment Name:</label>
                                                    <input
                                                        type="text"
                                                        placeholder="text-embedding-ada-002"
                                                        value={azureDeployment}
                                                        onChange={(e) => setAzureDeployment(e.target.value)}
                                                    />
                                                    <small>The name of your embedding model deployment</small>
                                                </div>
                                                <button className="btn btn-primary" onClick={saveAzureConfig}>
                                                    💾 Save Configuration
                                                </button>
                                            </div>
                                        )}
                                    </div>

                                <div className="info-box">
                                    <p><strong>Index:</strong> {indexPath}</p>
                                    <p><strong>Files:</strong> {indexStats?.total_files || 0}</p>
                                    <p><strong>Status:</strong> {indexStats?.has_embeddings ? "Embeddings exist" : "No embeddings yet"}</p>
                                </div>

                                <div className="action-row">
                                        <button
                                            className="btn btn-primary btn-large"
                                            onClick={handleEmbed}
                                            disabled={embedStatus === "running" || !azureConfigured}
                                        >
                                            {embedStatus === "running" ? "🔄 Generating..." : "🧠 Generate Embeddings"}
                                        </button>
                                        {!azureConfigured && (
                                            <span className="hint">Configure Azure OpenAI first</span>
                                        )}
                                </div>

                                {embedStatus === "running" && (
                                    <div className="progress-section">
                                        <div className="progress-bar">
                                            <div className="progress-fill" style={{ width: `${embedProgress}%` }} />
                                        </div>
                                            <p>Generating embeddings... This may take a few minutes for large indexes.</p>
                                            <small>Check the terminal for detailed progress.</small>
                                    </div>
                                )}

                                    {embedStatus === "complete" && embedResult && (
                                    <div className="success-message">
                                        <h3>✅ Embeddings Generated!</h3>
                                            <p>Generated: {embedResult.embeddings_generated} new embeddings</p>
                                            <p>Cached: {embedResult.cached_count} (unchanged files)</p>
                                            {embedResult.error_count > 0 && (
                                                <p className="warning">Errors: {embedResult.error_count} files failed</p>
                                            )}
                                        <p>Your files are now ready for semantic search and clustering.</p>
                                    </div>
                                )}

                                    {embedStatus === "error" && (
                                        <div className="error-message">
                                            <h3>❌ Embedding Failed</h3>
                                            <p>{errorMsg}</p>
                                        </div>
                                    )}
                            </>
                        )}
                    </section>
                )}

                {/* Cluster Section */}
                {activeSection === "cluster" && (
                    <section className="content-section">
                        <h2>🗂️ Create Clusters</h2>
                        <p className="section-desc">
                            Group similar files together for better organization.
                        </p>

                        {!indexPath ? (
                            <div className="empty-state">
                                <p>No index available. Please scan a folder first.</p>
                            </div>
                        ) : !indexStats?.has_embeddings ? (
                            <div className="empty-state">
                                <p>Embeddings required for clustering. Generate them first.</p>
                                <button className="btn btn-primary" onClick={() => setActiveSection("embed")}>
                                    🧠 Go to Embeddings
                                </button>
                                </div>
                            ) : (
                                <>
                                    <div className="form-group">
                                        <label>Number of Clusters (optional):</label>
                                        <input
                                            type="number"
                                            min="2"
                                            max="50"
                                            placeholder="Auto-detect"
                                            value={numClusters || ""}
                                            onChange={(e) => setNumClusters(e.target.value ? parseInt(e.target.value) : undefined)}
                                        />
                                        <small>Leave empty for automatic estimation</small>
                                    </div>

                                    <div className="action-row">
                                            <button 
                                                className="btn btn-primary btn-large"
                                                onClick={handleCluster}
                                                disabled={clusterStatus === "running"}
                                            >
                                                {clusterStatus === "running" ? "🔄 Clustering..." : "🗂️ Create Clusters"}
                                            </button>
                                </div>

                                {clusterStatus === "complete" && clusters.length > 0 && (
                                    <div className="clusters-list">
                                        <h3>📊 {clusters.length} Clusters Created</h3>
                                        {clusters.map((cluster, i) => (
                                            <div key={i} className="cluster-card">
                                                <h4>Cluster {i + 1}: {cluster.label || `Group ${i + 1}`}</h4>
                                                <p>{cluster.file_count || cluster.files?.length || 0} files</p>
                                                {cluster.keywords && (
                                                    <div className="keywords">
                                                        {cluster.keywords.slice(0, 5).map((kw: string, j: number) => (
                                                            <span key={j} className="keyword-tag">{kw}</span>
                                                        ))}
                                                    </div>
                                                )}
                                            </div>
                                        ))}
                                    </div>
                                )}
                            </>
                        )}
                    </section>
                )}

                {/* Search Section */}
                {activeSection === "search" && (
                    <section className="content-section">
                        <h2>🔍 Search Files</h2>

                        {!indexPath ? (
                            <div className="empty-state">
                                <p>No index available. Please scan a folder first.</p>
                            </div>
                        ) : (
                            <>
                                <div className="search-controls">
                                    <div className="search-box">
                                        <input
                                            type="text"
                                                placeholder="What are you looking for?"
                                                value={searchQuery}
                                                onChange={(e) => setSearchQuery(e.target.value)}
                                                onKeyDown={(e) => e.key === "Enter" && handleSearch()}
                                                className="search-input"
                                            />
                                            <button className="btn btn-primary" onClick={handleSearch}>
                                                Search
                                            </button>
                                        </div>

                                        <div className="search-options">
                                            <div className="option-group">
                                                <label>Results: {topK}</label>
                                                <input
                                                    type="range"
                                                    min="1"
                                                    max="50"
                                                    value={topK}
                                                    onChange={(e) => setTopK(parseInt(e.target.value))}
                                                />
                                            </div>
                                            <div className="option-group">
                                                <label>Semantic Weight: {semanticWeight.toFixed(1)}</label>
                                                <input
                                                    type="range"
                                                    min="0"
                                                    max="100"
                                                    value={semanticWeight * 100}
                                                    onChange={(e) => setSemanticWeight(parseInt(e.target.value) / 100)}
                                                />
                                            </div>
                                        </div>
                                    </div>

                                    <div className="search-results">
                                        {searchResults.length === 0 ? (
                                            <p className="no-results">Enter a search term to find files</p>
                                        ) : (
                                            <>
                                                <p className="results-count">{searchResults.length} results found</p>
                                                {searchResults.map((result, i) => (
                                                    <div key={i} className="search-result-item">
                                                        <div className="result-header">
                                                            <span className="result-name">{result.name}</span>
                                                            {result.score && (
                                                                <span className="result-score">
                                                                    {(result.score * 100).toFixed(0)}% match
                                                                </span>
                                                            )}
                                                        </div>
                                                        <div className="result-path">{result.path}</div>
                                                        {result.preview && (
                                                            <div className="result-preview">{result.preview}</div>
                                                        )}
                                                    </div>
                                                ))}
                                            </>
                                        )}
                                    </div>
                            </>
                        )}
                    </section>
                )}

                {/* Timeline Section */}
                {activeSection === "timeline" && (
                    <section className="content-section">
                        <h2>📅 Timeline</h2>
                        <p className="section-desc">View your files organized by modification date.</p>

                        {!indexPath ? (
                            <div className="empty-state">
                                <p>No index available. Please scan a folder first.</p>
                            </div>
                        ) : (
                            <>
                                <div className="timeline-controls">
                                    <label>Show files from last:</label>
                                    <select
                                        value={timelineDays}
                                        onChange={(e) => setTimelineDays(parseInt(e.target.value))}
                                    >
                                        <option value="7">7 days</option>
                                        <option value="14">2 weeks</option>
                                        <option value="30">30 days</option>
                                        <option value="90">90 days</option>
                                    </select>
                                    <button className="btn btn-primary" onClick={handleTimeline}>
                                        Load Timeline
                                    </button>
                                    </div>

                                <div className="timeline-results">
                                    {timelineData.length === 0 ? (
                                        <p className="no-results">Click "Load Timeline" to view recent files</p>
                                    ) : (
                                        timelineData.map((day, i) => (
                                            <div key={i} className="timeline-day">
                                                <h3 className="day-header">{day.date}</h3>
                                                <div className="day-files">
                                                    {day.files?.map((file: any, j: number) => (
                                                        <div key={j} className="timeline-file">
                                                            <span className="file-name">{file.name}</span>
                                                            <span className="file-path">{file.path}</span>
                                                        </div>
                                                    ))}
                                                </div>
                                            </div>
                                        ))
                                    )}
                                </div>
                            </>
                        )}
                    </section>
                )}

                {/* Git Assistant Section */}
                {activeSection === "git" && (
                    <section className="content-section">
                        <h2>📎 Git Clippy Assistant</h2>
                        <p className="section-desc">Your friendly git helper for ADHD developers.</p>
                        <GitAssistant repoPath={scanPath} indexPath={indexPath} />
                    </section>
                )}
            </main>
        </div>
    );
}
