use clap::{Parser, Subcommand};
use design_everything::data::TSVReader;

#[derive(Parser)]
#[command(name = "tsv_reader")]
#[command(about = "Read and search TSV files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long)]
    tsv_dir: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Read {
        #[arg(short, long)]
        module: String,
    },
    Search {
        #[arg(short, long)]
        query: String,
        #[arg(short, long)]
        module: Option<String>,
    },
    List,
    Count {
        #[arg(short, long)]
        module: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    let tsv_dir = match cli.tsv_dir {
        Some(path) => std::path::PathBuf::from(path),
        None => TSVReader::default_path()?,
    };
    
    eprintln!("TSV directory: {:?}", tsv_dir);
    
    let reader = TSVReader::new(&tsv_dir)?;

    match cli.command {
        Commands::Read { module } => {
            let filename = format!("{}.tsv", module);
            let items = reader.read_tsv(&filename)?;

            println!("\n=== {} === ({} items)\n", module.to_uppercase(), items.len());
            for item in items {
                println!("[{}]", item.title);
                let content = if item.content.len() > 200 {
                    format!("{}...", &item.content[..200])
                } else {
                    item.content.clone()
                };
                println!("  {}", content);
                println!("  Tags: {}\n", item.tags);
            }
        }
        Commands::Search { query, module } => {
            match module {
                Some(m) => {
                    let filename = format!("{}.tsv", m);
                    let items = reader.search_in_tsv(&filename, &query)?;

                    println!("\n=== {} - '{}' ({} matches) ===\n", m.to_uppercase(), query, items.len());
                    for item in items {
                        println!("[{}]", item.title);
                        println!("  {}", item.content);
                        println!("  Tags: {}\n", item.tags);
                    }
                }
                None => {
                    let results = reader.search_all(&query)?;

                    println!("\nSearching: '{}'\n", query);
                    for (module, items) in results {
                        println!("=== {} ({} matches) ===", module.to_uppercase(), items.len());
                        for item in items.iter().take(5) {
                            println!("  • {}", item.title);
                        }
                        if items.len() > 5 {
                            println!("  ... and {} more", items.len() - 5);
                        }
                        println!();
                    }
                }
            }
        }
        Commands::List => {
            let modules = reader.get_module_list()?;

            println!("\nAvailable modules ({}):\n", modules.len());
            for m in modules {
                let count = reader.count_items(&format!("{}.tsv", m)).unwrap_or(0);
                println!("  {}: {} items", m, count);
            }
        }
        Commands::Count { module } => {
            match module {
                Some(m) => {
                    let count = reader.count_items(&format!("{}.tsv", m))?;
                    println!("{}: {} items", m, count);
                }
                None => {
                    let counts = reader.get_all_counts()?;

                    println!("\nModule counts:\n");
                    let mut total = 0;
                    for (module, count) in counts {
                        println!("  {}: {}", module, count);
                        total += count;
                    }
                    println!("\nTotal: {} items", total);
                }
            }
        }
    }

    Ok(())
}
