use clap::{Parser, Subcommand};
use design_everything::data::sqlite::IndexDB;

#[derive(Parser)]
#[command(name = "index_manager")]
#[command(about = "Manage module index in SQLite database", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    RegisterModule {
        name: String,
        slug: String,
        description: String,
        category: String,
        keywords: String,
        tsv: String,
    },
    RegisterCategory {
        name: String,
        slug: String,
        description: String,
    },
    List,
    ListCategories,
    ListByCategory {
        category: String,
    },
    Search {
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
            println!("Database initialized: {:?}", db_path);
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
                println!("[Category] {} ({}) - {} items", m.name, m.slug, m.item_count);
            }
        }
        Commands::ListCategories => {
            let categories = db.get_all_categories()?;
            for c in categories {
                println!("{} ({})", c.name, c.slug);
            }
        }
        Commands::ListByCategory { category } => {
            let modules = db.get_all_modules()?;
            for m in modules {
                if let Some(cat_id) = m.category_id {
                    let categories = db.get_all_categories()?;
                    if let Some(cat) = categories.iter().find(|c| c.id == cat_id && c.slug == category) {
                        println!("  {} ({})", m.name, m.slug);
                    }
                }
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
