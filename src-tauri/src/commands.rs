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

// Error logging structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorLogEntry {
    pub timestamp: String,
    pub operation: String,
    pub file_path: Option<String>,
    pub error_message: String,
    pub error_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ErrorLog {
    pub entries: Vec<ErrorLogEntry>,
    pub last_updated: String,
}

// Batch processing progress
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchProgress {
    pub batch_id: String,
    pub total_files: usize,
    pub processed_files: usize,
    pub current_batch: usize,
    pub total_batches: usize,
    pub batch_size: usize,
    pub status: String,  // "running", "paused", "complete", "error"
    pub started_at: String,
    pub last_updated: String,
    pub errors: Vec<String>,
}

// Embedding job configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingJobConfig {
    pub batch_size: usize,        // Files per batch (default: 100)
    pub delay_ms: u64,            // Delay between requests (default: 50)
    pub max_retries: usize,       // Max retries per file (default: 3)
    pub save_interval: usize,     // Save progress every N files (default: 50)
    pub max_files: Option<usize>, // Limit total files (for testing)
}

impl Default for EmbeddingJobConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            delay_ms: 50,
            max_retries: 3,
            save_interval: 50,
            max_files: None,
        }
    }
}

// Helper to log errors to file
fn log_error(index_dir: &Path, operation: &str, file_path: Option<&str>, error_message: &str, error_code: Option<&str>) {
    let error_log_file = index_dir.join("error_log.json");
    
    let mut error_log: ErrorLog = if error_log_file.exists() {
        fs::read_to_string(&error_log_file)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        ErrorLog::default()
    };
    
    error_log.entries.push(ErrorLogEntry {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        operation: operation.to_string(),
        file_path: file_path.map(|s| s.to_string()),
        error_message: error_message.to_string(),
        error_code: error_code.map(|s| s.to_string()),
    });
    error_log.last_updated = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Keep only last 1000 errors
    if error_log.entries.len() > 1000 {
        error_log.entries = error_log.entries.split_off(error_log.entries.len() - 1000);
    }
    
    if let Ok(json) = serde_json::to_string_pretty(&error_log) {
        let _ = fs::write(&error_log_file, json);
    }
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

/// Generate embeddings using Azure OpenAI with auto-batching and progress saving
#[tauri::command(rename_all = "camelCase")]
pub async fn generate_embeddings(index_dir: String, max_files: Option<usize>, batch_size: Option<usize>) -> Result<serde_json::Value, String> {
    println!("[RUST] generate_embeddings called for: {}", index_dir);
    
    let index_path = Path::new(&index_dir);
    let index_file = index_path.join("index.json");
    let config_file = index_path.join("azure_config.json");
    let embeddings_file = index_path.join("embeddings.json");
    let progress_file = index_path.join("embedding_progress.json");
    
    // Configuration
    let config_batch_size = batch_size.unwrap_or(100);
    let save_interval = 50; // Save every 50 files
    let delay_ms = 50; // 50ms delay between requests
    
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
    
    // Apply max_files limit if specified
    let files_to_process: Vec<FileEntry> = if let Some(max) = max_files {
        index_data.files.into_iter().take(max).collect()
    } else {
        index_data.files
    };
    
    let total_files = files_to_process.len();
    let total_batches = (total_files + config_batch_size - 1) / config_batch_size;
    
    println!("[RUST] Processing {} files in {} batches of {}", total_files, total_batches, config_batch_size);
    
    // Load existing embeddings (for caching and resuming)
    let mut existing_embeddings: HashMap<String, FileEmbedding> = HashMap::new();
    if embeddings_file.exists() {
        if let Ok(content) = fs::read_to_string(&embeddings_file) {
            if let Ok(data) = serde_json::from_str::<EmbeddingsData>(&content) {
                println!("[RUST] Loaded {} existing embeddings from cache", data.embeddings.len());
                for emb in data.embeddings {
                    existing_embeddings.insert(emb.path.clone(), emb);
                }
            }
        }
    }
    
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    let mut new_embeddings: Vec<FileEmbedding> = existing_embeddings.values().cloned().collect();
    let processed_paths: std::collections::HashSet<String> = existing_embeddings.keys().cloned().collect();
    
    let mut cached_count = 0;
    let mut generated_count = 0;
    let mut error_count = 0;
    let mut skipped_count = 0;
    
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
    
    // Initialize progress
    let mut progress = BatchProgress {
        batch_id: format!("{}", Local::now().timestamp()),
        total_files,
        processed_files: 0,
        current_batch: 0,
        total_batches,
        batch_size: config_batch_size,
        status: "running".to_string(),
        started_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        last_updated: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        errors: Vec::new(),
    };
    
    // Save initial progress
    let _ = fs::write(&progress_file, serde_json::to_string_pretty(&progress).unwrap_or_default());
    
    for (i, file) in files_to_process.iter().enumerate() {
        // Skip if already processed
        if processed_paths.contains(&file.path) {
            cached_count += 1;
            continue;
        }
        
        // Read file content
        let content = match fs::read_to_string(&file.path) {
            Ok(c) => c,
            Err(e) => {
                skipped_count += 1;
                log_error(&index_path, "read_file", Some(&file.path), &e.to_string(), None);
                continue;
            }
        };
        
        // Skip empty files
        if content.trim().is_empty() {
            skipped_count += 1;
            continue;
        }
        
        // Simple hash of content for caching
        let content_hash = format!("{:x}", md5_hash(&content));
        
        // Truncate content to ~8000 tokens (roughly 32000 chars)
        let truncated_content = if content.len() > 32000 {
            content[..32000].to_string()
        } else {
            content.clone()
        };
        
        // Call Azure OpenAI with retry logic
        let request_body = serde_json::json!({
            "input": truncated_content
        });
        
        let mut retries = 0;
        let max_retries = 3;
        let mut success = false;
        
        while retries < max_retries && !success {
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
                                    content_hash: content_hash.clone(),
                                });
                                generated_count += 1;
                                success = true;
                            }
                        }
                    } else if response.status().as_u16() == 429 {
                        // Rate limited - wait and retry
                        let wait_time = 2u64.pow(retries as u32) * 1000;
                        println!("[RUST] Rate limited, waiting {}ms...", wait_time);
                        log_error(&index_path, "rate_limit", Some(&file.path), "Rate limited by Azure", Some("429"));
                        tokio::time::sleep(tokio::time::Duration::from_millis(wait_time)).await;
                        retries += 1;
                    } else {
                        let status = response.status();
                        let error_text = response.text().await.unwrap_or_default();
                        log_error(&index_path, "api_error", Some(&file.path), &error_text, Some(&status.to_string()));
                        error_count += 1;
                        progress.errors.push(format!("{}: {} - {}", file.name, status, error_text));
                        break;
                    }
                }
                Err(e) => {
                    if retries < max_retries - 1 {
                        let wait_time = 2u64.pow(retries as u32) * 500;
                        tokio::time::sleep(tokio::time::Duration::from_millis(wait_time)).await;
                        retries += 1;
                    } else {
                        log_error(&index_path, "request_error", Some(&file.path), &e.to_string(), None);
                        error_count += 1;
                        progress.errors.push(format!("{}: {}", file.name, e));
                        break;
                    }
                }
            }
        }
        
        // Update progress
        progress.processed_files = i + 1;
        progress.current_batch = (i / config_batch_size) + 1;
        progress.last_updated = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        // Log and save progress periodically
        if (i + 1) % save_interval == 0 || i == total_files - 1 {
            println!("[RUST] Progress: {}/{} files ({} generated, {} cached, {} errors)", 
                i + 1, total_files, generated_count, cached_count, error_count);
            
            // Save progress file
            let _ = fs::write(&progress_file, serde_json::to_string_pretty(&progress).unwrap_or_default());
            
            // Save embeddings periodically
            let embeddings_data = EmbeddingsData {
                embeddings: new_embeddings.clone(),
                model: config.deployment_name.clone(),
                created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            };
            
            if let Ok(json) = serde_json::to_string_pretty(&embeddings_data) {
                let _ = fs::write(&embeddings_file, json);
            }
        }
        
        // Delay between requests
        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
    }
    
    // Final save
    progress.status = "complete".to_string();
    progress.last_updated = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let _ = fs::write(&progress_file, serde_json::to_string_pretty(&progress).unwrap_or_default());
    
    let embeddings_data = EmbeddingsData {
        embeddings: new_embeddings.clone(),
        model: config.deployment_name.clone(),
        created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    
    let json = serde_json::to_string_pretty(&embeddings_data)
        .map_err(|e| format!("Failed to serialize embeddings: {}", e))?;
    
    fs::write(&embeddings_file, json)
        .map_err(|e| format!("Failed to write embeddings file: {}", e))?;
    
    println!("[RUST] Embeddings complete: {} generated, {} cached, {} skipped, {} errors", 
        generated_count, cached_count, skipped_count, error_count);
    
    Ok(serde_json::json!({
        "embeddings_generated": generated_count,
        "cached_count": cached_count,
        "skipped_count": skipped_count,
        "error_count": error_count,
        "total_files": new_embeddings.len(),
        "message": format!("Generated {} new embeddings, {} from cache, {} skipped, {} errors", 
            generated_count, cached_count, skipped_count, error_count)
    }))
}

