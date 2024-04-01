use std::env;
use std::fs;
use inline_colorization::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("-h") => help(),
        Some("--help") => help(),
        Some("-n") => new(args),
        Some("--new") => new(args),
        _ => println!("Unknown or missingcommand, {color_red}-h{color_reset} for help"),
    }
}

fn new_dir(path: &str) {
    fs::create_dir(path).expect("Failed to create directory");
}

fn help() {
    println!("{style_bold}{color_green}mcShader {color_red} HELP COMMAND: {color_reset}{style_reset} -h, --help");
    println!("");
    println!("{color_yellow}Summary{color_reset}");
    println!("This is a set of commands meant to allow the user to auto generate a set of folders for their new shader");
    println!("in a few diffrent varients based on prefrence and use case.");
    println!("");
    println!("{color_yellow}Commands{color_reset}");
    println!("{color_cyan}Help{color_reset}");
    println!("{style_bold}-h OR --help{style_reset}");
    println!("Loads this help commands");
    println!("");
    println!("{color_cyan}New{color_reset}");
    println!("{style_bold}-n OR -new{style_reset}");
    println!("Creates a new project folder set, requires a subcommand of the following and a project name following the subcommand.");
    println!("{style_bold}Subcommands{style_reset}");
    println!("{style_bold}-i{style_reset} OR {style_bold}--include{style_reset} for an include based shader");
    println!("{style_bold}-l{style_reset} OR {style_bold}--inlude-less{style_reset} for an include less shader");
    println!("{style_bold}-m{style_reset} OR {style_bold}--minimal{style_reset} for a basic structure for demo shaders");

}

fn new(args: Vec<String>) {
    match args.get(2).map(|s| s.as_str()) {
        Some("-i") => w_partition(args),
        Some("--include") => w_partition(args),
        Some("-l") => u_partition(args),
        Some("--inlude-less") => u_partition(args),
        Some("-m") => min(args),
        Some("--minimal") => min(args),
        _ => println!("Unknown or missing command, you need one of the following: -p, --include, -u, --include-less, -m, --minimal, or {color_red}-h{color_reset} for help"),
    }
}

fn w_partition(args: Vec<String>) {
    let name: &str = args.get(3).map(|s| s.as_str()).unwrap();
    println!("{style_bold}{color_green}Creating New Project{style_reset}{color_reset}");
    println!("{style_bold}{color_bright_magenta}Name: {style_reset}{color_reset}{}", name);
    println!("{style_bold}{color_bright_magenta}Type: {style_reset}{color_reset}partition");

    new_dir(name);
    new_dir(format!("{}/shaders", name).as_str());

    new_dir(format!("{}/shaders/textures", name).as_str());
    new_dir(format!("{}/shaders/textures/luts", name).as_str());
    new_dir(format!("{}/shaders/textures/noise", name).as_str());
    new_dir(format!("{}/shaders/textures/misc", name).as_str());

    new_dir(format!("{}/shaders/programs", name).as_str());
    new_dir(format!("{}/shaders/programs/gbuffers", name).as_str());
    new_dir(format!("{}/shaders/programs/shadow", name).as_str());
    new_dir(format!("{}/shaders/programs/composite", name).as_str());
    new_dir(format!("{}/shaders/programs/prep", name).as_str());

    new_dir(format!("{}/shaders/libs", name).as_str());

    new_dir(format!("{}/shaders/lang", name).as_str());

    new_dir(format!("{}/shaders/world-1", name).as_str());
    new_dir(format!("{}/shaders/world0", name).as_str());
    new_dir(format!("{}/shaders/world1", name).as_str());

    println!("{style_bold}{color_green}Project Created{style_reset}{color_reset}");
}

fn u_partition(args: Vec<String>) {
    let name: &str = args.get(3).map(|s| s.as_str()).unwrap();
    println!("{style_bold}{color_green}Creating New Project{style_reset}{color_reset}");
    println!("{style_bold}{color_bright_magenta}Name: {style_reset}{color_reset}{}", name);
    println!("{style_bold}{color_bright_magenta}Type: {style_reset}{color_reset}un-partition");

    new_dir(name);
    new_dir(format!("{}/shaders", name).as_str());

    new_dir(format!("{}/shaders/textures", name).as_str());
    new_dir(format!("{}/shaders/textures/luts", name).as_str());
    new_dir(format!("{}/shaders/textures/noise", name).as_str());
    new_dir(format!("{}/shaders/textures/misc", name).as_str());

    new_dir(format!("{}/shaders/libs", name).as_str());

    new_dir(format!("{}/shaders/lang", name).as_str());

    new_dir(format!("{}/shaders/world-1", name).as_str());
    new_dir(format!("{}/shaders/world0", name).as_str());
    new_dir(format!("{}/shaders/world1", name).as_str());

    println!("{style_bold}{color_green}Project Created{style_reset}{color_reset}");
}

fn min(args: Vec<String>) {
    let name: &str = args.get(3).map(|s| s.as_str()).unwrap();
    println!("{style_bold}{color_green}Creating New Project{style_reset}{color_reset}");
    println!("{style_bold}{color_bright_magenta}Name: {style_reset}{color_reset}{}", name);
    println!("{style_bold}{color_bright_magenta}Type: {style_reset}{color_reset}minimal");

    new_dir(name);
    new_dir(format!("{}/shaders", name).as_str());

    println!("{style_bold}{color_green}Project Created{style_reset}{color_reset}")
}