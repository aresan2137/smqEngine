use std::path::{Path, PathBuf};
use std::process::Command;
use crate::editor::inspector::*;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct MeshAttributeBuild {
    pub shader_location: u32,
    pub format: String,
    pub offset: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct MeshLayoutBuild {
    pub array_stride: u64,
    pub step_mode: String,
    pub attributes: Vec<MeshAttributeBuild>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct MeshBuild {
    pub vertex_count: u32,
    pub index_count: u32,
    pub layout: MeshLayoutBuild,
}

#[allow(unused_variables)]
pub fn blend(ui: &mut egui::Ui, inspector_state: &mut InspectorState, path: &PathBuf) {
    ui.heading("Blender Model");
    ui.separator();

    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown");
    ui.label(format!("file: {}", file_name));

    ui.separator();

    if ui.button("Bake").clicked() {
        let relative_path = path.strip_prefix("smq_proj/assets/").unwrap_or(path);
        let tmp_gltf_path = std::path::PathBuf::from("smq_proj/tmp_assets/")
            .join(relative_path)
            .with_extension("glb");

        if let Some(parent) = tmp_gltf_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if call_blender_export(path, &tmp_gltf_path) {
            // ZAMIAST PLIKU, TWORZYMY FOLDER BEZ ROZSZERZENIA (.blend)
            let final_output_dir = path.with_extension(""); 
            compile_gltf_to_smf(&tmp_gltf_path, &final_output_dir);
        } else {
            eprintln!("failed to bake");
        }
    }
}

fn call_blender_export(blend_path: &Path, output_gltf_path: &Path) -> bool {
    let python_cmd = format!(
        "import bpy; bpy.ops.export_scene.gltf(filepath='{}', export_animations=False, export_image_format='NONE', export_materials='NONE')",
        output_gltf_path.display().to_string().replace("\\", "/")
    );

    let status = Command::new("blender")
        .arg("-b")
        .arg(blend_path)
        .arg("--python-expr")
        .arg(python_cmd)
        .status();

    match status {
        Ok(s) => s.success(),
        Err(_) => {
            eprintln!("blender not found");
            false
        }
    }
}

// Zmieniliśmy drugi argument na folder (output_dir)
fn compile_gltf_to_smf(gltf_path: &Path, output_dir: &Path) {
    let (document, buffers, _) = gltf::import(gltf_path)
        .expect("failed to load glb file");

    // Tworzymy folder docelowy dla wyodrębnionych obiektów
    let _ = std::fs::create_dir_all(output_dir);

    // Iterujemy po każdym meshu osobno!
    for mesh in document.meshes() {
        // Wyciągamy oryginalną nazwę z Blendera i robimy ją bezpieczną dla plików
        let raw_name = mesh.name().unwrap_or("unnamed_mesh");
        let safe_name = raw_name.replace(|c: char| !c.is_alphanumeric(), "_");

        let mut vertices: Vec<f32> = Vec::new();
        let mut vertex_count = 0;

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

        // Pomijamy puste meshe (np. same collidery lub ukryte obiekty bez geometrii)
        if vertex_count == 0 {
            continue; 
        }

        let array_stride = 32; 
        let metadata = MeshBuild {
            vertex_count,
            index_count: 0,
            layout: MeshLayoutBuild {
                array_stride,
                step_mode: "Vertex".to_string(),
                attributes: vec![
                    MeshAttributeBuild { shader_location: 0, format: "Float32x3".to_string(), offset: 0 },
                    MeshAttributeBuild { shader_location: 1, format: "Float32x2".to_string(), offset: 12 },
                    MeshAttributeBuild { shader_location: 2, format: "Float32x3".to_string(), offset: 12 + 8 },
                ],
            },
        };

        // --- ZAPIS PLIKU JSON DLA KONKRETNEGO MESHA ---
        let json_path = output_dir.join(format!("{}.smf.json", safe_name));
        if let Ok(json_str) = serde_json::to_string_pretty(&metadata) {
            let _ = std::fs::write(json_path, json_str);
        }

        // --- ZAPIS DANYCH BINARNYCH SMF ---
        let mut payload = Vec::new();
        payload.push(0b10110000);
        payload.extend_from_slice(&vertex_count.to_le_bytes()); 
        payload.extend_from_slice(&0u32.to_le_bytes()); 

        for float_val in vertices {
            payload.extend_from_slice(&float_val.to_le_bytes());
        }

        let smf_path = output_dir.join(format!("{}.smf", safe_name));
        if let Err(e) = std::fs::write(&smf_path, payload) {
            eprintln!("failed to save smf for {}: {}", safe_name, e);
        } else {
            println!("Exported: {}", smf_path.display());
        }
    }
}