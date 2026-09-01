use std::{fs, path::Path};

use wgpu::naga::{self, ImageClass, ImageDimension, StructMember};
use naga::{front::wgsl, valid::{Validator}};

use std::error::Error;

pub struct WgslStorage {
    pub access: String
}

pub struct WgslImage {
    pub dim: ImageDimension,
    pub class: ImageClass,
}

pub enum WgslBindingType {
    Ubo(()),
    Storage(WgslStorage),
    Image(WgslImage),
    Sampler(())
}

pub struct WgslBinding {
    pub group: u32,
    pub binding: u32,
    pub var_name: String,
    pub var_type: String,
    pub binding_type: WgslBindingType   
}

#[derive(Clone, PartialEq)]
pub struct WgslStruct {
    pub name: String,
    pub vars: Vec<StructMember>
}

pub fn analize_wgsl_file(path: &Path, validator: &mut Validator) -> Result<(Vec<WgslBinding>, Vec<WgslStruct>), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let module = wgsl::parse_str(&source)?;

    validator.validate(&module)?;

    let mut bindings = Vec::with_capacity(module.global_variables.len());

    let mut structs = Vec::with_capacity(module.types.len());

    for (_, global) in module.global_variables.iter() {
        if let Some(binding) = &global.binding {
            let group = binding.group;
            let binding_idx = binding.binding;
            let var_name = global.name.as_deref().ok_or(format!("namless varible at group: {group}, and binding: {binding_idx}"))?;
            
            let ty = &module.types[global.ty];
            let var_type = ty.name.as_deref().ok_or(format!("namless type at group: {group}, and binding: {binding_idx}"))?;

            match global.space {
                naga::AddressSpace::Uniform => {
                    bindings.push(WgslBinding {
                        group: group,
                        binding: binding_idx,
                        var_name: var_name.to_string(), 
                        var_type: var_type.to_string(),
                        binding_type: WgslBindingType::Ubo(())
                    });
                }
                naga::AddressSpace::Storage { access } => {
                    let access = if access.contains(naga::StorageAccess::STORE) { "Read/Write" } else { "Read-Only" };
                    bindings.push(WgslBinding {
                        group: group,
                        binding: binding_idx,
                        var_name: var_name.to_string(), 
                        var_type: var_type.to_string(),
                        binding_type: WgslBindingType::Storage(WgslStorage { 
                            access: access.to_string()
                        })
                    });                
                }
                naga::AddressSpace::Handle => {
                    match &ty.inner {
                        naga::TypeInner::Image { class, dim, .. } => {
                            bindings.push(WgslBinding {
                                group: group,
                                binding: binding_idx,
                                var_name: var_name.to_string(), 
                                var_type: var_type.to_string(),
                                binding_type: WgslBindingType::Image(WgslImage { 
                                    dim: dim.clone(), 
                                    class: class.clone()
                                })
                            });   
                        }
                        naga::TypeInner::Sampler { comparison } => {
                            if *comparison { 
                                todo!("comparition sampler") 
                            };

                            bindings.push(WgslBinding {
                                group: group,
                                binding: binding_idx,
                                var_name: var_name.to_string(), 
                                var_type: var_type.to_string(),
                                binding_type: WgslBindingType::Sampler(())
                            });  
                        }
                        _ => todo!("some other handle"),
                    }
                }
                _ => {} 
            }
        }
    }

    for (_, ty) in module.types.iter() {
        if let naga::TypeInner::Struct { members, .. } = &ty.inner {
            let struct_name = ty.name.as_deref().ok_or(format!("namless struct"))?;
            
            if struct_name.starts_with('_') || struct_name.starts_with("gl_") { // skip buildin
                continue;
            }

            let mut struc = WgslStruct {
                name: struct_name.to_string(),
                vars: Vec::with_capacity(members.len())
            };

            for member in members {
                if member.name.is_none() { return Err(format!("namless struct var at: {struct_name}").into()); }
                struc.vars.push(member.clone());
            }

            structs.push(struc);
        }
    }

    return Ok((bindings, structs));

}

pub fn wgsl_analisys_to_ubos(analisys: &[(Vec<WgslBinding>, Vec<WgslStruct>)]) -> Result<Vec<WgslStruct>, Box<dyn Error>> {
    let mut final_ubo: Vec<WgslStruct> = Vec::new();

    for anlize in analisys.iter() {
        for struc in anlize.1.iter() {

            if !struc.name.starts_with("Ubo") {
                continue;
            }

            let existing_struct = final_ubo.iter().find(|s| s.name == struc.name);

            if let Some(existing) = existing_struct {
                if existing.vars != struc.vars {
                    return Err(format!("two structs with the same name have diffrent containts: {}", struc.name).into());
                }
            } else {
                final_ubo.push(struc.clone());
            }     
        }
    }

    return Ok(final_ubo);
}