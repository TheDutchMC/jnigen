mod models;
mod codegen;
mod ejni_types;
mod fmt;

use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use convert_case::{Case, Casing};
use log::{debug, error, info};
use structopt::StructOpt;
use crate::codegen::mod_file::gen_mod;
use crate::models::Input;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    input_file: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    output_dir: PathBuf,
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{}=INFO", env!("CARGO_PKG_NAME")));
    }
    env_logger::init();

    let opts: Opt = Opt::from_args();
    info!("Starting {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    debug!("Reading input");
    let f = match std::fs::File::open(opts.input_file) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to open input file: {}", e);
            exit(1)
        }
    };
    let input: Input = match serde_json::from_reader(&f) {
        Ok(i) => i,
        Err(e) => {
            error!("Failed to deserialize input: {}", e);
            exit(1);
        }
    };

    // Generate packages
    let src_dir = opts.output_dir.join("src");
    let root_path = input.packages.iter().min_by(|a, b| a.cmp(&b)).expect("Failed to find root path");
    info!("Using root path {}", root_path);

    // Create directories
    for pkg in &input.packages {
        let mut path = src_dir.clone();
        pkg
            .replace(&format!("{}.", root_path), "")
            .split(".")
            .for_each(|f| path = path.join(f));
        debug!("Creating package directory {:?}", path);
        std::fs::create_dir_all(&path).expect("Failed to create package");
    }

    // Generate interfaces -> traits
    for interface in &input.interfaces {
        let tokens = codegen::interface::gen_interface(&interface).to_string();
        let formatted = fmt::rustfmt(tokens).expect("Failed to run rustfmt");

        let out_path = build_path(&interface.name, root_path, &src_dir);
        let mut f = std::fs::File::create(&out_path).expect("Failed to open file");
        BufWriter::new(&mut f).write_all(formatted.as_bytes()).expect("Failed to write to file");
    }

    // Generate classes -> struct and struct impls
    for class in &input.classes {
        let tokens = codegen::class::class_def(&class).to_string();
        let formatted = fmt::rustfmt(tokens).expect("Failed to run rustfmt");

        let out_path = build_path(&class.name, root_path, &src_dir);
        let mut f = std::fs::File::create(&out_path).expect("Failed to open file");
        BufWriter::new(&mut f).write_all(formatted.as_bytes()).expect("Failed to write to file");
    }

    // Generated mod.rs files
    for pkg in &input.packages {
        debug!("{}", pkg);
        let mut path = src_dir.clone();
        pkg
            .replace(&format!("{}.", root_path), "")
            .replace(root_path, "")
            .split(".")
            .for_each(|f| path = path.join(f));

        let tokens = gen_mod(&path).expect("Failed to generate mod file").to_string();
        let formatted = fmt::rustfmt(tokens).expect("Failed to run rustfmt");

        let out_path = path.join("mod.rs");
        debug!("Generating mod file {:?}", out_path);

        let mut f = std::fs::File::create(&out_path).expect("Failed to open file");
        BufWriter::new(&mut f).write_all(formatted.as_bytes()).expect("Failed to write to file");
    }
}

fn build_path(name: &str, root_path: &str, src_dir: &Path) -> PathBuf {
    let path = name
        .replace(root_path, "")
        .replace(".", "/");
    let mut path = path
        .split("/")
        .collect::<Vec<_>>();

    let file_name = path.pop()
        .unwrap()
        .to_case(Case::Snake);
    let file_name = format!("{}.rs", file_name);
    path.push(&file_name);

    let mut out_path = PathBuf::from(src_dir);
    path.iter()
        .for_each(|f| out_path.push(f));

    out_path
}