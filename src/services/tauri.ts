// Tauri command wrapper service
import { invoke } from "@tauri-apps/api/core";
import * as Types from "../types";

export const tauriService = {
    async scanDirectory(path: string, indexDir: string): Promise<Types.ScanResult> {
        return invoke("scan_directory", { path, indexDir });
    },

    async generateEmbeddings(indexDir: string): Promise<Types.EmbedResult> {
        return invoke("generate_embeddings", { indexDir });
    },

    async createClusters(
        indexDir: string,
        numClusters?: number
    ): Promise<Types.ClusterResult> {
        return invoke("create_clusters", {
            indexDir,
            numClusters,
        });
    },

    async search(
        query: string,
        indexDir: string,
        topK: number = 10,
        semanticWeight: number = 0.7
    ): Promise<Types.SearchResult[]> {
        return invoke("search", {
            query,
            indexDir,
            topK,
            semanticWeight,
        });
    },

    async getClustersSummary(indexDir: string): Promise<Types.ClustersSummary> {
        return invoke("get_clusters_summary", { indexDir });
    },

    async getTimeline(indexDir: string, days: number = 30): Promise<Types.TimelineData> {
        return invoke("get_timeline", { indexDir, days });
    },

    async getStats(indexDir: string): Promise<Types.IndexStats> {
        return invoke("get_stats", { indexDir });
    },

    async validateIndex(indexDir: string): Promise<Types.IndexState> {
        return invoke("validate_index", { indexDir });
    },

    async getSystemInfo(): Promise<Types.SystemInfo> {
        return invoke("get_system_info");
    },
};
