include!("src/cli.rs");

use clap_mangen::Man;
use clap::CommandFactory;

fn main() {
    let man_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("man");

    std::fs::create_dir_all(&man_dir).unwrap();

    let mut cmd = Cli::command();
    cmd.set_bin_name("hyprland-autodim");

    let man = Man::new(cmd.to_owned());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer).expect("Man page generation failed");
    std::fs::write(man_dir.join("hyprland-autodim.1"), buffer).expect("Failed to write man page");
}
