use std::env;

use anyhow::Result;
use clap::Parser;
use leetcode_tool::{fetch, submit, template, util::get_title_slug};

#[derive(Debug, clap::Subcommand)]
enum Action {
    Fetch { title: String },
    Submit { title: String },
    // Login,
}

#[derive(Debug, Parser)]
#[clap(author, version, about = "leetcode tool for Rust", long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

async fn main_inner() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Fetch { title } => {
            let title = get_title_slug(&title);
            let question = fetch::fetch_question(title.clone().into_owned()).await?;
            let project_dir = env::current_dir()?;
            let file_path = template::w::write_template(&question, project_dir).await?;
            log::info!("Fetched project {}", title);
            log::info!("{}", file_path.display());
        }
        Action::Submit { title } => {
            let title = get_title_slug(&title);
            submit::submit_code(&title).await?
        } // Action::Login => todo!(),
    }
    Ok(())
}

fn main() {
    pretty_env_logger::init();
    async_std::task::block_on(async {
        if let Err(error) = main_inner().await {
            eprintln!("{:?}", error);
        }
    });
}
