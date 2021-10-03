use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;

use bevy::prelude::*;
use bevy::reflect::{TypeUuidDynamic, Uuid};
use bevy::render::texture::{Extent3d, ImageType};
use dashmap::DashMap;

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
        return Color::rgb_u8(self.color[0], self.color[1], self.color[2]);
    }

    pub fn emissive(&self) -> Color {
        return Color::rgb_u8(self.emissive[0], self.emissive[1], self.emissive[2]);
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

    pub fn pbr(&self, normal: Option<Handle<Texture>>) -> StandardMaterial {
        StandardMaterial {
            base_color: self.color(),
            emissive: self.emissive(),
            double_sided: false,
            metallic: self.metalic(),
            reflectance: self.reflectance(),
            roughness: self.roughness(),
            unlit: self.unlit,

            normal_map: normal,

            ..Default::default()
        }
    }
}

impl TypeUuidDynamic for PbrConfig {
    fn type_uuid(&self) -> Uuid {
        Uuid::from_str("17bd4300-be62-4fbe-b32f-40e1a0294421").unwrap()
    }

    fn type_name(&self) -> &'static str {
        "PbrConfig"
    }
}

// impl TypeUuid for PbrConfig {
//     const TYPE_UUID: Uuid = Uuid::from_str("17bd4300-be62-4fbe-b32f-40e1a0294421").unwrap();
// }

pub struct MaterialsMapping {
    pub map: Arc<DashMap<u64, Handle<StandardMaterial>>>
}

impl Default for MaterialsMapping {
    fn default() -> Self {
        Self {
            map: Arc::new(DashMap::new())
        }
    }
}

pub fn load_materials(
    mut commands: Commands,
    mut textures: ResMut<Assets<Texture>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let box_mesh_handle = meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 1.0 }));
    commands.insert_resource(BoxMeshHandle(box_mesh_handle));

    let cwd = std::env::current_dir().unwrap().display().to_string();
    let path = format!("{0}/assets/materials/base.toml", cwd);

    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let dict: HashMap<String, PbrConfig> = toml::from_str(&contents).unwrap();

    let map = DashMap::new();

    // let mut pixel: Vec<u8> = Vec::new();
    // for x in 0..32 {
    //     for y in 0..32 {
    //         pixel.push(crate::noise::noise_2d(x as u64, y as u64, 54u64) as u8);
    //     }
    // }

    // let texture = Texture::new_fill(Extent3d::new(32, 32, 1), bevy::render::texture::TextureDimension::D2, &pixel, bevy::render::texture::TextureFormat::R8Uint);
    // let texture_handle = Some(textures.add(texture));
    
    for (_k, v) in dict {
        map.insert(v.id, materials.add(v.pbr(None)));
    }

    commands.insert_resource(MaterialsMapping {
        map: Arc::new(map)
    });
}
