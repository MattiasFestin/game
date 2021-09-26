#[macro_use] extern crate bevy;
#[macro_use] extern crate bevycheck;
#[macro_use] extern crate serde;

use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::str::FromStr;

use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::reflect::{TypeUuid, TypeUuidDynamic, Uuid};
use bevy::render::colorspace::HslRepresentation;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::pbr::*;
use futures_lite::future;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_env.system())
        .add_startup_system(add_assets.system())
        .add_startup_system(spawn_tasks.system())
        .add_system(handle_tasks.system())
        .run();
}

#[derive(Debug, PartialEq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
struct Voxel {
    pub position: Vec3,
    pub pbr_id: u64
}

impl Default for Voxel {
    fn default() -> Self {
        Self { position: Vec3::ZERO , pbr_id: 0u64 }
    }
}


const CHUNK_SIZE: usize = 10;
const CHUNK_SIZE_CUBE: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[derive(Debug, PartialEq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
struct VoxelChunk {
    pub position: Vec3,
    pub voxels: Vec<Voxel>
}

impl Default for VoxelChunk {
    fn default() -> Self {
        Self { 
            position: Vec3::ZERO,
            voxels: (1..CHUNK_SIZE_CUBE).into_iter().map(|index| {
                let rest = index;
                let x = rest % CHUNK_SIZE;

                let rest = (rest - x) / CHUNK_SIZE;
                let y = rest % CHUNK_SIZE;

                let rest = (rest - y) / CHUNK_SIZE;
                let z = rest % CHUNK_SIZE;


                let position = Vec3::new(x as f32, y as f32, z as f32); //TODO: Chunk Offset
                Voxel {
                    position,
                    pbr_id: ((x + y) % 2) as u64
                }
            }).collect() 
        }
    }
}


struct BoxMeshHandle(Handle<Mesh>);

// #[derive(Debug, TypeUuid)]
// #[uuid = "f28c2ec3-0d0c-4ecd-8622-63e6d4262a60"]
// struct MaterialMapping {
//     map: HashMap<u64, PbrConfig>
// }

static mut material_mappings: Option<HashMap<u64, StandardMaterial>> = None;


fn add_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let box_mesh_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    commands.insert_resource(BoxMeshHandle(box_mesh_handle));


    let cwd = std::env::current_dir().unwrap().display().to_string();
    let contents = fs::read_to_string(format!("{0}/assets/materials/base.toml", cwd))
        .expect("Something went wrong reading the file");

    let dict: HashMap<String, PbrConfig> = toml::from_str(&contents).unwrap();
    let mut map = HashMap::new();
    for (k, v) in dict {
        map.insert(v.id, v.pbr());
        // let material_id = HandleId::new(v.uuid, v.id);
        // let handle = Handle::<StandardMaterial>::weak(material_id);
    
        //TODO: Ref
        // materials.add(v.pbr());
    //     let pbr = v.pbr();
    //     if materials.get(handle.clone()).is_none() {
    //         materials.set_untracked(handle.clone(), pbr);
    //     }
    }

    unsafe {
        material_mappings = Some(map);
    }
}

fn spawn_tasks(
        mut commands: Commands,
        thread_pool: Res<AsyncComputeTaskPool>
    ) {
    //TODO: Spawn in new chunks
    let chunk = VoxelChunk::default();

    for voxel in chunk.voxels {
        let task = thread_pool.spawn(async move {
            let mut v= voxel.clone();
            // v.pbr_id = Some(HandleId::new(
            //     Uuid::from_str("2642b340-7267-4b72-8af7-bb4f279508dd").unwrap(), 
            //     2
            // ));
            return v;
        });

        // Spawn new entity and add our new task as a component
        commands.spawn().insert(task);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct PbrConfig {
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

fn handle_tasks<'a>(
    mut commands: Commands,
    mut voxel_chunk_tasks: Query<(Entity, &mut Task<Voxel>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    box_mesh_handle: Res<BoxMeshHandle>,
    // material_mappings: ResMut<MaterialMapping>,
    // box_other_material_handle: Res<BoxOtherMaterialHandle>,
) {
    for (entity, mut task) in voxel_chunk_tasks.iter_mut() {
        if let Some(voxel) = future::block_on(future::poll_once(&mut *task)) {

            // let uuid = Uuid::from_u128((voxel.pbr_id * 6299357110868187461) as u128);
            // let material_id = HandleId::new(uuid, voxel.pbr_id);
            
            // let handle = Handle::<StandardMaterial>::weak(material_id);
            // let mat: StandardMaterial;
            // {
            //     // let path = &voxel.pbr_id.to_string();
            //     let pbr= materials.get(path).unwrap();
            //     mat = StandardMaterial {
            //         base_color: pbr.base_color,
            //         emissive: pbr.emissive,
            //         ..Default::default()
            //     };
            //     println!("handle: {:?}", pbr);
            // }
            
			// if let Some(handler_id) = voxel.material {
                // materials.get_or_insert_with(handler_id, || {

                // });
            let mut mat: Option<&StandardMaterial> = None;
            unsafe {
                if let Some(mm) = &material_mappings {
                    println!("pbr_id: {:?}", voxel.pbr_id);
                    mat = mm.get(&voxel.pbr_id);
                }
            }

            if let Some(m) = mat {
                commands.entity(entity).insert_bundle(PbrBundle {
                    mesh: box_mesh_handle.0.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: m.base_color,
                        ..Default::default()
                    }),
                    transform: Transform::from_translation(voxel.position),
                    ..Default::default()
                });
            }
			// }
            // if let Some(material) = materials.get(&voxel.pbr_id) {
            //         let material_id = HandleId::new(material.uuid, material.id);
            //         let handle = Handle::<StandardMaterial>::weak(material_id);
                
            //         let pbr = material.pbr();
            //         if materials.get(handle.clone()).is_none() {
            //             materials.set_untracked(handle.clone(), pbr);
            //         }

                    
            // }

            //TODO: Load from disk
            // let surf = match voxel.pbr_id {
            //     0 => {
            //         Some(box_material_handle.0.clone())
            //     },

            //     1 => {
            //         let material = PbrConfig {
            //             id: voxel.pbr_id,
            //             uuid: Uuid::from_u128((voxel.pbr_id * 6299357110868187461) as u128),
            //             color: [0, 255, 255],
            //             ..Default::default()
            //         };
            //         let material_id = HandleId::new(material.uuid, material.id);
            //         let pbr = material.pbr();
            //         let handle = Handle::<StandardMaterial>::weak(material_id);
            //         // materials.get_or_insert_with(handle, insert_fn)
                    
            //         if materials.get(handle.clone()).is_none() {
            //             materials.set_untracked(handle.clone(), pbr);
            //         }

            //         Some(handle.clone())
            //     }

            //     _ => {
            //         None
            //     }
            // };

            // if let Some(mat) = surf {
                
            // }
            
            

            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<Task<Voxel>>();
        }
    }
}

fn setup_env(mut commands: Commands) {
    // Used to center camera on spawned cubes
    let offset = if CHUNK_SIZE % 2 == 0 {
        (CHUNK_SIZE / 2) as f32 - 0.5
    } else {
        (CHUNK_SIZE / 2) as f32
    };

    // println!("offset: {:?}", offset);

    // lights
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.8, 0.8, 0.8)),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(offset, offset, CHUNK_SIZE as f32 * 3.0f32))
            .looking_at(Vec3::new(offset, offset, 0.0), Vec3::Y),
        ..Default::default()
    });
}