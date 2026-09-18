use std::{path::{Path, PathBuf}, process::Command};

use crate::process_common_file_descriptor;

// if doesn't work check if blender is in the Path enviroment varible and restart vscode/console

pub fn draw(path: &PathBuf, ui: &mut egui::Ui) {
    let (file_path, _) = process_common_file_descriptor(path);
    ui.heading("Blender Model");
    ui.separator();

    let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown");
    ui.label(format!("file: {}", file_name));

    ui.separator();

    if ui.button("Bake").clicked() {
        let relative_path = file_path.strip_prefix("smq_proj/assets/").unwrap_or(&file_path);
        let tmp_gltf_path = std::path::PathBuf::from("smq_proj/tmp_assets/")
            .join(relative_path)
            .with_extension("glb");

        if let Some(parent) = tmp_gltf_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if call_blender_export(&file_path, &tmp_gltf_path) {
            let final_output_dir = file_path.with_extension(""); 
            compile_gltf_to_smf(&tmp_gltf_path, &final_output_dir);
            log::info!("bake sucess");
        } else {
            log::error!("failed to bake smf");
        }
    }
}

fn call_blender_export(blend_path: &Path, output_gltf_path: &Path) -> bool {
    let python_cmd = format!(
        "import bpy; bpy.ops.export_scene.gltf(filepath='{}', export_animations=False, export_image_format='NONE', export_materials='NONE')",
        output_gltf_path.display().to_string().replace("\\", "/")
    );

    let status = Command::new("blender").arg("-b").arg(blend_path).arg("--python-expr").arg(python_cmd).status();

    match status {
        Ok(s) => s.success(),
        Err(_) => {
            eprintln!("blender not found");
            false
        }
    }
}

pub fn compile_gltf_to_smf(gltf_path: &Path, output_dir: &Path) {
    let (document, buffers, _) = gltf::import(gltf_path).expect("failed to load glb file");

    if output_dir.exists() {
        let _ = std::fs::remove_dir_all(output_dir);
    }
    
    let _ = std::fs::create_dir_all(output_dir);

    for mesh in document.meshes() {
        let raw_name = mesh.name().unwrap_or("unnamed_mesh");
        let safe_name = raw_name.replace(|c: char| !c.is_alphanumeric(), "_");

        let mut vertices: Vec<f32> = Vec::new();
        let mut vertex_count: u32 = 0;

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| buffers.get(buffer.index()).map(|b| &b.0[..]));

            let positions = reader.read_positions().map(|p| p.collect::<Vec<_>>());
            let normals = reader.read_normals().map(|n| n.collect::<Vec<_>>());
            let tex_coords = reader.read_tex_coords(0).map(|t| t.into_f32().collect::<Vec<_>>());

            if let Some(pos_vec) = positions {
                let to_y_up = |v: [f32; 3]| -> [f32; 3] {
                    [v[0], v[1], v[2]]
                };

                if let Some(indices_iter) = reader.read_indices() {
                    let indices: Vec<u32> = indices_iter.into_u32().collect();
                    for idx in indices {
                        let i = idx as usize;

                        let transformed_pos = to_y_up(pos_vec[i]);
                        vertices.extend_from_slice(&transformed_pos);

                        if let Some(ref uv_vec) = tex_coords {
                            vertices.extend_from_slice(&uv_vec[i]);
                        } else {
                            vertices.extend_from_slice(&[0.0, 0.0]);
                        }

                        if let Some(ref norm_vec) = normals {
                            let transformed_norm = to_y_up(norm_vec[i]);
                            vertices.extend_from_slice(&transformed_norm);
                        } else {
                            vertices.extend_from_slice(&[0.0, 1.0, 0.0]);
                        }

                        vertex_count += 1;
                    }
                } else {
                    for i in 0..pos_vec.len() {
                        let transformed_pos = to_y_up(pos_vec[i]);
                        vertices.extend_from_slice(&transformed_pos);

                        if let Some(ref uv_vec) = tex_coords {
                            vertices.extend_from_slice(&uv_vec[i]);
                        } else {
                            vertices.extend_from_slice(&[0.0, 0.0]);
                        }

                        if let Some(ref norm_vec) = normals {
                            let transformed_norm = to_y_up(norm_vec[i]);
                            vertices.extend_from_slice(&transformed_norm);
                        } else {
                            vertices.extend_from_slice(&[0.0, 1.0, 0.0]);
                        }

                        vertex_count += 1;
                    }
                }
            }
        }

        if vertex_count == 0 {
            continue; 
        }

        let smf_path = output_dir.join(format!("{}.smf", safe_name));
        
        let vertex_bytes: Vec<u8> = vertices.iter().flat_map(|f| f.to_le_bytes()).collect();
        
        if let Err(e) = write_smf_file(&smf_path, vertex_count, &vertex_bytes) {
            log::error!("failed to save smf for {}: {}", safe_name, e);
        } else {
            log::info!("Exported: {}", smf_path.display());
        }
    }
}

pub fn write_smf_file(path: &Path, vertex_count: u32, vertex_data: &[u8]) -> std::io::Result<()> {
    let mut payload = Vec::new();
    
    payload.push(0b10110000);
    payload.extend_from_slice(&vertex_count.to_le_bytes()); 
    payload.extend_from_slice(&0u32.to_le_bytes()); 
    
    payload.extend_from_slice(vertex_data); 

    std::fs::write(path, payload)
}