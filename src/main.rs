use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::env;

fn main() {
    println!("---- Conventional Commits CLI ----");

    let current_dir = env::current_dir().expect("Error getting current directory");
    println!("Current directory: {:?}", current_dir);

    let repo = git2::Repository::open(current_dir).expect("Error opening repository");

    let commit_types = get_commit_types();
    let selection_commit_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select commit type")
        .items(&commit_types)
        .default(0)
        .interact_opt()
        .unwrap();

    match selection_commit_type {
        Some(commit_type) => {
            println!("Commit type: {}", commit_types[commit_type]);
        }
        None => panic!("No commit type selected"),
    }

    let gitmojis = get_gitmojis();
    let selection_gitmoji = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select gitmoji")
        .items(&gitmojis)
        .default(0)
        .interact_opt()
        .unwrap();

    match selection_gitmoji {
        Some(gitmoji) => {
            println!("Gitmoji: {}", gitmojis[gitmoji]);
        }
        None => panic!("No gitmoji selected"),
    }

    let commit_message = dialoguer::Input::<String>::new()
        .with_prompt("Enter commit message")
        .interact()
        .unwrap();

    println!("Commit message: {}", commit_message);

    let commit = format!(
        "{}: {} {}",
        commit_types[selection_commit_type.unwrap()].commit_type,
        gitmojis[selection_gitmoji.unwrap()].emoji,
        commit_message
    );

    println!("{}", commit);

    repo.commit(
        Some("HEAD"),
        &repo.signature().unwrap(),
        &repo.signature().unwrap(),
        commit.as_str(),
        &repo
            .find_tree(repo.index().unwrap().write_tree().unwrap())
            .unwrap(),
        &vec![&repo.head().unwrap().peel_to_commit().unwrap()],
        
    )
    .expect("Error committing");
}

// Get gitmojis
#[derive(Serialize, Deserialize, Debug)]
struct Gitmoji {
    emoji: String,
    entity: String,
    code: String,
    description: String,
    name: String,
    semver: Option<String>,
}
impl std::fmt::Display for Gitmoji {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.emoji, self.description)
    }
}
fn get_gitmojis() -> Vec<Gitmoji> {
    let gitmojis_json = include_str!("gitmojis.json");
    let gitmojis: Vec<Gitmoji> = from_str(gitmojis_json).expect("Error reading gitmojis.json");
    gitmojis
}

// Get commit types
#[derive(Serialize, Deserialize, Debug)]
struct CommitType {
    commit_type: String,
    description: String,
    title: String,
}
impl std::fmt::Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.commit_type, self.description)
    }
}
fn get_commit_types() -> Vec<CommitType> {
    let commit_types_json = include_str!("commit_types.json");
    let commit_types: Vec<CommitType> =
        from_str(commit_types_json).expect("Error reading commit-types.json");
    commit_types
}
