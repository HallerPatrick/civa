use std::process::Output;
use std::process::{Command, Stdio};

pub struct GitCli {}

impl GitCli {
    pub fn get_current_branch() -> String {
        let output: Output = Command::new("git")
            .args(&["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .unwrap();

        String::from_utf8(output.stdout).expect("Found invalid UTF-8")
    }

    pub fn no_upstream_commits<'a>() -> String {
        let git_call = Command::new("git")
            .args(&["cherry"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let wc = Command::new("wc")
            .args(&["-l"])
            .stdin(git_call.stdout.unwrap())
            .output()
            .unwrap();

        String::from_utf8(wc.stdout)
            .expect("Found invalid UTF-8")
            .trim()
            .to_string()
    }

    pub fn compose_git_component() -> String {
        let mut comps: Vec<String> = vec![];

        comps.push(GitCli::get_current_branch().trim_end().to_string());

        let us = GitCli::no_upstream_commits();
        if us != String::from("0") {
            comps.push(format!(" ⇡{}", us));
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

        assert_eq!(res, String::from("1"));
    }
}
