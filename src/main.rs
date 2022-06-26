use std::{env, process};

use clap::Parser;
use leetcode_tool::{fetch, submit, template};

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
    async_std::task::block_on(async {
        let args = Args::parse();

        match args.action {
            Action::Fetch { title } => {
                let question = fetch::fetch_question(title).await.unwrap();
                let project_dir = env::current_dir().unwrap();
                template::w::write_template(&question, project_dir)
                    .await
                    .unwrap();
            }
            Action::Submit { title } => submit::submit_code(&title).await.unwrap(),
            Action::Login => todo!(),
        }
    });
    Ok(())
}
