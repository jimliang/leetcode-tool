use std::env;

use anyhow::{bail, Result};
use clap::Parser;
use leetcode_tool::{
    fetch,
    leetcode::{question_of_today, random_question},
    submit, template,
    util::get_title_slug,
};

#[derive(Debug, clap::Subcommand)]
enum Action {
    Fetch {
        title: Option<String>,
        #[clap(short, long, action)]
        random: bool,
    },
    Submit {
        title: String,
    },
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
        Action::Fetch { title, random } => {
            let title = match title {
                Some(t) => get_title_slug(&t).into_owned(),
                None => {
                    if random {
                        random_question().await?
                    } else {
                        let question = question_of_today().await?;
                        let title_slug = question
                            .get(0)
                            .and_then(|q| q.question.as_ref())
                            .map(|q| &q.titleSlug)
                            .expect("can not find today's question");
                        title_slug.to_owned()
                    }
                }
            };
            let title = get_title_slug(&title);
            println!("start to fetch project {}", title);
            let question = fetch::fetch_question(&title).await?;
            let project_dir = env::current_dir()?;
            let file_path = template::w::write_template(&question, project_dir).await?;
            println!("> {}", file_path.display());
        }
        Action::Submit { title } => {
            let cookie = match std::env::var("COOKIE") {
                Ok(c) => c,
                Err(_) => bail!(
                    "neet to set cookie for login leetcode by `export COOKIE=<LEETCODE-COOKIE>`"
                ),
            };
            let title = get_title_slug(&title);
            submit::submit_code(&title, &cookie).await?
        }
    }
    Ok(())
}

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    async_std::task::block_on(async {
        if let Err(error) = main_inner().await {
            log::error!("{:?}", error);
        }
    });
}
