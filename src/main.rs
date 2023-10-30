use clap::Parser;

mod git_events;

use git_events::GitEvents;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    repo: String,

    #[arg(short, long)]
    sha: String,
}

fn main() {
    let args = Args::parse();
    let instance = GitEvents {
        repo: args.repo.clone(),
        sha: args.sha.clone(),
    };
    instance.collect_events();
}

