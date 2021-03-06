mod ue4;

use clap::{App, AppSettings, Arg};
use ue4::{UE4PathFinder, UE4Project};

fn all_args() -> Vec<Arg<'static, 'static>> {
    vec![Arg::with_name("getrootdir")
        .long("getrootdir")
        .help("Gets the UE rootdir in plain text. Based on an existing .uproject file in the current working directory.")]
}

fn main() {
    let args = all_args();
    let matches = App::new("ue4-cli-rs")
        .version("0.1")
        .about("UE4 build CLI helper application")
        .args(&args)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
    if matches.args.is_empty() {
        println!("{}", matches.usage());
    }

    let curr_dir = std::env::current_dir().unwrap();
    let project = UE4Project::guess_from_dir(&curr_dir).unwrap();
    let path = UE4PathFinder::new(project);

    if matches.is_present("getrootdir") {
        println!("{}", path.unwrap().engine_root.display());
    }
}
