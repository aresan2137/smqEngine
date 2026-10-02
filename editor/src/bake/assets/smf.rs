use std::path::PathBuf;

use walkdir::WalkDir;

use crate::{bake::assets::converters::path_to_var};

pub fn create_smf_bake(
    iterer: &Vec<(PathBuf, String)>,
    files: &mut Vec<PathBuf>,
) -> (i32, String, String) {
    let mut smf_files = 0;
    let mut smf_enum = "".to_string();
    let mut smf_gen = "".to_string();

    for (path, ext) in iterer {
        match ext.as_str() {
            "blend" => {
                for entry in WalkDir::new(path.with_extension(""))
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry.path().is_file() {
                        let smf_var_name = path_to_var(entry.path());

                        smf_gen += &format!(
                            "\nMesh::new(context, ssf_data[{}].as_ref(), 32, Some(\"{}\")).unwrap(),",
                            files.len() as i32,
                            smf_var_name
                        );

                        smf_enum += &format!("{} = {},", smf_var_name, smf_files);

                        smf_files += 1;

                        files.push(entry.path().to_path_buf());
                    }
                }
            }
            _ => {}
        }
    }

    return (smf_files, smf_enum, smf_gen);
}
