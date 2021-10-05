pub trait Reflectable {
    fn struct_name() -> &'static str;
    fn field_names() -> &'static [&'static str];
}

#[macro_export] macro_rules! reflectable {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl Reflectable for $name {
            fn struct_name() -> &'static str {
                $name
            }
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    }
}