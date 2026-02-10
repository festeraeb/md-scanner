use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: quick_embed <index_dir> [max_files] [batch_size]");
        std::process::exit(1);
    }

    let index_dir = args[1].clone();
    let max_files = args.get(2).and_then(|s| s.parse::<usize>().ok());
    let batch_size = args.get(3).and_then(|s| s.parse::<usize>().ok());

    println!("Starting quick embedding test for index: {} (max_files={:?}, batch_size={:?})", index_dir, max_files, batch_size);

    match wayfinder_tauri::commands::generate_embeddings(index_dir, max_files, batch_size).await {
        Ok(res) => println!("Embedding result: {}", serde_json::to_string_pretty(&res).unwrap_or_default()),
        Err(e) => eprintln!("Embedding failed: {}", e),
    }
}
