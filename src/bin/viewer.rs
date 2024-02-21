use clap::Parser;
use std::{fmt::Debug, fs::File, path::PathBuf};
use web_splats::{open_window, RenderConfig};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Opt {
    /// Input file
    input: PathBuf,

    /// Scene json file
    scene: Option<PathBuf>,

    #[arg(long, default_value_t = false)]
    no_vsync: bool,

    /// Sky box image
    #[arg(long)]
    skybox: Option<PathBuf>,
}

/// check if there is a scene file in the same directory or parent directory as the input file
fn try_find_scene_file(input: &PathBuf, depth: u32) -> Option<PathBuf> {
    if let Some(parent) = input.parent() {
        let scene = parent.join("cameras.json");
        if scene.exists() {
            return Some(scene);
        }
        return try_find_scene_file(&parent.to_path_buf(), depth - 1);
    }
    return None;
}

#[pollster::main]
async fn main() {
    let mut opt = Opt::parse();

    if opt.scene.is_none() {
        opt.scene = try_find_scene_file(&opt.input, 2);
        log::warn!("No scene file specified, using {:?}", opt.scene);
    }
    let data_file = File::open(&opt.input).unwrap();

    let scene_file = opt.scene.as_ref().map(|p| File::open(p).unwrap());

    if opt.no_vsync {
        log::info!("V-sync disabled");
    }

    open_window(
        data_file,
        scene_file,
        RenderConfig {
            no_vsync: opt.no_vsync,
            skybox: opt.skybox,
        },
        Some(opt.input),
        opt.scene,
    )
    .await;
}
