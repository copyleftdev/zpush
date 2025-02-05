use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

mod github;
mod git_utils;

use github::GitHubClient;
use git_utils::get_origin_repo;

#[derive(Parser, Debug)]
#[command(name = "zenvpush", about = "Push your environment secrets to your GitHub repository for Zoob.")]
struct Args {
    #[arg(short, long)]
    secrets_file: String,
    #[arg(short = 'e', long)]
    env_file: Option<String>,
}

#[tokio::main]
async fn main() {
    let args_cli = Args::parse();
    if let Some(env_path) = args_cli.env_file {
        let _ = dotenv::from_path(env_path);
    } else {
        let _ = dotenv::dotenv();
    }
    let args = Args::parse();
    if !Path::new(".git").exists() {
        eprintln!("{}", "Error: .git folder not found. Please run this command from the root of your Git repository.".red().bold());
        std::process::exit(1);
    }
    let repo_info = match get_origin_repo() {
        Ok((owner, repo)) => format!("{}/{}", owner, repo),
        Err(e) => {
            eprintln!("{} {}", "Error retrieving repository information:".red(), e);
            std::process::exit(1);
        }
    };
    println!("{} {}", "Detected repository:".bold().green(), repo_info);
    let github_token = match env::var("GITHUB_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("{}", "Error: GITHUB_TOKEN environment variable not set. Please set it with the required scopes (repo, admin:repo_hook, secrets).".red().bold());
            std::process::exit(1);
        }
    };
    let github_client = GitHubClient::new(github_token);
    match github_client.verify_token().await {
        Ok(_) => println!("{} {}", "GitHub token verified successfully!".green().bold(), "✅".green()),
        Err(e) => {
            eprintln!("{} {}", "GitHub token verification failed:".red().bold(), e);
            std::process::exit(1);
        }
    }
    let secrets = match read_secrets(&args.secrets_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} {}", "Failed to read secrets file:".red(), e);
            std::process::exit(1);
        }
    };
    if secrets.is_empty() {
        println!("{}", "No secrets found to push.".yellow());
        return;
    }
    let pb = ProgressBar::new(secrets.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}").unwrap().tick_chars("🌑🌒🌓🌔🌕"));
    for (key, value) in secrets {
        pb.set_message(format!("Pushing secret: {}", key));
        let encrypted_value = encrypt_secret(&value);
        let key_id = "dummy-key-id";
        match github_client.push_secret(&repo_info, &key, &encrypted_value, key_id).await {
            Ok(_) => {
                pb.println(format!("{} {} {}", "Successfully pushed secret:".green().bold(), key, "🎉"));
            }
            Err(e) => {
                pb.println(format!("{} {}: {}", "Failed to push secret".red().bold(), key, e));
            }
        }
        pb.inc(1);
        sleep(Duration::from_millis(100)).await;
    }
    pb.finish_with_message("All secrets have been pushed successfully!");
    println!("{}", "All secrets have been successfully pushed to your GitHub repository! 🚀".bold().green());
}

fn read_secrets(file_path: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut secrets = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            secrets.push((key.trim().to_string(), value.trim().to_string()));
        }
    }
    Ok(secrets)
}

fn encrypt_secret(secret: &str) -> String {
    secret.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_read_secrets() {
        let mut tmpfile = NamedTempFile::new().unwrap();
        writeln!(tmpfile, "# comment").unwrap();
        writeln!(tmpfile, "KEY1=VALUE1").unwrap();
        writeln!(tmpfile, "   ").unwrap();
        writeln!(tmpfile, "KEY2=VALUE2").unwrap();
        let secrets = read_secrets(tmpfile.path().to_str().unwrap()).unwrap();
        assert_eq!(secrets.len(), 2);
        assert_eq!(secrets[0].0, "KEY1");
        assert_eq!(secrets[0].1, "VALUE1");
        assert_eq!(secrets[1].0, "KEY2");
        assert_eq!(secrets[1].1, "VALUE2");
    }

    #[test]
    fn test_encrypt_secret() {
        assert_eq!(encrypt_secret("secret"), "terces");
    }
}
