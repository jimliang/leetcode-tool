use std::{env, process};

use clap::Parser;
use leetcode_tool::{fetch, template};

#[derive(Debug, clap::Subcommand)]
enum Action {
    Fetch { title: String },
    Submit { title: String },
    Login,
}

#[derive(Debug, Parser)]
#[clap(author, version, about = "leetcode tool for Rust", long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    match args.action {
        Action::Fetch { title } => async_std::task::block_on(async {
            let question = fetch::fetch_question(title).await.unwrap();
            let project_dir = env::current_dir().unwrap();
            template::w::write_template(&question, project_dir)
                .await
                .unwrap();
        }),
        Action::Submit { title } => todo!(),
        Action::Login => todo!(),
    }
    Ok(())
}
