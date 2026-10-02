use smq_engine::{other::bvh::save_bvh_data, save_custom_ssf};

use std::{error::Error, fs, path::PathBuf};

mod wgsl_anaizer;

use crate::editor::{EditorState, write_smf_file};

mod assets;
use assets::*;

mod bvh;
use bvh::*;

pub fn bake(state: &EditorState) -> Result<(), Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();

    files.extend(bake_assets(state)?);

    let mut smffs: Vec<Vec<u8>> = Vec::with_capacity(files.len());

    for smf in &files {
        if smf.extension().is_some_and(|ext| ext == "smf") {
            let data = fs::read(smf)?;
            smffs.push(data);
        }
    }

    let smf_slices: Vec<&[u8]> = smffs.iter().map(|v| v.as_slice()).collect();

    let bvh_gen = build_mega_geometry(&smf_slices, 32);

    let bvh_path = PathBuf::from("smq_proj/build/bvh.smf");
    write_smf_file(&bvh_path, bvh_gen.vertex_count, &bvh_gen.mega_vertex_bytes)?;
    files.push(bvh_path);

    let bvh_path_2 = PathBuf::from("smq_proj/build/bvh.bin");
    save_bvh_data(&bvh_path_2, &bvh_gen.mega_nodes, &bvh_gen.bvh_roots)?;
    files.push(bvh_path_2);

    let mut comp: Vec<(PathBuf, String, u8)> = Vec::new();
    comp.reserve(files.len());

    for file in files {
        comp.push((
            file.clone(),
            file.display()
                .to_string()
                .replace("\\", "/")
                .replace("smq_proj/data/", "")
                .replace("/", "_")
                .replace(".", "_"),
            0x00,
        ));
    }

    save_custom_ssf("game", comp);

    log::info!("bake sucess");

    return Ok(());
}