/// Get embedding progress
#[tauri::command(rename_all = "camelCase")]
pub async fn get_embedding_progress(index_dir: String) -> Result<serde_json::Value, String> {
    let progress_file = Path::new(&index_dir).join("embedding_progress.json");
    
    if !progress_file.exists() {
        return Ok(serde_json::json!({
            "status": "not_started",
            "message": "No embedding job has been started"
        }));
    }
    
    let content = fs::read_to_string(&progress_file)
        .map_err(|e| format!("Failed to read progress file: {}", e))?;
    
    let progress: BatchProgress = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse progress file: {}", e))?;
    
    Ok(serde_json::to_value(&progress).unwrap_or_default())
}

/// Get error log
#[tauri::command(rename_all = "camelCase")]
pub async fn get_error_log(index_dir: String, limit: Option<usize>) -> Result<serde_json::Value, String> {
    let error_log_file = Path::new(&index_dir).join("error_log.json");
    
    if !error_log_file.exists() {
        return Ok(serde_json::json!({
            "entries": [],
            "message": "No errors logged"
        }));
    }
    
    let content = fs::read_to_string(&error_log_file)
        .map_err(|e| format!("Failed to read error log: {}", e))?;
    
    let mut error_log: ErrorLog = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse error log: {}", e))?;
    
    // Apply limit
    let limit = limit.unwrap_or(100);
    if error_log.entries.len() > limit {
        error_log.entries = error_log.entries.split_off(error_log.entries.len() - limit);
    }
    
    Ok(serde_json::to_value(&error_log).unwrap_or_default())
}

