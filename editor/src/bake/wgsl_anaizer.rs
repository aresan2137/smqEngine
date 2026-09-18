use std::{fs, path::{Path, PathBuf}};

use wgpu::naga::{self, ImageClass, ImageDimension};
use naga::{front::wgsl, valid::{Validator}};

use std::error::Error;

// pub struct WgslStorage {
//     pub access: String
// }

// pub struct WgslImage {
//     pub dim: ImageDimension,
//     pub class: ImageClass,
// }

// pub enum WgslBindingType {
//     Ubo(()),
//     Storage(WgslStorage),
//     Image(WgslImage),
//     Sampler(())
// }

// pub struct WgslBinding {
//     pub group: u32,
//     pub binding: u32,
//     pub var_name: String,
//     pub var_type: String,
//     pub binding_type: WgslBindingType   
// }

#[derive(Clone, PartialEq)]
pub struct WgslStructVarType {
    pub rusty: String,
    pub required_structs: Vec<String>
}

#[derive(Clone, PartialEq)]
pub struct WgslStructVar {
    pub name: String,
    pub type_: WgslStructVarType,
    pub offset: u32
}

#[derive(Clone, PartialEq)]
pub struct WgslStruct {
    pub name: String,
    pub vars: Vec<WgslStructVar>
}

pub fn analize_wgsl_file(path: &Path, validator: &mut Validator) -> Result<Vec<WgslStruct>, Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let module = wgsl::parse_str(&source)?;

    validator.validate(&module)?;

    // let mut bindings = Vec::with_capacity(module.global_variables.len());

    let mut structs = Vec::with_capacity(module.types.len());

    // for (_, global) in module.global_variables.iter() {
    //     if let Some(binding) = &global.binding {
    //         let group = binding.group;
    //         let binding_idx = binding.binding;
    //         let var_name = global.name.as_deref().ok_or(format!("namless varible at group: {group}, and binding: {binding_idx}"))?;
            
    //         let ty = &module.types[global.ty];
    //         let var_type = ty.name.as_deref().unwrap_or("null");

    //         match global.space {
    //             naga::AddressSpace::Uniform => {
    //                 bindings.push(WgslBinding {
    //                     group: group,
    //                     binding: binding_idx,
    //                     var_name: var_name.to_string(), 
    //                     var_type: var_type.to_string(),
    //                     binding_type: WgslBindingType::Ubo(())
    //                 });
    //             }
    //             naga::AddressSpace::Storage { access } => {
    //                 let access = if access.contains(naga::StorageAccess::STORE) { "Read/Write" } else { "Read-Only" };
    //                 bindings.push(WgslBinding {
    //                     group: group,
    //                     binding: binding_idx,
    //                     var_name: var_name.to_string(), 
    //                     var_type: var_type.to_string(),
    //                     binding_type: WgslBindingType::Storage(WgslStorage { 
    //                         access: access.to_string()
    //                     })
    //                 });                
    //             }
    //             naga::AddressSpace::Handle => {
    //                 match &ty.inner {
    //                     naga::TypeInner::Image { class, dim, .. } => {
    //                         bindings.push(WgslBinding {
    //                             group: group,
    //                             binding: binding_idx,
    //                             var_name: var_name.to_string(), 
    //                             var_type: var_type.to_string(),
    //                             binding_type: WgslBindingType::Image(WgslImage { 
    //                                 dim: dim.clone(), 
    //                                 class: class.clone()
    //                             })
    //                         });   
    //                     }
    //                     naga::TypeInner::Sampler { comparison } => {
    //                         if *comparison { 
    //                             todo!("comparition sampler") 
    //                         };

    //                         bindings.push(WgslBinding {
    //                             group: group,
    //                             binding: binding_idx,
    //                             var_name: var_name.to_string(), 
    //                             var_type: var_type.to_string(),
    //                             binding_type: WgslBindingType::Sampler(())
    //                         });  
    //                     }
    //                     _ => todo!("some other handle")
    //                 }
    //             }
    //             _ => {} 
    //         }
    //     }
    // }

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
                let member_name = member.name.clone().ok_or(format!("namless struct var at: {struct_name}"))?;

                let type_ = get_element_type(member.ty, &module).map_err(|e| {
                    format!("Error parsing type for {}.{}: {}", struct_name, member_name, e)
                })?;

                struc.vars.push(WgslStructVar {
                    name: (member.name.clone().ok_or("eoeoeoeoeoeo")?).to_string(),
                    type_,
                    offset: member.offset
                });
            }

            structs.push(struc);
        }
    }

    return Ok(structs);

}

