use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

fn data_dir() -> PathBuf {
    if let Ok(d) = std::env::var("STARFETCH_DATA") {
        PathBuf::from(d)
    } else {
        let exe = std::env::current_exe().unwrap_or_default();
        let rel = exe.parent().unwrap_or(Path::new(".")).join("res");
        if rel.is_dir() {
            rel
        } else {
            PathBuf::from("/usr/local/share/starfetch/")
        }
    }
}

const DIRECTORIES: [&str; 2] = ["constellations", "norse-constellations"];

fn color_map() -> HashMap<&'static str, &'static str> {
    [
        ("black", "\x1b[1;30m"),
        ("white", "\x1b[1;37m"),
        ("cyan", "\x1b[1;36m"),
        ("magenta", "\x1b[1;35m"),
        ("yellow", "\x1b[1;33m"),
        ("green", "\x1b[1;32m"),
        ("red", "\x1b[1;31m"),
        ("blue", "\x1b[1;34m"),
    ]
    .into_iter()
    .collect()
}

fn help(data: &Path) {
    let help_path = data.join("help_message.txt");
    if let Ok(content) = fs::read_to_string(help_path) {
        print!("{}", content);
    }
    process::exit(0);
}

fn error_msg(code: i32, extra: &str, data: &Path) {
    match code {
        0 => println!("Error: you must input a constellation name after -n.\n"),
        1 => println!("Error: '{}' isn't a valid argument.\n", extra),
        2 => println!("Error: the constellation you asked for isn't recognized.\n"),
        _ => {}
    }
    help(data);
}

fn shuffle_file() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".fileToParseWithConstellations.txt")
}

fn collect_constellation_paths(data: &Path) -> Vec<String> {
    let mut paths = Vec::new();
    for dir_name in &DIRECTORIES {
        let dir = data.join(dir_name);
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                paths.push(entry.path().to_string_lossy().into_owned());
            }
        }
    }
    paths
}

fn pick_constellation(data: &Path, use_random: bool, explicit_path: &str) -> String {
    let const_set = shuffle_file();

    let mut list: Vec<String> = Vec::new();
    if const_set.is_file() {
        if let Ok(content) = fs::read_to_string(&const_set) {
            for line in content.lines() {
                if !line.is_empty() {
                    list.push(line.to_string());
                }
            }
        }
    }

    if list.is_empty() {
        list = collect_constellation_paths(data);
        let mut rng = thread_rng();
        list.shuffle(&mut rng);
    }

    let chosen = if use_random {
        list.first().cloned().unwrap_or_default()
    } else {
        explicit_path.to_string()
    };

    if !list.is_empty() {
        list.remove(0);
    }

    if let Ok(()) = fs::write(&const_set, list.join("\n") + if list.is_empty() { "" } else { "\n" }) {
        // written
    }

    chosen
}

fn render_graph_line(graph: &Value, line_num: usize, color: &str) -> String {
    let line_key = format!("line{}", line_num);
    let line_obj = &graph[&line_key];
    let mut result = String::new();
    for k in 1..=22 {
        let k_str = k.to_string();
        if let Some(star) = line_obj.get(&k_str) {
            if let Some(s) = star.as_str() {
                result.push_str(color);
                result.push_str(s);
                result.push_str("\x1b[0;0m");
            } else {
                result.push(' ');
            }
        } else {
            result.push(' ');
        }
    }
    result
}

