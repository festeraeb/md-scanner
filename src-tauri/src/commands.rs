// Tauri command handlers - Pure Rust implementation
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use chrono::{DateTime, Local, Duration};
use rand::Rng;

// Import git_assistant module from crate root
use crate::git_assistant;

// Azure OpenAI Configuration
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AzureConfig {
    pub endpoint: String,           // e.g., "https://your-resource.openai.azure.com"
    pub api_key: String,            // Your API key
    pub deployment_name: String,    // e.g., "text-embedding-ada-002"
    pub api_version: String,        // e.g., "2024-02-01"
}

// Embedding data stored per file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEmbedding {
    pub path: String,
    pub embedding: Vec<f32>,        // 1536 dimensions for ada-002
    pub content_hash: String,       // To detect if file changed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingsData {
    pub embeddings: Vec<FileEmbedding>,
    pub model: String,
    pub created_at: String,
}

// Cluster data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cluster {
    pub id: usize,
    pub centroid: Vec<f32>,
    pub file_paths: Vec<String>,
    pub label: Option<String>,      // Auto-generated label
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClustersData {
    pub clusters: Vec<Cluster>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanResult {
    pub files_scanned: usize,
    pub total_size: u64,
    pub index_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub modified: String,
    pub extension: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexData {
    pub files: Vec<FileEntry>,
    pub scan_path: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub path: String,
    pub name: String,
    pub score: f32,
    pub preview: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexStats {
    pub total_files: usize,
    pub total_size_bytes: u64,
    pub extensions: HashMap<String, usize>,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexState {
    pub has_files: bool,
    pub index_valid: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
}

// Pure Rust command handlers - no Python dependency

/// Scan a directory and create an index of text files
#[tauri::command(rename_all = "camelCase")]
pub async fn scan_directory(path: String, index_dir: String) -> Result<serde_json::Value, String> {
    println!("[RUST] scan_directory called - path: {}, index_dir: {}", path, index_dir);
    
    let scan_path = Path::new(&path);
    if !scan_path.exists() {
        println!("[RUST] Path does not exist: {}", path);
        return Err(format!("Path does not exist: {}", path));
    }
    println!("[RUST] Path exists, starting scan...");

    let mut files: Vec<FileEntry> = Vec::new();
    let mut total_size: u64 = 0;

    // Common text file extensions
    let text_extensions = vec![
        "md", "txt", "text", "markdown", "mdx",
        "py", "pyw", "pyi",
        "js", "jsx", "ts", "tsx",
        "json", "yaml", "yml", "toml", "ini", "cfg",
        "html", "htm", "css", "scss", "sass",
        "rs", "go", "java", "c", "cpp", "h", "hpp",
        "sh", "bash", "zsh", "ps1", "bat", "cmd",
        "xml", "svg", "log",
    ];

    for entry in WalkDir::new(&path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path();
        
        // Skip hidden files and directories
        if file_path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with('.'))
            .unwrap_or(false) 
        {
            continue;
        }

        if file_path.is_file() {
            let ext = file_path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            // Only index text files
            if text_extensions.contains(&ext.as_str()) {
                if let Ok(metadata) = fs::metadata(file_path) {
                    let size = metadata.len();
                    total_size += size;
                    
                    let modified = metadata.modified()
                        .ok()
                        .and_then(|t| DateTime::<Local>::from(t).format("%Y-%m-%d %H:%M:%S").to_string().into())
                        .unwrap_or_else(|| "Unknown".to_string());

                    files.push(FileEntry {
                        path: file_path.to_string_lossy().to_string(),
                        name: file_path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        size,
                        modified,
                        extension: ext,
                    });
                }
            }
        }
    }

    // Create index directory
    let index_path = if index_dir.is_empty() {
        Path::new(&path).join(".wayfinder_index")
    } else {
        Path::new(&index_dir).to_path_buf()
    };
    
    fs::create_dir_all(&index_path)
        .map_err(|e| format!("Failed to create index directory: {}", e))?;

    // Save index data
    let index_data = IndexData {
        files: files.clone(),
        scan_path: path.clone(),
        created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    let index_file = index_path.join("index.json");
    let json = serde_json::to_string_pretty(&index_data)
        .map_err(|e| format!("Failed to serialize index: {}", e))?;
    
    fs::write(&index_file, json)
        .map_err(|e| format!("Failed to write index file: {}", e))?;

    println!("[RUST] Scan complete - {} files found, {} bytes total", files.len(), total_size);
    println!("[RUST] Index written to: {}", index_file.display());
    
    Ok(serde_json::json!({
        "files_scanned": files.len(),
        "total_size": total_size,
        "index_path": index_path.to_string_lossy().to_string()
    }))
}

/// Generate embeddings using Azure OpenAI
#[tauri::command(rename_all = "camelCase")]
pub async fn generate_embeddings(index_dir: String) -> Result<serde_json::Value, String> {
    println!("[RUST] generate_embeddings called for: {}", index_dir);
    
    let index_path = Path::new(&index_dir);
    let index_file = index_path.join("index.json");
    let config_file = index_path.join("azure_config.json");
    let embeddings_file = index_path.join("embeddings.json");
    
    // Check if index exists
    if !index_file.exists() {
        return Err("Index not found. Please scan a directory first.".to_string());
    }
    
    // Load Azure config
    if !config_file.exists() {
        return Err("Azure config not found. Please configure Azure OpenAI settings first.".to_string());
    }
    
    let config_content = fs::read_to_string(&config_file)
        .map_err(|e| format!("Failed to read Azure config: {}", e))?;
    let config: AzureConfig = serde_json::from_str(&config_content)
        .map_err(|e| format!("Failed to parse Azure config: {}", e))?;
    
    if config.endpoint.is_empty() || config.api_key.is_empty() || config.deployment_name.is_empty() {
        return Err("Azure config is incomplete. Please set endpoint, API key, and deployment name.".to_string());
    }
    
    // Load index
    let index_content = fs::read_to_string(&index_file)
        .map_err(|e| format!("Failed to read index: {}", e))?;
    let index_data: IndexData = serde_json::from_str(&index_content)
        .map_err(|e| format!("Failed to parse index: {}", e))?;
    
    // Load existing embeddings if any
    let mut existing_embeddings: HashMap<String, FileEmbedding> = HashMap::new();
    if embeddings_file.exists() {
        if let Ok(content) = fs::read_to_string(&embeddings_file) {
            if let Ok(data) = serde_json::from_str::<EmbeddingsData>(&content) {
                for emb in data.embeddings {
                    existing_embeddings.insert(emb.path.clone(), emb);
                }
            }
        }
    }
    
    let client = reqwest::Client::new();
    let mut new_embeddings: Vec<FileEmbedding> = Vec::new();
    let mut cached_count = 0;
    let mut generated_count = 0;
    let mut error_count = 0;
    
    let api_version = if config.api_version.is_empty() { 
        "2024-02-01".to_string() 
    } else { 
        config.api_version.clone() 
    };
    
    let url = format!(
        "{}/openai/deployments/{}/embeddings?api-version={}",
        config.endpoint.trim_end_matches('/'),
        config.deployment_name,
        api_version
    );
    
    println!("[RUST] Embedding API URL: {}", url);
    
    for (i, file) in index_data.files.iter().enumerate() {
        // Read file content
        let content = match fs::read_to_string(&file.path) {
            Ok(c) => c,
            Err(_) => continue, // Skip files that can't be read
        };
        
        // Simple hash of content for caching
        let content_hash = format!("{:x}", md5_hash(&content));
        
        // Check if we already have this embedding
        if let Some(existing) = existing_embeddings.get(&file.path) {
            if existing.content_hash == content_hash {
                new_embeddings.push(existing.clone());
                cached_count += 1;
                continue;
            }
        }
        
        // Truncate content to ~8000 tokens (roughly 32000 chars for ada-002)
        let truncated_content = if content.len() > 32000 {
            content[..32000].to_string()
        } else {
            content.clone()
        };
        
        // Call Azure OpenAI
        let request_body = serde_json::json!({
            "input": truncated_content
        });
        
        match client
            .post(&url)
            .header("api-key", &config.api_key)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    if let Ok(json) = response.json::<serde_json::Value>().await {
                        if let Some(embedding) = json["data"][0]["embedding"].as_array() {
                            let emb_vec: Vec<f32> = embedding
                                .iter()
                                .filter_map(|v| v.as_f64().map(|f| f as f32))
                                .collect();
                            
                            new_embeddings.push(FileEmbedding {
                                path: file.path.clone(),
                                embedding: emb_vec,
                                content_hash,
                            });
                            generated_count += 1;
                            
                            if (i + 1) % 10 == 0 {
                                println!("[RUST] Progress: {}/{} files processed", i + 1, index_data.files.len());
                            }
                        }
                    }
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    println!("[RUST] API error for {}: {} - {}", file.name, status, error_text);
                    error_count += 1;
                }
            }
            Err(e) => {
                println!("[RUST] Request error for {}: {}", file.name, e);
                error_count += 1;
            }
        }
        
        // Small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    // Save embeddings
    let embeddings_data = EmbeddingsData {
        embeddings: new_embeddings.clone(),
        model: config.deployment_name.clone(),
        created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    
    let json = serde_json::to_string_pretty(&embeddings_data)
        .map_err(|e| format!("Failed to serialize embeddings: {}", e))?;
    
    fs::write(&embeddings_file, json)
        .map_err(|e| format!("Failed to write embeddings file: {}", e))?;
    
    println!("[RUST] Embeddings complete: {} generated, {} cached, {} errors", 
        generated_count, cached_count, error_count);
    
    Ok(serde_json::json!({
        "embeddings_generated": generated_count,
        "cached_count": cached_count,
        "error_count": error_count,
        "total_files": new_embeddings.len(),
        "message": format!("Generated {} new embeddings, {} from cache", generated_count, cached_count)
    }))
}

// Simple MD5-like hash for content comparison
fn md5_hash(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Create clusters using k-means algorithm
#[tauri::command(rename_all = "camelCase")]
pub async fn create_clusters(index_dir: String, num_clusters: Option<usize>) -> Result<serde_json::Value, String> {
    println!("[RUST] create_clusters called for: {}", index_dir);
    
    let index_path = Path::new(&index_dir);
    let embeddings_file = index_path.join("embeddings.json");
    let clusters_file = index_path.join("clusters.json");
    
    // Load embeddings
    if !embeddings_file.exists() {
        return Err("Embeddings not found. Please generate embeddings first.".to_string());
    }
    
    let content = fs::read_to_string(&embeddings_file)
        .map_err(|e| format!("Failed to read embeddings: {}", e))?;
    let embeddings_data: EmbeddingsData = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse embeddings: {}", e))?;
    
    if embeddings_data.embeddings.is_empty() {
        return Err("No embeddings found. Please generate embeddings first.".to_string());
    }
    
    // Determine number of clusters (default: sqrt of file count, min 2, max 20)
    let k = num_clusters.unwrap_or_else(|| {
        let sqrt = (embeddings_data.embeddings.len() as f64).sqrt() as usize;
        sqrt.max(2).min(20)
    });
    
    println!("[RUST] Clustering {} files into {} clusters", embeddings_data.embeddings.len(), k);
    
    // Run k-means clustering
    let clusters = kmeans_cluster(&embeddings_data.embeddings, k);
    
    // Save clusters
    let clusters_data = ClustersData {
        clusters: clusters.clone(),
        created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    
    let json = serde_json::to_string_pretty(&clusters_data)
        .map_err(|e| format!("Failed to serialize clusters: {}", e))?;
    
    fs::write(&clusters_file, json)
        .map_err(|e| format!("Failed to write clusters file: {}", e))?;
    
    println!("[RUST] Clustering complete: {} clusters created", clusters.len());
    
    Ok(serde_json::json!({
        "clusters_created": clusters.len(),
        "total_files": embeddings_data.embeddings.len(),
        "message": format!("Created {} clusters from {} files", clusters.len(), embeddings_data.embeddings.len())
    }))
}

/// K-means clustering implementation
fn kmeans_cluster(embeddings: &[FileEmbedding], k: usize) -> Vec<Cluster> {
    if embeddings.is_empty() || k == 0 {
        return Vec::new();
    }
    
    let dim = embeddings[0].embedding.len();
    let mut rng = rand::thread_rng();
    
    // Initialize centroids randomly from the embeddings
    let mut centroids: Vec<Vec<f32>> = Vec::with_capacity(k);
    let mut used_indices: Vec<usize> = Vec::new();
    
    for _ in 0..k.min(embeddings.len()) {
        let mut idx = rng.gen_range(0..embeddings.len());
        while used_indices.contains(&idx) {
            idx = rng.gen_range(0..embeddings.len());
        }
        used_indices.push(idx);
        centroids.push(embeddings[idx].embedding.clone());
    }
    
    // Run k-means for 50 iterations
    let mut assignments: Vec<usize> = vec![0; embeddings.len()];
    
    for iteration in 0..50 {
        // Assign each embedding to nearest centroid
        let mut changed = false;
        for (i, emb) in embeddings.iter().enumerate() {
            let mut min_dist = f32::MAX;
            let mut min_idx = 0;
            
            for (j, centroid) in centroids.iter().enumerate() {
                let dist = cosine_distance(&emb.embedding, centroid);
                if dist < min_dist {
                    min_dist = dist;
                    min_idx = j;
                }
            }
            
            if assignments[i] != min_idx {
                assignments[i] = min_idx;
                changed = true;
            }
        }
        
        if !changed {
            println!("[RUST] K-means converged at iteration {}", iteration);
            break;
        }
        
        // Update centroids
        for j in 0..centroids.len() {
            let mut new_centroid = vec![0.0f32; dim];
            let mut count = 0;
            
            for (i, emb) in embeddings.iter().enumerate() {
                if assignments[i] == j {
                    for (d, val) in emb.embedding.iter().enumerate() {
                        new_centroid[d] += val;
                    }
                    count += 1;
                }
            }
            
            if count > 0 {
                for val in new_centroid.iter_mut() {
                    *val /= count as f32;
                }
                centroids[j] = new_centroid;
            }
        }
    }
    
    // Build cluster results
    let mut clusters: Vec<Cluster> = Vec::with_capacity(k);
    
    for j in 0..centroids.len() {
        let file_paths: Vec<String> = embeddings
            .iter()
            .enumerate()
            .filter(|(i, _)| assignments[*i] == j)
            .map(|(_, emb)| emb.path.clone())
            .collect();
        
        if !file_paths.is_empty() {
            clusters.push(Cluster {
                id: j,
                centroid: centroids[j].clone(),
                file_paths,
                label: None, // Could be auto-generated from common terms
            });
        }
    }
    
    clusters
}

/// Cosine distance between two vectors (1 - cosine similarity)
fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;
    
    for i in 0..a.len().min(b.len()) {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    
    let similarity = dot / (norm_a.sqrt() * norm_b.sqrt() + 1e-10);
    1.0 - similarity
}

/// Search indexed files by query string
#[tauri::command(rename_all = "camelCase")]
pub async fn search(
    query: String,
    index_dir: String,
    top_k: usize,
    semantic_weight: f32,
) -> Result<serde_json::Value, String> {
    let index_path = Path::new(&index_dir);
    let index_file = index_path.join("index.json");
    
    if !index_file.exists() {
        return Err("Index not found. Please scan a directory first.".to_string());
    }

    let content = fs::read_to_string(&index_file)
        .map_err(|e| format!("Failed to read index: {}", e))?;
    
    let index_data: IndexData = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse index: {}", e))?;

    let query_lower = query.to_lowercase();
    let mut results: Vec<SearchResult> = Vec::new();

    for file in &index_data.files {
        let name_lower = file.name.to_lowercase();
        let path_lower = file.path.to_lowercase();
        
        // Simple text matching score
        let mut score: f32 = 0.0;
        
        if name_lower.contains(&query_lower) {
            score += 1.0;
        }
        if path_lower.contains(&query_lower) {
            score += 0.5;
        }

        // Try to search within file content
        if let Ok(content) = fs::read_to_string(&file.path) {
            if content.to_lowercase().contains(&query_lower) {
                score += 0.8;
                
                // Get a preview snippet
                let content_lower = content.to_lowercase();
                if let Some(pos) = content_lower.find(&query_lower) {
                    let start = pos.saturating_sub(50);
                    let end = (pos + query.len() + 50).min(content.len());
                    let preview = &content[start..end];
                    
                    if score > 0.0 {
                        results.push(SearchResult {
                            path: file.path.clone(),
                            name: file.name.clone(),
                            score,
                            preview: Some(preview.trim().to_string()),
                        });
                    }
                    continue;
                }
            }
        }

        if score > 0.0 {
            results.push(SearchResult {
                path: file.path.clone(),
                name: file.name.clone(),
                score,
                preview: None,
            });
        }
    }

    // Sort by score descending and take top_k
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(top_k);

    Ok(serde_json::to_value(results).unwrap())
}

/// Get summary of clusters (stub)
#[tauri::command(rename_all = "camelCase")]
pub async fn get_clusters_summary(index_dir: String) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!([]))
}

/// Get timeline of file modifications (stub)
#[tauri::command(rename_all = "camelCase")]
pub async fn get_timeline(index_dir: String, days: usize) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "entries": []
    }))
}

