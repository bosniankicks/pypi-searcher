use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

// Global HTTP client with connection pooling and optimizations
static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(std::time::Duration::from_secs(90))
        .tcp_nodelay(true) // Disable Nagle's algorithm for lower latency
        .tcp_keepalive(std::time::Duration::from_secs(60))
        .gzip(true)
        .brotli(true)
        .build()
        .expect("Failed to build HTTP client")
});

#[derive(Parser, Debug)]
#[command(author, version, about = "Blazingly fast PyPI package search", long_about = None)]
struct Args {
    /// Package name to search
    package: String,

    /// Show full description
    #[arg(short, long)]
    full: bool,

    /// Output as JSON
    #[arg(short, long)]
    json: bool,

    /// Benchmark mode (show timing)
    #[arg(short, long)]
    benchmark: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct PyPIResponse {
    info: PackageInfo,
    releases: HashMap<String, Vec<ReleaseInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_serial: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PackageInfo {
    name: String,
    version: String,
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    author_email: Option<String>,
    #[serde(default)]
    license: Option<String>,
    #[serde(default)]
    home_page: Option<String>,
    #[serde(default)]
    project_urls: Option<HashMap<String, String>>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    requires_python: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReleaseInfo {
    filename: String,
    #[serde(default)]
    upload_time: String,
}

#[derive(Debug, Serialize)]
struct SearchResult {
    success: bool,
    name: String,
    version: String,
    summary: String,
    author: String,
    license: String,
    homepage: String,
    description: String,
    requires_python: String,
    total_releases: usize,
    recent_releases: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fetch_time_ms: Option<u128>,
}

async fn fetch_package_info(package_name: &str) -> Result<PyPIResponse> {
    let url = format!("https://pypi.org/pypi/{}/json", package_name);

    let response = HTTP_CLIENT
        .get(&url)
        .header("Accept", "application/json")
        .header("Accept-Encoding", "gzip, br") // Enable compression
        .send()
        .await
        .context("Failed to fetch package data")?;

    if response.status() == 404 {
        anyhow::bail!("Package '{}' not found", package_name);
    }

    response
        .json::<PyPIResponse>()
        .await
        .context("Failed to parse JSON response")
}

fn format_package_info(info: &SearchResult, full: bool) -> String {
    let mut output = String::new();

    output.push_str(&format!("{}\n", "=".repeat(60).bright_cyan()));
    output.push_str(&format!("{}: {}\n", "Package".bold(), info.name.bright_green()));
    output.push_str(&format!("{}\n", "=".repeat(60).bright_cyan()));
    output.push('\n');

    output.push_str(&format!(
        "{}: {}\n",
        "Latest Version".bold(),
        info.version.bright_yellow()
    ));
    output.push_str(&format!("{}: {}\n", "Summary".bold(), info.summary));
    output.push_str(&format!("{}: {}\n", "Author".bold(), info.author));
    output.push_str(&format!("{}: {}\n", "License".bold(), info.license));
    output.push_str(&format!("{}: {}\n", "Homepage".bold(), info.homepage));

    if !info.requires_python.is_empty() {
        output.push_str(&format!(
            "{}: {}\n",
            "Python Version".bold(),
            info.requires_python
        ));
    }

    output.push('\n');

    if full && !info.description.is_empty() {
        output.push_str(&format!("{}:\n", "Description".bold()));
        output.push_str(&format!(
            "{}\n",
            info.description
                .lines()
                .take(20)
                .collect::<Vec<_>>()
                .join("\n")
        ));
        output.push('\n');
    }

    output.push_str(&format!(
        "{}: {}\n",
        "Total Releases".bold(),
        info.total_releases.to_string().bright_magenta()
    ));

    if !info.recent_releases.is_empty() {
        output.push_str(&format!("{}: ", "Recent Versions".bold()));
        output.push_str(&info.recent_releases.join(", "));
        output.push('\n');
    }

    output.push('\n');
    output.push_str(&format!("{}\n", "=".repeat(60).bright_cyan()));

    if let Some(time) = info.fetch_time_ms {
        output.push_str(&format!(
            "{} Fetched in {}ms\n",
            "⚡".bright_yellow(),
            time.to_string().bright_green()
        ));
    }

    output
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let start = Instant::now();

    // Fetch package info
    let response = fetch_package_info(&args.package).await?;

    let fetch_time = start.elapsed().as_millis();

    // Get recent releases
    let mut release_versions: Vec<String> = response.releases.keys().cloned().collect();
    release_versions.sort();
    let recent_releases: Vec<String> = release_versions
        .iter()
        .rev()
        .take(5)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    // Get author
    let author = response.info.author
        .as_ref()
        .filter(|s| !s.is_empty())
        .cloned()
        .or_else(|| response.info.author_email.as_ref().filter(|s| !s.is_empty()).cloned())
        .unwrap_or_else(|| "Unknown".to_string());

    // Get homepage
    let homepage = response.info.home_page
        .as_ref()
        .filter(|s| !s.is_empty())
        .cloned()
        .or_else(|| {
            response.info.project_urls.as_ref().and_then(|urls| {
                urls.get("Homepage")
                    .or_else(|| urls.values().next())
                    .cloned()
            })
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let description = if args.full {
        response.info.description.clone().unwrap_or_default()
    } else {
        response.info.description
            .as_ref()
            .map(|desc| {
                let chars: String = desc.chars().take(200).collect();
                if desc.len() > 200 {
                    format!("{}...", chars)
                } else {
                    chars
                }
            })
            .unwrap_or_default()
    };

    let result = SearchResult {
        success: true,
        name: response.info.name,
        version: response.info.version,
        summary: response.info.summary.unwrap_or_else(|| "No summary available".to_string()),
        author,
        license: response.info.license
            .as_ref()
            .filter(|s| !s.is_empty())
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string()),
        homepage,
        description,
        requires_python: response.info.requires_python.unwrap_or_default(),
        total_releases: response.releases.len(),
        recent_releases,
        fetch_time_ms: if args.benchmark {
            Some(fetch_time)
        } else {
            None
        },
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", format_package_info(&result, args.full));
        println!("{}", "✓ SUCCESS - No challenge required!".bright_green());
    }

    Ok(())
}
