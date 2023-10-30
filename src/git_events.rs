extern crate git2;

use git2::{DiffDelta, DiffOptions, Repository};

pub struct GitEvents {
    pub repo: String,
    pub sha: String,
    filter: Vec<String>
}

impl GitEvents {
    pub fn new(repo: String, sha: String) -> Self {
        Self {
            repo,
            sha,
            filter: vec![String::from("md")]
        }
    }

    pub fn collect_events(&self) {
        let repo_path = &self.repo; // Update this with the path to your repo
        let sha = &self.sha; // Update this with the SHA you want to compare to

        let repo = Repository::open(repo_path).expect("Failed to open repository");
        let commit = repo.find_commit(sha.parse().unwrap()).expect("Failed to find commit");
        let tree = commit.tree().expect("Failed to get commit tree");

        let mut diff_options = DiffOptions::new();

        let diff = repo
            .diff_tree_to_index(Some(&tree), None, Some(&mut diff_options))
            .expect("Failed to get diff");

        let ext = self.filter.first().unwrap().as_str();
        diff.foreach(
            &mut |delta: DiffDelta, _| {
                match delta.status() {
                    git2::Delta::Added => {
                        let path = delta.new_file().path().unwrap();
                        if let Some(extension) = path.extension() {
                            if extension == ext {
                                println!("Added: {}", delta.new_file().path().unwrap().display());
                            }
                        }

                        // println!("Added: {}", delta.new_file().path().unwrap().display());
                    }
                    git2::Delta::Deleted => {
                        println!("Deleted: {}", delta.old_file().path().unwrap().display());
                    }
                    git2::Delta::Modified => {
                        println!("Modified: {}", delta.new_file().path().unwrap().display());
                    }
                    git2::Delta::Renamed => {
                        println!(
                            "Renamed: {} => {}",
                            delta.old_file().path().unwrap().display(),
                            delta.new_file().path().unwrap().display()
                        );
                    }
                    git2::Delta::Copied => {
                        println!(
                            "Copied: {} => {}",
                            delta.old_file().path().unwrap().display(),
                            delta.new_file().path().unwrap().display()
                        );
                    }
                    _ => {}
                }
                true
            },
            None,
            None,
            None,
        )
            .expect("Failed to process diff");
    }
}