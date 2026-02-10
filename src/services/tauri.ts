// Tauri command wrapper service
import { invoke } from "@tauri-apps/api/core";
import * as Types from "../types";

export const tauriService = {
    async scanDirectory(path: string, indexDir: string): Promise<Types.ScanResult> {
        return invoke("scan_directory", { path, indexDir });
    },

    async generateEmbeddings(indexDir: string, maxFiles?: number, batchSize?: number): Promise<Types.EmbedResult> {
        return invoke("generate_embeddings", { indexDir, maxFiles, batchSize });
    },

    async getEmbeddingProgress(indexDir: string): Promise<Types.BatchProgress> {
        return invoke("get_embedding_progress", { indexDir });
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

    async saveAzureConfig(
        indexDir: string,
        endpoint: string,
        apiKey: string,
        deploymentName: string,
        apiVersion?: string
    ): Promise<{ success: boolean; message: string }> {
        return invoke("save_azure_config", {
            indexDir,
            endpoint,
            apiKey,
            deploymentName,
            apiVersion,
        });
    },

    async loadAzureConfig(indexDir: string): Promise<Types.AzureConfigStatus> {
        return invoke("load_azure_config", { indexDir });
    },

    async validateAzureConfig(indexDir: string, endpoint: string, apiKey: string, deploymentName: string, apiVersion?: string): Promise<Types.AzureValidationResult> {
        return invoke("validate_azure_config", { indexDir, endpoint, apiKey, deploymentName, apiVersion });
    },

    async getClustersData(indexDir: string): Promise<Types.ClustersData> {
        return invoke("get_clusters_data", { indexDir });
    },

    // Git Assistant
    async getGitClippyReport(repoPath: string, indexDir?: string): Promise<Types.GitClippyReport> {
        return invoke("get_git_clippy_report", { repoPath, indexDir });
    },

    async executeClippyAction(repoPath: string, action: string, data?: any): Promise<{ success: boolean; output: string }> {
        return invoke("execute_clippy_action", { repoPath, action, data });
    },

    async isGitRepo(path: string): Promise<boolean> {
        return invoke("is_git_repo", { path });
    },
};
