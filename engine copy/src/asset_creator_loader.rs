use std::{fs::{self, File}, path::Path};
use flate2::read::GzDecoder;
use crate::*;


pub fn load_assets(context: &Context, asf_filepath: &Path, ssf_filepath: &Path) {
    let file = File::open(asf_filepath).expect("failed to load asf file");
    
    let decoder = GzDecoder::new(file);
    
    let data: AssetCreatorData = bincode::deserialize_from(decoder).expect("failed to unpack asset data");

    let assets = load_ssf(fs::read(ssf_filepath).expect("failed to load ssf file")).expect("failed to parse ssf file");

    let mut textures = Vec::with_capacity(data.textures.len());
    for texture in data.textures {
        match &assets[texture.id as usize] {
            SSFAsset::Binary(data) => {
                textures.push(TextureS::from_asset_creator(context, data.as_slice(), texture));
            },
            _ => panic!("expected SSFAsset::Binary got sompthing else")
        }        
    }

    let mut render_textures = Vec::with_capacity(data.render_textures.len());
    for render_texture in data.render_textures {
        render_textures.push(RenderTexture::from_asset_creator(context, render_texture)); 
    }

    let mut ubos = Vec::with_capacity(data.render_textures.len());
    for ubo in data.render_textures {
        ubos.push(RenderTexture::from_asset_creator(context, render_texture)); 
    }
}