fn get_element_type(handle: wgpu::naga::Handle<naga::Type>, module: &naga::Module) -> Result<WgslStructVarType, String> {
    let ty = &module.types[handle];

    match &ty.inner {
        naga::TypeInner::Scalar(scalar) => {
            let rusty = match scalar.kind {
                naga::ScalarKind::Float => Ok("f32".to_string()),
                naga::ScalarKind::Sint => Ok("i32".to_string()),
                naga::ScalarKind::Uint => Ok("u32".to_string()),
                _ => Err("unknown scalar type".to_string())
            }?;

            return Ok(WgslStructVarType { 
                rusty,
                required_structs: Vec::new()
            });
        },
        naga::TypeInner::Vector { size, scalar } => {
            let prefix = match scalar.kind {
                naga::ScalarKind::Float => "Vec",
                naga::ScalarKind::Sint => "IVec",
                naga::ScalarKind::Uint => "UVec",
                _ => return Err("unknown Vector type".to_string()),
            };
            let dim = match size {
                naga::VectorSize::Bi => "2",
                naga::VectorSize::Tri => "3",
                naga::VectorSize::Quad => "4",
            };

            return Ok(WgslStructVarType { 
                rusty: format!("{}{}", prefix, dim), 
                required_structs: Vec::new()
            });
        }
        naga::TypeInner::Matrix { columns, rows, .. } => {
            let rusty = if *columns == naga::VectorSize::Quad && *rows == naga::VectorSize::Quad {
                Ok("Mat4".to_string())
            } else if *columns == naga::VectorSize::Tri && *rows == naga::VectorSize::Tri {
                Ok("Mat3".to_string())
            } else {
                Err("unknown Matrix size".to_string())
            }?;

            return Ok(WgslStructVarType { 
                rusty, 
                required_structs: Vec::new()
            });
        }
        naga::TypeInner::Array { base, size, .. } => {
            let base_str = get_element_type(*base, module)?;

            let size_str = match size {
                naga::ArraySize::Constant(num) => num.get().to_string(),
                naga::ArraySize::Pending(_) => return Err("Pending array sizes unsupported".to_string()),
                naga::ArraySize::Dynamic => return Err("dynamic array sizes unsupported".to_string()),
            };

            return Ok(WgslStructVarType { 
                rusty: format!("[{}; {}]", base_str.rusty, size_str),
                required_structs: vec![base_str.rusty]
            });           
        }
        naga::TypeInner::Struct { .. } => {
            if let Some(name) = &ty.name {
                return Ok(WgslStructVarType { 
                rusty: name.clone(), 
                required_structs: Vec::new()
            })
            } else {
                Err("Unnamed struct found".to_string())
            }
        }
        _ => Err("Unsupported TypeInner".to_string()),
    }
}

pub fn wgsl_analisys_to_ubos(analisys: &[Vec<WgslStruct>]) -> Result<Vec<WgslStruct>, String> {
    let mut final_ubo: Vec<WgslStruct> = Vec::new();

    for anlize in analisys.iter() {
        for struc in anlize.iter() {

            if !struc.name.starts_with("Ubo") && !struc.name.starts_with("Uc") {
                continue;
            }

            let existing_struct = final_ubo.iter().find(|s| s.name == struc.name);

            if let Some(existing) = existing_struct {
                if existing.vars != struc.vars {
                    return Err(format!("two structs with the same name have diffrent containts: {}", struc.name));
                }
            } else {
                final_ubo.push(struc.clone());
            }     
        }
    }

    return Ok(final_ubo);
}