/// Get index statistics
#[tauri::command(rename_all = "camelCase")]
pub async fn get_stats(index_dir: String) -> Result<serde_json::Value, String> {
    let index_path = Path::new(&index_dir);
    let index_file = index_path.join("index.json");
    
    if !index_file.exists() {
        return Err("Index not found".to_string());
    }

    let content = fs::read_to_string(&index_file)
        .map_err(|e| format!("Failed to read index: {}", e))?;
    
    let index_data: IndexData = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse index: {}", e))?;

    let mut total_size: u64 = 0;
    let mut extensions: HashMap<String, usize> = HashMap::new();

    for file in &index_data.files {
        total_size += file.size;
        *extensions.entry(file.extension.clone()).or_insert(0) += 1;
    }

    Ok(serde_json::json!({
        "total_files": index_data.files.len(),
        "total_size_bytes": total_size,
        "extensions": extensions,
        "last_updated": index_data.created_at,
        "scan_path": index_data.scan_path
    }))
}

/// Validate if index exists and is valid
#[tauri::command(rename_all = "camelCase")]
pub async fn validate_index(index_dir: String) -> Result<serde_json::Value, String> {
    let index_path = Path::new(&index_dir);
    let index_file = index_path.join("index.json");
    
    if !index_file.exists() {
        return Ok(serde_json::json!({
            "has_files": false,
            "index_valid": false,
            "message": "Index not found"
        }));
    }

    let content = fs::read_to_string(&index_file);
    match content {
        Ok(c) => {
            match serde_json::from_str::<IndexData>(&c) {
                Ok(data) => Ok(serde_json::json!({
                    "has_files": !data.files.is_empty(),
                    "index_valid": true,
                    "message": format!("Index valid with {} files", data.files.len())
                })),
                Err(_) => Ok(serde_json::json!({
                    "has_files": false,
                    "index_valid": false,
                    "message": "Index file is corrupted"
                }))
            }
        },
        Err(_) => Ok(serde_json::json!({
            "has_files": false,
            "index_valid": false,
            "message": "Cannot read index file"
        }))
    }
}

