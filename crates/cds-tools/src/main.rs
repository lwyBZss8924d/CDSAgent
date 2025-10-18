//! CDS CLI - Code search and navigation tool

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cds")]
#[command(about = "Code Dependency Search - Navigate codebases with graph-based search")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for entities by name or keyword
    Search {
        query: String,
        #[arg(short, long)]
        entity_type: Option<String>,
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Traverse dependency graph
    Traverse {
        entity_id: String,
        #[arg(short, long, default_value = "outgoing")]
        direction: String,
        #[arg(short, long)]
        edge_type: Option<String>,
    },
    /// Retrieve code content
    Retrieve {
        entity_id: String,
        #[arg(short, long, default_value = "5")]
        context: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            query,
            entity_type,
            limit,
        } => {
            println!(
                "TODO: Search for '{}' (type: {:?}, limit: {})",
                query, entity_type, limit
            );
        }
        Commands::Traverse {
            entity_id,
            direction,
            edge_type,
        } => {
            println!(
                "TODO: Traverse '{}' (direction: {}, edge_type: {:?})",
                entity_id, direction, edge_type
            );
        }
        Commands::Retrieve { entity_id, context } => {
            println!("TODO: Retrieve '{}' (context: {})", entity_id, context);
        }
    }

    Ok(())
}