fn process_struct_and_give_aligment(ubo: &WgslStruct, ubos: &[WgslStruct], struct_code: &mut String, fn_code: &mut String, ret_code: &mut String, ret_struct_code: &mut String) -> Result<(u32, u32), String> {
    let struct_name = &ubo.name;
    let var_name = struct_name.to_lowercase();

    let mut current_offset: u32 = 0;
    let mut max_align: u32 = 0;
    let mut pad_counter: u32 = 0;

    let mut struct_code_cp = format!("\n#[repr(C)]\n#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]\npub struct {} {{", struct_name);

    let mut builder = format!("
        impl Default for {} {{
            fn default() -> Self {{
                Self {{", struct_name
    );

    for var in &ubo.vars {
        let (var_size, var_align) = get_var_size_anigment(var, ubos, struct_code,fn_code, ret_code, ret_struct_code)?;

        if var_align > max_align { max_align = var_align; }

        if var.offset > current_offset {
            let padding_needed = var.offset - current_offset;
            
            struct_code_cp += &format!("\npub _pad{}: [u8; {}],", pad_counter, padding_needed);
            builder += &format!("\n_pad{}: [0; {}],", pad_counter, padding_needed);
            
            pad_counter += 1;
            current_offset = var.offset;
        } else if var.offset < current_offset {
            return Err(format!("overlap in: {} at: {}", struct_name, var.name));
        }

        struct_code_cp += &format!("\npub {}: {},", var.name, var.type_.rusty);

        let default_value_str: String = match var.type_.rusty.as_str() {
            "i32" | "u32" => "0".to_string(),
            "f32" => "0.0".to_string(),
            "Vec2" | "IVec2" | "UVec2" | 
            "Vec3" | "IVec3" | "UVec3" | 
            "Vec4" | "IVec4" | "UVec4" => format!("{}::ZERO", var.type_.rusty),
            "Mat3" | "Mat4" => format!("{}::IDENTITY", var.type_.rusty),
            _ => format!("Default::default()")
        };
        
        builder += &format!("\n{}: {},", var.name, default_value_str);

        current_offset += var_size;
    }

    let final_size = if max_align > 0 { (current_offset + max_align - 1) & !(max_align - 1) } else { 0 };
    let end_padding = final_size - current_offset;
    
    if end_padding > 0 {
        struct_code_cp += &format!("\npub _pad{}: [u8; {}],", pad_counter, end_padding);
        builder += &format!("\n_pad{}: [0; {}],", pad_counter, end_padding);
    }

    struct_code_cp += &format!("\n}}\n{builder}\n}}\n}}\n}}\n\n");

    *struct_code += &struct_code_cp;

    if ubo.name.starts_with("Ubo") {
        
        *fn_code += &format!("\nlet mut {} = Ubo::new(context, {}::default());", var_name, struct_name);
        *ret_code += &format!("\n{},", var_name);
        *ret_struct_code += &format!("\npub {}: Ubo<{}>,", var_name, struct_name);
    }

    return Ok((final_size, max_align));
}

pub fn get_var_size_anigment(var: &WgslStructVar, ubos: &[WgslStruct], struct_code: &mut String, fn_code: &mut String, ret_code: &mut String, ret_struct_code: &mut String) -> Result<(u32, u32), String> {
    let mut type_name = var.type_.rusty.as_str();
    let mut array_len: u32 = 1;
    let mut is_array = false;

    if type_name.starts_with('[') && type_name.ends_with(']') {
        is_array = true;
        let parts: Vec<&str> = type_name[1..type_name.len()-1].split(';').collect();
        if parts.len() == 2 {
            type_name = parts[0].trim();
            array_len = parts[1].trim().parse().unwrap_or(1);
        }
    }

    let (base_size, align) = match type_name {
        "i32" | "u32" | "f32" => (4, 4),
        "Vec2" | "IVec2" | "UVec2" => (8, 8),
        "Vec3" | "IVec3" | "UVec3" => (12, 16),
        "Vec4" | "IVec4" | "UVec4" => (16, 16),
        "Mat3" => (48, 16),
        "Mat4" => (64, 16),
        _ => {
            if var.type_.required_structs.len() >= 1 {
                let req_struct = &var.type_.required_structs[0];
                let ubo = ubos.iter().find(|u| &u.name == req_struct).ok_or(format!("required struct doesnt exist: {}", req_struct))?;
                process_struct_and_give_aligment(ubo, ubos, struct_code, fn_code, ret_code, ret_struct_code)?
            } else {
                return Err(format!("cant give aligment for something unknown: {}", type_name));
            }
        }
    };

    if is_array {
        let stride = (base_size + align - 1) & !(align - 1);
        Ok((stride * array_len, align))
    } else {
        Ok((base_size, align))
    }
}

pub fn create_rust_ubo_structs(ubos: &[WgslStruct], struct_code: &mut String, fn_code: &mut String, ret_code: &mut String, ret_struct_code: &mut String) -> Result<(), String> {
    for ubo in ubos.iter() {

        // todays meal: spaggeti code

        if ubo.name.starts_with("Uc") {
            continue;
        }

        let _ = process_struct_and_give_aligment(ubo, ubos, struct_code, fn_code, ret_code, ret_struct_code)?;
    }
    return Ok(());
}