use std::process::Command;
use std::process::Output;

pub struct GitCli {}

impl GitCli {
    pub fn get_current_branch() -> String {
        let output: Output = Command::new("git")
            .args(&["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .unwrap();

        String::from_utf8(output.stdout).expect("Found invalid UTF-8")
    }

    pub fn no_upstream_commits() -> usize {
        let output: Output = Command::new("git")
            .args(&["git", "cherry"])
            .output()
            .unwrap();

        let output = String::from_utf8(output.stdout).expect("Found invalid UTF-8");
        println!("{:?}", output);

        if output == "" {
            0
        } else {
            output.split("\n").count()
        }
    }

    pub fn compose_git_component() -> String {
        let mut comps: Vec<String> = vec![];

        comps.push(GitCli::get_current_branch().trim_end().to_string());

        match GitCli::no_upstream_commits() {
            0 => {}
            num => comps.push(format!(" ⇡{}", num)),
        }

        comps.join("")
    }

    // fn number_untracked_files() -> String {}

    // fn number_modified_files() -> String {}
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = GitCli::get_current_branch();
        assert_eq!(result, "master\n");
    }

    #[test]
    fn it_works1() {
        let res = GitCli::no_upstream_commits();

        assert_eq!(res, 0);
    }
}
