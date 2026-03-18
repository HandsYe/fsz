use std::{
    path::PathBuf,
    time::Instant,
    collections::HashMap,
};

use clap::Parser;
use walkdir::WalkDir;

/// Fast file size analyzer - Analyze disk usage by directory
#[derive(Parser, Debug)]
#[command(name = "fsz")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Minimum file size (e.g., 1K, 1M, 1G)
    #[arg(short = 's', long, default_value = "0")]
    min_size: String,

    /// Directory depth for grouping (1 = show immediate subdirs)
    #[arg(short = 'd', long, default_value = "0")]
    depth: usize,

    /// Number of top items to show
    #[arg(short = 'n', long, default_value = "10")]
    top: usize,
}

fn parse_size(size_str: &str) -> u64 {
    let size_str = size_str.trim().to_uppercase();
    if size_str == "0" || size_str.is_empty() {
        return 0;
    }

    let multiplier = if size_str.ends_with('K') {
        1024u64
    } else if size_str.ends_with('M') {
        1024u64 * 1024
    } else if size_str.ends_with('G') {
        1024u64 * 1024 * 1024
    } else if size_str.ends_with('T') {
        1024u64 * 1024 * 1024 * 1024
    } else {
        1u64
    };

    let num_str = size_str.trim_end_matches(|c: char| c.is_alphabetic());
    num_str.parse::<u64>().unwrap_or(0) * multiplier
}

fn format_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut sz = size as f64;
    let mut idx = 0;
    while sz >= 1024.0 && idx < UNITS.len() - 1 {
        sz /= 1024.0;
        idx += 1;
    }
    format!("{:.1} {}", sz, UNITS[idx])
}

#[derive(Debug)]
enum Item {
    Dir { path: PathBuf, size: u64, file_count: usize },
    File { path: PathBuf, size: u64 },
}

impl Item {
    fn size(&self) -> u64 {
        match self {
            Item::Dir { size, .. } => *size,
            Item::File { size, .. } => *size,
        }
    }

    fn path(&self) -> &PathBuf {
        match self {
            Item::Dir { path, .. } => path,
            Item::File { path, .. } => path,
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self, Item::Dir { .. })
    }

    fn file_count(&self) -> Option<usize> {
        match self {
            Item::Dir { file_count, .. } => Some(*file_count),
            Item::File { .. } => None,
        }
    }
}

fn main() {
    let args = Args::parse();
    let path = args.path.canonicalize().unwrap_or_else(|e| {
        eprintln!("Error: Cannot access path '{}': {}", args.path.display(), e);
        std::process::exit(1);
    });

    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory", path.display());
        std::process::exit(1);
    }

    let min_size = parse_size(&args.min_size);
    let start = Instant::now();

    // Scan all files
    let mut files: Vec<(PathBuf, u64)> = Vec::new();
    let mut dir_count = 0u64;

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            dir_count += 1;
        } else if let Ok(meta) = entry.metadata() {
            let size = meta.len();
            if size >= min_size {
                files.push((entry.path().to_path_buf(), size));
            }
        }
    }

    let scan_time = start.elapsed();
    let total_size: u64 = files.iter().map(|(_, s)| s).sum();
    let file_count = files.len();

    // Build items list
    let mut items: Vec<Item> = Vec::new();

    if args.depth >= 1 {
        // Group by directory at specified depth
        let mut dir_sizes: HashMap<PathBuf, (u64, usize)> = HashMap::new();
        
        for (file_path, size) in &files {
            if let Ok(relative) = file_path.strip_prefix(&path) {
                let mut current = PathBuf::new();
                for (i, component) in relative.components().enumerate() {
                    if i >= args.depth { break; }
                    current.push(component);
                }
                if !current.as_os_str().is_empty() {
                    let entry = dir_sizes.entry(path.join(&current)).or_insert((0, 0));
                    entry.0 += size;
                    entry.1 += 1;
                }
            }
        }

        for (dir_path, (size, count)) in dir_sizes {
            items.push(Item::Dir { path: dir_path, size, file_count: count });
        }
    } else {
        // Show individual files
        for (file_path, size) in &files {
            items.push(Item::File { path: file_path.clone(), size: *size });
        }
    }

    // Sort by size descending
    items.sort_by(|a, b| b.size().cmp(&a.size()));

    // Output
    println!("\n{}", "═".repeat(70));
    println!("  📊 FSZ - File Size Analyzer");
    println!("{}", "─".repeat(70));
    println!("  Path:       {}", path.display());
    if min_size > 0 {
        println!("  Min size:   {}", format_size(min_size));
    }
    if args.depth > 0 {
        println!("  Depth:      {}", args.depth);
    }
    println!("  Scan time:  {:.2?}", scan_time);
    println!("  Files:      {}", file_count);
    println!("  Dirs:       {}", dir_count);
    println!("  Total:      {}", format_size(total_size));
    println!("{}", "═".repeat(70));

    let title = if args.depth >= 1 {
        format!("Top {} Directories (depth {})", args.top.min(items.len()), args.depth)
    } else {
        format!("Top {} Largest Files", args.top.min(items.len()))
    };

    println!("\n  {}", title);
    println!("  {}", "─".repeat(title.len() + 2));

    for (i, item) in items.iter().take(args.top).enumerate() {
        let marker = if item.is_dir() {
            format!("[DIR {}]", item.file_count().unwrap_or(0))
        } else {
            "[FILE]".to_string()
        };
        println!("  {:3}. {:>10}  {:8}  {}", 
            i + 1, 
            format_size(item.size()), 
            marker,
            item.path().display()
        );
    }

    println!("\n{}", "═".repeat(70));
}
