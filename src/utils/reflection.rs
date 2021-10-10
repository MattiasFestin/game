use bevy::prelude::*;
use bevy::render::renderer::RenderResources;
use bevy::reflect::*;

pub trait Reflectable {
    fn struct_name() -> &'static str;
    fn field_names() -> &'static [&'static str];
}

#[macro_export] macro_rules! resource {
    (#[uuid = $uuid:expr] struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        #[derive(Debug, RenderResources, TypeUuid, Clone)]
        #[uuid = $uuid]
        pub struct $name {
            $(pub $fname : $ftype),*
        }

        impl Default for $name {
            fn default() -> Self {
                Self { 
                    $($fname : Default::default()),*
                }
            }
            
        }

        impl Reflectable for $name {
            fn struct_name() -> &'static str {
                static NAME: &'static str =  stringify!($name);
                NAME
            }
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    }
}