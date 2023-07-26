include!("src/cli.rs");

use clap_mangen::Man;
use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Fish, Zsh};

fn generate_man_pages() {
    let man_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("man");

    std::fs::create_dir_all(&man_dir).unwrap();

    let mut cmd = Cli::command();
    cmd.set_bin_name("hyprland-autodim");

    let man = Man::new(cmd.to_owned());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer).expect("Man page generation failed");
    std::fs::write(man_dir.join("hyprland-autodim.1"), buffer).expect("Failed to write man page");
}

fn generate_shell_completions() {
    let comp_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("completions");

    std::fs::create_dir_all(&comp_dir).unwrap();

    let mut cmd = Cli::command();
    cmd.set_bin_name("hyprland-autodim");

    for shell in [Bash, Fish, Zsh] {
        generate_to(shell, &mut cmd, "hyprland-autodim", &comp_dir).unwrap();
    }
}

fn main() {
    generate_man_pages();
    generate_shell_completions();
}
