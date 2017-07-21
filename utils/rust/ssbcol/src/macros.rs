macro_rules! enum_bits {
    ($(#[$attr:meta])* pub enum $name:ident: $int_t:ty {
        $($variant:ident = $value:expr,)+
    }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $(#[$attr])*
        pub enum $name {
            $($variant = $value,)*
        }

        __impl_enum_bits! {
            enum $name: $int_t {
                $($variant = $value,)*
            }
        }
    };
    ($(#[$attr:meta])* enum $name:ident: $int_t:ty {
        $($variant:ident = $value:expr,)+
    }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $(#[$attr])*
        enum $name {
            $($variant = $value,)*
        }

        __impl_enum_bits! {
            enum $name: $int_t {
                $($variant = $value,)*
            }
        }
    }
}

macro_rules! __impl_enum_bits {
    (enum $name:ident: $int_t:ty {
        $($variant:ident = $value:expr,)+
    }) => {
        impl $name {
            pub fn from_bits(input: $int_t) -> ::std::result::Result<Self, String> {
                match input {
                    $($value => Ok($name::$variant),)*
                    _ => Err(format!("Value {:#X} is not a proper variant for enum {}", input, stringify!($name)))
                }
            }
        }
    }
}
