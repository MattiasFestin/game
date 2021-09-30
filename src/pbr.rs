use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;
use bevy::reflect::{TypeUuidDynamic, Uuid};

pub struct BoxMeshHandle(pub Handle<Mesh>);

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct PbrConfig {
    pub uuid: Uuid,
    pub id: u64,
    
    pub unlit: bool,

    pub color: [u8; 3],
    pub emissive: [u8; 3],

    pub metalic: u8,
    pub roughness: u8,
    pub reflectance: u8,
    // pub clearcoat: u8,
    // pub clearcoatroughness: u8,
    // pub ansiotropy: u8,
}

impl TypeUuidDynamic for PbrConfig {
    fn type_uuid(&self) -> Uuid {
        self.uuid
    }

    fn type_name(&self) -> &'static str {
        "PbrConfig"
    }
}

impl Default for PbrConfig {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: Uuid::new_v4(),
            
            unlit: false,
            color: [0u8; 3],
            emissive: [0u8; 3],
            metalic: 0u8,
            reflectance: 0u8,
            roughness: 0u8,
        }
    }
}

impl PbrConfig {
    pub fn color(&self) -> Color {
        let h = (self.color[0] & 31) as f32;
        let s = (self.color[1] & 31) as f32;
        let l = (self.color[2] & 31) as f32;
        return Color::hsl(h, s, l);
    }

    pub fn emissive(&self) -> Color {
        let h = (self.emissive[0] & 31) as f32;
        let s = (self.emissive[1] & 31) as f32;
        let l = (self.emissive[2] & 31) as f32;
        return Color::hsl(h, s, l);
    }

    pub fn metalic(&self) -> f32 {
        return (self.metalic & 31) as f32 / 32.0f32;
    }

    pub fn roughness(&self) -> f32 {
        return (self.roughness & 31) as f32 / 32.0f32;
    }

    pub fn reflectance(&self) -> f32 {
        return (self.reflectance & 31) as f32 / 32.0f32;
    }

    // pub fn clearcoat(&self) -> f32 {
    //     return (self.clearcoat & 31) as f32 / 32.0f32;
    // }

    // pub fn clearcoatroughness(&self) -> f32 {
    //     return (self.clearcoatroughness & 31) as f32 / 32.0f32;
    // }

    // pub fn ansiotropy(&self) -> f32 {
    //     return (self.ansiotropy & 31) as f32 / 32.0f32;
    // }

    pub fn pbr(&self) -> StandardMaterial {
        StandardMaterial {
            base_color: self.color(),
            emissive: self.emissive(),
            double_sided: false,
            metallic: self.metalic(),
            reflectance: self.reflectance(),
            roughness: self.roughness(),
            unlit: self.unlit,

            ..Default::default()
        }
    }
}

// static mut MATERIAL_MAPPINGS: Option<HashMap<u64, StandardMaterial>> = None;
// static mut NUMBER_OF_MATERIALS: u64 = 0;
pub struct MaterialsMapping {
    pub map: HashMap<u64, StandardMaterial>
}

impl Default for MaterialsMapping {
    fn default() -> Self {
        Self {
            map: HashMap::new()
        }
    }
}

pub fn load_materials(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials_mappings: ResMut<MaterialsMapping>
) {
    let box_mesh_handle = meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 1.0 }));
    commands.insert_resource(BoxMeshHandle(box_mesh_handle));


    let cwd = std::env::current_dir().unwrap().display().to_string();
    let contents = fs::read_to_string(format!("{0}/assets/materials/base.toml", cwd))
        .expect("Something went wrong reading the file");

    let dict: HashMap<String, PbrConfig> = toml::from_str(&contents).unwrap();
    for (_k, v) in dict {
        // materials_mappings.map.insert(v.id, v.pbr());
    }
}
