use clap::{Parser, Subcommand};
use design_everything::data::sqlite::{IndexDB, Module};

#[derive(Parser)]
#[command(name = "db_manager")]
#[command(about = "Manage SQLite database for design modules", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    RegisterModule {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        slug: String,
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        category: String,
        #[arg(short, long)]
        keywords: String,
        #[arg(short, long)]
        tsv: String,
    },
    RegisterCategory {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        slug: String,
        #[arg(short, long)]
        description: String,
    },
    List,
    ListCategories,
    Search {
        #[arg(short, long)]
        query: String,
    },
    Stats,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db_path = IndexDB::default_path()?;
    let db = IndexDB::new(&db_path)?;

    match cli.command {
        Commands::Init => {
            println!("Database initialized.");
            println!("Location: {:?}", db_path);
        }
        Commands::RegisterModule {
            name,
            slug,
            description,
            category,
            keywords,
            tsv,
        } => {
            db.register_module(&name, &slug, &description, &category, &keywords, &tsv)?;
            println!("Module '{}' registered.", name);
        }
        Commands::RegisterCategory {
            name,
            slug,
            description,
        } => {
            db.register_category(&name, &slug, &description)?;
            println!("Category '{}' registered.", name);
        }
        Commands::List => {
            let modules = db.get_all_modules()?;
            for m in modules {
                println!("[{}] {} ({}) - {} items", 
                    m.category_id.map_or("N/A".to_string(), |_| "Category".to_string()),
                    m.name, 
                    m.slug, 
                    m.item_count
                );
            }
        }
        Commands::ListCategories => {
            let categories = db.get_all_categories()?;
            for c in categories {
                println!("{} ({})", c.name, c.slug);
            }
        }
        Commands::Search { query } => {
            let modules = db.search_modules(&query)?;
            for m in modules {
                println!("[Category] {}: {}", m.name, m.description);
            }
        }
        Commands::Stats => {
            let modules = db.get_all_modules()?;
            let categories = db.get_all_categories()?;

            println!("Categories: {}", categories.len());
            println!("Modules: {}", modules.len());

            let total_items: i32 = modules.iter().map(|m| m.item_count).sum();
            println!("Total items: {}", total_items);
        }
    }

    Ok(())
}
