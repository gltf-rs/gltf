// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;
extern crate serde_json;

macro_rules! enum_string {
    ($name:ident {
        $($variant:ident = $value:expr,)*
    }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(u32)]
        pub enum $name {
            $($variant,)*
        }

        impl ::serde::de::Deserialize for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
                where D: ::serde::de::Deserializer
            {
                struct Visitor;
                impl ::serde::de::Visitor for Visitor {
                    type Value = $name;
                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter)
                                 -> ::std::fmt::Result
                    {
                        $(
                            let _ = formatter.write_str(concat!($value, "\n"))?;
                        )*
                            Ok(())
                    }

                    fn visit_str<E>(self, value: &str)-> Result<Self::Value, E>
                        where E: ::serde::de::Error
                    {
                        match value {
                            $($value => Ok($name::$variant),)*
                                bad => {
                                let msg = format!("invalid value: {}", bad);
                                    Err(E::custom(msg))
                                },
                        }
                    }
                }
                deserializer.deserialize_str(Visitor)
            }

        }

        impl ::serde::ser::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::ser::Serializer
            {
                match *self {
                    $( $name::$variant => serializer.serialize_str($value), )*
                }
            }
        }
    }
}

macro_rules! enum_number {
    ($name:ident { $($variant:ident = $value:expr, )* }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $($variant = $value,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer
            {
                // Serialize the enum as a u64.
                serializer.serialize_u64(*self as u64)
            }
        }

        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer
            {
                use std::fmt;

                struct Visitor;

                impl ::serde::de::Visitor for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<$name, E>
                        where E: ::serde::de::Error
                    {
                        // Rust does not come with a simple way of converting a
                        // number to an enum, so use a big `match`.
                        match value {
                            $( $value => Ok($name::$variant), )*
                            _ => Err(E::custom(
                                format!("unknown {} value: {}",
                                stringify!($name), value))),
                        }
                    }
                }

                // Deserialize the enum from a u64.
                deserializer.deserialize_u64(Visitor)
            }
        }
    }
}
