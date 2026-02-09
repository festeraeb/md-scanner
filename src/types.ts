// TypeScript types for Tauri commands and responses

export interface ScanResult {
    files_scanned: number;
    total_size: number;
    index_path: string;
}

export interface EmbedResult {
    embeddings_generated: number;
    cached_count: number;
    message?: string;
}

export interface ClusterResult {
    clusters_created: number;
    total_files: number;
    message?: string;
}

export interface SearchResult {
    path: string;
    name: string;
    score: number;
    preview?: string;
}

export interface ClusterInfo {
    id: number;
    size: number;
    sample_files: string[];
    summary?: string;
    label?: string;
    file_count?: number;
    keywords?: string[];
    files?: string[];
}

export interface TimelineEntry {
    date: string;
    file_count: number;
    files: { name: string; path: string }[];
}

export interface TimelineData {
    entries?: TimelineEntry[];
    timeline?: TimelineEntry[];
}

export interface IndexStats {
    total_files: number;
    total_size_bytes: number;
    extensions: Record<string, number>;
    last_updated: string;
    scan_path?: string;
    has_embeddings?: boolean;
    has_clusters?: boolean;
    cluster_count?: number;
}

export interface AgeBucket {
    label: string;
    count: number;
}

export interface IndexState {
    has_files: boolean;
    index_valid: boolean;
    message: string;
}

export interface SystemInfo {
    os: string;
    arch: string;
}

// Cluster summary response
export interface ClustersSummary {
    clusters?: ClusterInfo[];
}

// UI State types
export interface OperationProgress {
    operation: string;
    current: number;
    total: number;
    percent: number;
    status: "pending" | "running" | "complete" | "error";
    error?: string;
}