fn print_constellation(data: &Path, pathc: &str, color: &str) -> Result<()> {
    let template_path = data.join("template");
    let template = fs::read_to_string(&template_path)
        .with_context(|| format!("Cannot open template: {}", template_path.display()))?;

    let template = template.replace('^', "\x1b");

    let constellation_content = fs::read_to_string(pathc);
    let constellation_content = match constellation_content {
        Ok(c) => c,
        Err(_) => {
            error_msg(2, "", data);
            unreachable!()
        }
    };

    let j: Value = serde_json::from_str(&constellation_content)
        .with_context(|| format!("Invalid JSON in {}", pathc))?;

    let mut s = template;

    s = s.replacen("%0", j["title"].as_str().unwrap_or(""), 1);
    s = s.replacen("%11", j["name"].as_str().unwrap_or(""), 1);
    s = s.replacen("%12", j["quadrant"].as_str().unwrap_or(""), 1);
    s = s.replacen("%13", j["right ascension"].as_str().unwrap_or(""), 1);
    s = s.replacen("%14", j["declination"].as_str().unwrap_or(""), 1);
    s = s.replacen("%15", j["area"].as_str().unwrap_or(""), 1);
    s = s.replacen("%16", j["main stars"].as_str().unwrap_or(""), 1);

    for i in 1..=10 {
        let graph_line = render_graph_line(&j["graph"], i, color);
        let placeholder = format!("%{}", i);
        s = s.replacen(&placeholder, &graph_line, 1);
        s = s.replace("requestedColor", color);
    }

    println!("{}", s);
    Ok(())
}

fn print_list(data: &Path, color: &str) {
    for dir_name in &DIRECTORIES {
        let dir = data.join(dir_name);
        println!("\n{}✦ available {}\x1b[0;0m:", color, dir_name);
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let fname = entry.file_name();
                let name = fname.to_string_lossy();
                if name == ".DS_Store" {
                    continue;
                }
                let stem = name.strip_suffix(".json").unwrap_or(&name);
                println!("{}{}\x1b[0;0m", color, stem);
            }
        }
    }
}

fn main() {
    let data = data_dir();
    let args: Vec<String> = std::env::args().collect();
    let mut color = "\x1b[1;37m".to_string();
    let mut use_random = false;
    let mut pathc = String::new();

    if args.len() == 1 {
        use_random = true;
    } else {
        let flag_bytes = args[1].as_bytes();
        if flag_bytes.len() < 2 {
            error_msg(1, &args[1], &data);
        }
        match flag_bytes[1] {
            b'p' => {
                if args.len() < 3 {
                    error_msg(0, " ", &data);
                }
                pathc = data
                    .join(DIRECTORIES[0])
                    .join(format!("{}.json", args[2]))
                    .to_string_lossy()
                    .into_owned();
            }
            b'n' => {
                if args.len() < 3 {
                    error_msg(0, " ", &data);
                }
                if args[2] == "norse" && args.len() >= 4 {
                    pathc = data
                        .join(DIRECTORIES[1])
                        .join(format!("{}.json", args[3]))
                        .to_string_lossy()
                        .into_owned();
                } else {
                    pathc = data
                        .join(DIRECTORIES[0])
                        .join(format!("{}.json", args[2]))
                        .to_string_lossy()
                        .into_owned();
                }
            }
            b'h' => {
                help(&data);
            }
            b't' | b'r' => {
                use_random = true;
            }
            b'l' => {
                print_list(&data, &color);
                process::exit(0);
            }
            b'c' => {
                if args.len() == 2 {
                    println!(
                        "Available colors are: black, white, cyan, magenta, yellow, red, blue, green"
                    );
                    process::exit(0);
                } else if args.len() == 3 || args.len() == 4 {
                    let cmap = color_map();
                    if let Some(&c) = cmap.get(args[2].as_str()) {
                        color = c.to_string();
                    }
                    if args.len() == 4 && args[3] == "-l" {
                        print_list(&data, &color);
                        process::exit(0);
                    }
                    use_random = true;
                } else {
                    // argc >= 5: -c <color> -n <name>
                    let cmap = color_map();
                    if let Some(&c) = cmap.get(args[2].as_str()) {
                        color = c.to_string();
                    }
                    pathc = data
                        .join(DIRECTORIES[0])
                        .join(format!("{}.json", args[4]))
                        .to_string_lossy()
                        .into_owned();
                }
            }
            _ => {
                error_msg(1, &args[1], &data);
            }
        }
    }

    let chosen = pick_constellation(&data, use_random, &pathc);
    if let Err(e) = print_constellation(&data, &chosen, &color) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