/// Clear error log
#[tauri::command(rename_all = "camelCase")]
pub async fn clear_error_log(index_dir: String) -> Result<serde_json::Value, String> {
    let error_log_file = Path::new(&index_dir).join("error_log.json");
    
    if error_log_file.exists() {
        fs::remove_file(&error_log_file)
            .map_err(|e| format!("Failed to delete error log: {}", e))?;
    }
    
    Ok(serde_json::json!({
        "success": true,
        "message": "Error log cleared"
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

/// Get timeline of file modifications
#[tauri::command(rename_all = "camelCase")]
pub async fn get_timeline(index_dir: String, days: usize) -> Result<serde_json::Value, String> {
    let index_path = Path::new(&index_dir);
    let index_file = index_path.join("index.json");
    
    if !index_file.exists() {
        return Err("Index not found. Please scan a directory first.".to_string());
    }
    
    let content = fs::read_to_string(&index_file)
        .map_err(|e| format!("Failed to read index: {}", e))?;
    
    let index_data: IndexData = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse index: {}", e))?;
    
    // Group files by date
    let mut files_by_date: std::collections::HashMap<String, Vec<serde_json::Value>> = std::collections::HashMap::new();
    
    for file in &index_data.files {
        // Parse the modified date and extract just the date part
        let date_part = if file.modified.len() >= 10 {
            file.modified[..10].to_string()
        } else {
            file.modified.clone()
        };
        
        files_by_date
            .entry(date_part)
            .or_insert_with(Vec::new)
            .push(serde_json::json!({
                "name": file.name,
                "path": file.path,
                "size": file.size,
                "modified": file.modified
            }));
    }
    
    // Sort dates in descending order and take only requested number of days
    let mut dates: Vec<String> = files_by_date.keys().cloned().collect();
    dates.sort_by(|a, b| b.cmp(a)); // Descending order (newest first)
    
    let timeline: Vec<serde_json::Value> = dates
        .into_iter()
        .take(days)
        .map(|date| {
            let files = files_by_date.get(&date).cloned().unwrap_or_default();
            serde_json::json!({
                "date": date,
                "files": files,
                "count": files.len()
            })
        })
        .collect();
    
    Ok(serde_json::json!({
        "timeline": timeline,
        "total_days": timeline.len(),
        "total_files": index_data.files.len()
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
    
    let config_file = index_path.join("azure_config.json");
    
    // If no new key provided, try to preserve existing key
    let final_api_key = if api_key.is_empty() {
        // Try to load existing config to get the key
        if config_file.exists() {
            let content = fs::read_to_string(&config_file).ok();
            content.and_then(|c| {
                serde_json::from_str::<AzureConfig>(&c).ok()
            }).map(|c| c.api_key).unwrap_or_default()
        } else {
            String::new()
        }
    } else {
        api_key
    };
    
    let config = AzureConfig {
        endpoint,
        api_key: final_api_key,
        deployment_name,
        api_version: api_version.unwrap_or_else(|| "2024-02-01".to_string()),
    };
    
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

/// Delete duplicate files - used by Git Clippy
#[tauri::command(rename_all = "camelCase")]
pub async fn delete_duplicate_files(file_paths: Vec<String>) -> Result<serde_json::Value, String> {
    println!("[RUST] delete_duplicate_files: {} files", file_paths.len());
    
    let mut deleted = 0;
    let mut errors: Vec<String> = Vec::new();
    
    for path in &file_paths {
        match fs::remove_file(path) {
            Ok(_) => {
                deleted += 1;
                println!("[RUST] Deleted: {}", path);
            }
            Err(e) => {
                let error_msg = format!("Failed to delete {}: {}", path, e);
                println!("[RUST] {}", error_msg);
                errors.push(error_msg);
            }
        }
    }
    
    Ok(serde_json::json!({
        "success": errors.is_empty(),
        "deleted": deleted,
        "errors": errors
    }))
}
