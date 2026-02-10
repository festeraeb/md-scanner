// Git Clippy Assistant - Helpful git companion for ADHD developers

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use crate::commands::FileEntry;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitStatus {
    pub is_repo: bool,
    pub branch: String,
    pub uncommitted_files: usize,
    pub staged_files: usize,
    pub untracked_files: usize,
    pub days_since_commit: i64,
    pub last_commit_message: Option<String>,
    pub last_commit_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DuplicateFile {
    pub original: String,
    pub duplicates: Vec<String>,
    pub content_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileSuggestion {
    pub file_path: String,
    pub suggestion: String,
    pub action: String,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitSuggestion {
    pub files: Vec<String>,
    pub suggested_message: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitClippyReport {
    pub status: GitStatus,
    pub urgency_level: String,
    pub message: String,
    pub suggestions: Vec<ClippySuggestion>,
    pub duplicates: Vec<DuplicateFile>,
    pub commit_suggestions: Vec<CommitSuggestion>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClippySuggestion {
    pub id: String,
    pub icon: String,
    pub title: String,
    pub description: String,
    pub actions: Vec<ClippyAction>,
    pub priority: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClippyAction {
    pub label: String,
    pub action_type: String,
    pub data: Option<serde_json::Value>,
}

/// Run a git command and return output
fn run_git_command(repo_path: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_path)
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Check if a path is a git repository
pub fn is_git_repo(path: &str) -> bool {
    Path::new(path).join(".git").exists()
}

/// Get comprehensive git status
pub fn get_git_status(repo_path: &str) -> Result<GitStatus, String> {
    if !is_git_repo(repo_path) {
        return Ok(GitStatus {
            is_repo: false,
            branch: String::new(),
            uncommitted_files: 0,
            staged_files: 0,
            untracked_files: 0,
            days_since_commit: 0,
            last_commit_message: None,
            last_commit_date: None,
        });
    }

    // Get current branch
    let branch = run_git_command(repo_path, &["branch", "--show-current"])
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();

    // Get status --porcelain for file counts
    let status_output = run_git_command(repo_path, &["status", "--porcelain"]).unwrap_or_default();

    let mut staged = 0;
    let mut uncommitted = 0;
    let mut untracked = 0;

    for line in status_output.lines() {
        if line.len() < 2 {
            continue;
        }
        let status_chars: Vec<char> = line.chars().take(2).collect();

        match (status_chars.get(0), status_chars.get(1)) {
            (Some('?'), Some('?')) => untracked += 1,
            (Some(' '), Some(_)) => uncommitted += 1,
            (Some(_), Some(' ')) => staged += 1,
            (Some(_), Some(_)) => {
                staged += 1;
                uncommitted += 1;
            }
            _ => {}
        }
    }

    // Get last commit info
    let last_commit_message = run_git_command(repo_path, &["log", "-1", "--format=%s"])
        .ok()
        .map(|s| s.trim().to_string());

    let last_commit_date = run_git_command(repo_path, &["log", "-1", "--format=%ci"])
        .ok()
        .map(|s| s.trim().to_string());

    // Calculate days since last commit
    let days_since_commit = if let Some(ref date_str) = last_commit_date {
        if let Ok(commit_date) = DateTime::parse_from_str(
            &format!("{} +0000", &date_str.get(..19).unwrap_or(date_str)),
            "%Y-%m-%d %H:%M:%S %z",
        ) {
            let now = Utc::now();
            let duration = now.signed_duration_since(commit_date.with_timezone(&Utc));
            duration.num_days()
        } else {
            0
        }
    } else {
        0
    };

    Ok(GitStatus {
        is_repo: true,
        branch,
        uncommitted_files: uncommitted + untracked,
        staged_files: staged,
        untracked_files: untracked,
        days_since_commit,
        last_commit_message,
        last_commit_date,
    })
}

/// Find duplicate files in the repository
pub fn find_duplicates(files: &[FileEntry]) -> Vec<DuplicateFile> {
    use std::collections::hash_map::DefaultHasher;
    use std::fs;
    use std::hash::{Hash, Hasher};

    let mut content_map: HashMap<u64, Vec<String>> = HashMap::new();

    for file in files {
        if let Ok(content) = fs::read_to_string(&file.path) {
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            let hash = hasher.finish();

            content_map
                .entry(hash)
                .or_insert_with(Vec::new)
                .push(file.path.clone());
        }
    }

    let mut duplicates: Vec<DuplicateFile> = Vec::new();

    for (hash, paths) in content_map {
        if paths.len() > 1 {
            let mut sorted_paths = paths.clone();
            sorted_paths.sort_by(|a, b| a.len().cmp(&b.len()));

            let original = sorted_paths.remove(0);
            duplicates.push(DuplicateFile {
                original,
                duplicates: sorted_paths,
                content_hash: format!("{:x}", hash),
            });
        }
    }

    duplicates
}

/// Detect copy/backup naming patterns
pub fn detect_copy_patterns(files: &[FileEntry]) -> Vec<FileSuggestion> {
    let copy_patterns = [
        "_copy", "_backup", "_old", "_new", "_working", "_temp", "_COPY", "_BACKUP", "_OLD",
        "_NEW", "_WORKING", "_TEMP", " copy", " Copy", " (copy)", " - Copy", "_v1", "_v2", "_v3",
        "_final", "_FINAL",
    ];

    let mut suggestions: Vec<FileSuggestion> = Vec::new();

    for file in files {
        let name = &file.name;
        let stem = Path::new(name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        for pattern in &copy_patterns {
            if stem.contains(pattern) {
                let potential_original = stem.replace(pattern, "");
                let ext = Path::new(name)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");

                suggestions.push(FileSuggestion {
                    file_path: file.path.clone(),
                    suggestion: format!("Looks like a copy of '{}.{}'", potential_original, ext),
                    action: "review".to_string(),
                    reason: format!("Contains '{}' pattern", pattern),
                });
                break;
            }
        }
    }

    suggestions
}

/// Generate smart commit suggestions based on file patterns
pub fn suggest_commits(repo_path: &str) -> Result<Vec<CommitSuggestion>, String> {
    let status_output = run_git_command(repo_path, &["status", "--porcelain"])?;

    let mut files_by_dir: HashMap<String, Vec<String>> = HashMap::new();
    let mut files_by_ext: HashMap<String, Vec<String>> = HashMap::new();

    for line in status_output.lines() {
        if line.len() < 3 {
            continue;
        }
        let file_path = line[3..].trim();

        let dir = Path::new(file_path)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("root")
            .to_string();

        files_by_dir
            .entry(dir)
            .or_insert_with(Vec::new)
            .push(file_path.to_string());

        let ext = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("none")
            .to_string();

        files_by_ext
            .entry(ext)
            .or_insert_with(Vec::new)
            .push(file_path.to_string());
    }

    let mut suggestions: Vec<CommitSuggestion> = Vec::new();

    for (dir, files) in &files_by_dir {
        if files.len() >= 2 {
            let dir_name = Path::new(dir)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(dir);

            suggestions.push(CommitSuggestion {
                files: files.clone(),
                suggested_message: format!("Update {} files in {}", files.len(), dir_name),
                category: "feature".to_string(),
            });
        }
    }

    let config_exts = ["json", "yaml", "yml", "toml", "ini", "cfg"];
    let config_files: Vec<String> = files_by_ext
        .iter()
        .filter(|(ext, _)| config_exts.contains(&ext.as_str()))
        .flat_map(|(_, files)| files.clone())
        .collect();

    if config_files.len() >= 2 {
        suggestions.push(CommitSuggestion {
            files: config_files,
            suggested_message: "Update configuration files".to_string(),
            category: "config".to_string(),
        });
    }

    let doc_exts = ["md", "txt", "rst", "doc"];
    let doc_files: Vec<String> = files_by_ext
        .iter()
        .filter(|(ext, _)| doc_exts.contains(&ext.as_str()))
        .flat_map(|(_, files)| files.clone())
        .collect();

    if !doc_files.is_empty() {
        suggestions.push(CommitSuggestion {
            files: doc_files,
            suggested_message: "Update documentation".to_string(),
            category: "docs".to_string(),
        });
    }

    Ok(suggestions)
}

/// Generate the full Clippy report
pub fn generate_clippy_report(
    repo_path: &str,
    index_files: Option<&[FileEntry]>,
) -> Result<GitClippyReport, String> {
    let status = get_git_status(repo_path)?;

    let mut suggestions: Vec<ClippySuggestion> = Vec::new();
    let mut duplicates: Vec<DuplicateFile> = Vec::new();

    // Check uncommitted files
    if status.uncommitted_files > 50 {
        suggestions.push(ClippySuggestion {
            id: "many_uncommitted".to_string(),
            icon: "⚠️".to_string(),
            title: format!("{} uncommitted files", status.uncommitted_files),
            description:
                "That's a lot of changes. Your future self might thank you for committing."
                    .to_string(),
            actions: vec![
                ClippyAction {
                    label: "Smart commit".to_string(),
                    action_type: "commit".to_string(),
                    data: None,
                },
                ClippyAction {
                    label: "Review changes".to_string(),
                    action_type: "review".to_string(),
                    data: None,
                },
                ClippyAction {
                    label: "Panic mode (commit all as WIP)".to_string(),
                    action_type: "wip_commit".to_string(),
                    data: None,
                },
            ],
            priority: 4,
        });
    } else if status.uncommitted_files > 10 {
        suggestions.push(ClippySuggestion {
            id: "some_uncommitted".to_string(),
            icon: "📝".to_string(),
            title: format!("{} uncommitted files", status.uncommitted_files),
            description: "Good progress! Consider committing related changes together.".to_string(),
            actions: vec![
                ClippyAction {
                    label: "Smart commit".to_string(),
                    action_type: "commit".to_string(),
                    data: None,
                },
                ClippyAction {
                    label: "Later".to_string(),
                    action_type: "dismiss".to_string(),
                    data: None,
                },
            ],
            priority: 2,
        });
    }

    // Check days since commit
    if status.days_since_commit > 7 {
        suggestions.push(ClippySuggestion {
            id: "long_since_commit".to_string(),
            icon: "⏰".to_string(),
            title: format!("{} days since last commit", status.days_since_commit),
            description: "It's been a while! Even a WIP commit is better than losing work."
                .to_string(),
            actions: vec![
                ClippyAction {
                    label: "Quick WIP save".to_string(),
                    action_type: "wip_commit".to_string(),
                    data: None,
                },
                ClippyAction {
                    label: "I know what I'm doing".to_string(),
                    action_type: "dismiss".to_string(),
                    data: None,
                },
            ],
            priority: 5,
        });
    } else if status.days_since_commit > 3 {
        suggestions.push(ClippySuggestion {
            id: "few_days_since_commit".to_string(),
            icon: "💭".to_string(),
            title: format!("{} days since last commit", status.days_since_commit),
            description: "Just a friendly reminder to save your progress.".to_string(),
            actions: vec![
                ClippyAction {
                    label: "Commit now".to_string(),
                    action_type: "commit".to_string(),
                    data: None,
                },
                ClippyAction {
                    label: "Remind me tomorrow".to_string(),
                    action_type: "snooze".to_string(),
                    data: Some(serde_json::json!({"hours": 24})),
                },
            ],
            priority: 2,
        });
    }

    // Find duplicates if we have index data
    if let Some(files) = index_files {
        duplicates = find_duplicates(files);

        if !duplicates.is_empty() {
            let total_dupes: usize = duplicates.iter().map(|d| d.duplicates.len()).sum();

            suggestions.push(ClippySuggestion {
                id: "duplicates_found".to_string(),
                icon: "🗑️".to_string(),
                title: format!("{} duplicate files found", total_dupes),
                description: "Same content, different names. Want to clean up?".to_string(),
                actions: vec![
                    ClippyAction {
                        label: "Show me".to_string(),
                        action_type: "show_duplicates".to_string(),
                        data: None,
                    },
                    ClippyAction {
                        label: "Clean up safely".to_string(),
                        action_type: "cleanup".to_string(),
                        data: None,
                    },
                    ClippyAction {
                        label: "I'm a hoarder, leave them".to_string(),
                        action_type: "dismiss".to_string(),
                        data: None,
                    },
                ],
                priority: 3,
            });
        }

        let copy_suggestions = detect_copy_patterns(files);
        if copy_suggestions.len() > 3 {
            suggestions.push(ClippySuggestion {
                id: "copy_patterns".to_string(),
                icon: "📋".to_string(),
                title: format!("{} files with copy/backup patterns", copy_suggestions.len()),
                description: "Found files like '_copy', '_old', '_backup'. Need help organizing?"
                    .to_string(),
                actions: vec![
                    ClippyAction {
                        label: "Review them".to_string(),
                        action_type: "show_copies".to_string(),
                        data: None,
                    },
                    ClippyAction {
                        label: "Ignore".to_string(),
                        action_type: "dismiss".to_string(),
                        data: None,
                    },
                ],
                priority: 2,
            });
        }
    }

    // Check if on main branch with uncommitted changes
    if (status.branch == "main" || status.branch == "master") && status.uncommitted_files > 5 {
        suggestions.push(ClippySuggestion {
            id: "experimenting_on_main".to_string(),
            icon: "🌿".to_string(),
            title: "Experimenting on main branch".to_string(),
            description: "You're making changes directly on main. Want to create a feature branch?".to_string(),
            actions: vec![
                ClippyAction {
                    label: "Create branch".to_string(),
                    action_type: "create_branch".to_string(),
                    data: Some(serde_json::json!({
                        "suggested_name": format!("feature-{}", chrono::Local::now().format("%Y%m%d"))
                    })),
                },
                ClippyAction {
                    label: "YOLO on main".to_string(),
                    action_type: "dismiss".to_string(),
                    data: None,
                },
            ],
            priority: 3,
        });
    }

    let commit_suggestions = suggest_commits(repo_path).unwrap_or_default();

    suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));

    let urgency_level = if status.days_since_commit > 7 && status.uncommitted_files > 50 {
        "panic"
    } else if status.days_since_commit > 3 || status.uncommitted_files > 20 {
        "warning"
    } else if status.uncommitted_files > 5 {
        "nudge"
    } else {
        "chill"
    };

    let message = match urgency_level {
        "panic" => "📎 Oh dear. We should probably talk about your commit habits...".to_string(),
        "warning" => "📎 Hey! Just checking in. Things are getting a bit messy.".to_string(),
        "nudge" => "📎 Looking good! A few suggestions when you have a moment.".to_string(),
        _ => "📎 All clear! You're doing great.".to_string(),
    };

    Ok(GitClippyReport {
        status,
        urgency_level: urgency_level.to_string(),
        message,
        suggestions,
        duplicates,
        commit_suggestions,
    })
}

/// Execute a git action
pub fn execute_git_action(
    repo_path: &str,
    action: &str,
    data: Option<&serde_json::Value>,
) -> Result<String, String> {
    match action {
        "wip_commit" => {
            run_git_command(repo_path, &["add", "-A"])?;
            run_git_command(repo_path, &["commit", "-m", "WIP: Work in progress save"])
        }
        "create_branch" => {
            let branch_name = data
                .and_then(|d| d.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("feature-branch");
            run_git_command(repo_path, &["checkout", "-b", branch_name])
        }
        "stage_all" => run_git_command(repo_path, &["add", "-A"]),
        "commit" => {
            let message = data
                .and_then(|d| d.get("message"))
                .and_then(|m| m.as_str())
                .unwrap_or("Update files");
            run_git_command(repo_path, &["commit", "-m", message])
        }
        "git_init" => run_git_command(repo_path, &["init"]),
        _ => Err(format!("Unknown action: {}", action)),
    }
}