/// Get system information
#[tauri::command]
pub async fn get_system_info() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH
    }))
}

/// Save Azure OpenAI configuration
#[tauri::command(rename_all = "camelCase")]
pub async fn save_azure_config(
    index_dir: String,
    endpoint: String,
    api_key: String,
    deployment_name: String,
    api_version: Option<String>,
) -> Result<serde_json::Value, String> {
    println!("[RUST] save_azure_config called");
    
    let index_path = Path::new(&index_dir);
    fs::create_dir_all(&index_path)
        .map_err(|e| format!("Failed to create index directory: {}", e))?;
    
    let config = AzureConfig {
        endpoint,
        api_key,
        deployment_name,
        api_version: api_version.unwrap_or_else(|| "2024-02-01".to_string()),
    };
    
    let config_file = index_path.join("azure_config.json");
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    
    fs::write(&config_file, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;
    
    Ok(serde_json::json!({
        "success": true,
        "message": "Azure config saved successfully"
    }))
}

/// Load Azure OpenAI configuration
#[tauri::command(rename_all = "camelCase")]
pub async fn load_azure_config(index_dir: String) -> Result<serde_json::Value, String> {
    let index_path = Path::new(&index_dir);
    let config_file = index_path.join("azure_config.json");
    
    if !config_file.exists() {
        return Ok(serde_json::json!({
            "configured": false,
            "endpoint": "",
            "deployment_name": "",
            "api_version": "2024-02-01"
        }));
    }
    
    let content = fs::read_to_string(&config_file)
        .map_err(|e| format!("Failed to read config: {}", e))?;
    
    let config: AzureConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    
    Ok(serde_json::json!({
        "configured": !config.api_key.is_empty(),
        "endpoint": config.endpoint,
        "deployment_name": config.deployment_name,
        "api_version": config.api_version,
        "has_key": !config.api_key.is_empty()
    }))
}

/// Get clusters summary for display
#[tauri::command(rename_all = "camelCase")]
pub async fn get_clusters_data(index_dir: String) -> Result<serde_json::Value, String> {
    let index_path = Path::new(&index_dir);
    let clusters_file = index_path.join("clusters.json");
    
    if !clusters_file.exists() {
        return Ok(serde_json::json!({
            "has_clusters": false,
            "clusters": []
        }));
    }
    
    let content = fs::read_to_string(&clusters_file)
        .map_err(|e| format!("Failed to read clusters: {}", e))?;
    
    let clusters_data: ClustersData = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse clusters: {}", e))?;
    
    // Return cluster summaries (without full centroids for UI)
    let clusters_summary: Vec<serde_json::Value> = clusters_data.clusters.iter().map(|c| {
        serde_json::json!({
            "id": c.id,
            "file_count": c.file_paths.len(),
            "files": c.file_paths,
            "label": c.label
        })
    }).collect();
    
    Ok(serde_json::json!({
        "has_clusters": true,
        "clusters": clusters_summary,
        "created_at": clusters_data.created_at
    }))
}

/// Get Git Clippy report for a repository
#[tauri::command(rename_all = "camelCase")]
pub async fn get_git_clippy_report(repo_path: String, index_dir: Option<String>) -> Result<serde_json::Value, String> {
    println!("[RUST] get_git_clippy_report called for: {}", repo_path);
    
    // Load index data if available
    let index_files = if let Some(ref dir) = index_dir {
        let index_file = Path::new(dir).join("index.json");
        if index_file.exists() {
            let content = fs::read_to_string(&index_file).ok();
            content.and_then(|c| serde_json::from_str::<IndexData>(&c).ok())
                .map(|d| d.files)
        } else {
            None
        }
    } else {
        None
    };
    
    let report = git_assistant::generate_clippy_report(&repo_path, index_files.as_deref())?;
    
    serde_json::to_value(report)
        .map_err(|e| format!("Failed to serialize report: {}", e))
}

/// Execute a Git Clippy action
#[tauri::command(rename_all = "camelCase")]
pub async fn execute_clippy_action(
    repo_path: String, 
    action: String, 
    data: Option<serde_json::Value>
) -> Result<serde_json::Value, String> {
    println!("[RUST] execute_clippy_action: {} for {}", action, repo_path);
    
    let result = git_assistant::execute_git_action(&repo_path, &action, data.as_ref())?;
    
    Ok(serde_json::json!({
        "success": true,
        "output": result
    }))
}

/// Check if path is a git repository
#[tauri::command(rename_all = "camelCase")]
pub async fn is_git_repo(path: String) -> Result<bool, String> {
    Ok(git_assistant::is_git_repo(&path))
}
