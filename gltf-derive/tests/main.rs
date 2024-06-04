/// Collection for extension data without built-in support.
///
/// The deserialize and serialize macros assume a field called `unrecognized_extensions` exists in
/// every data structure with this type.
type UnrecognizedExtensions = serde_json::Map<String, serde_json::Value>;

mod boolean {
    #[derive(gltf_derive::Deserialize, gltf_derive::Serialize)]
    struct Example {
        pub x: bool,
        pub unrecognized_extensions: crate::UnrecognizedExtensions,
    }

    #[test]
    fn serialize() {
        let a = Example {
            x: false,
            unrecognized_extensions: Default::default(),
        };
        assert_eq!("{}", serde_json::to_string(&a).unwrap());

        let b = Example {
            x: true,
            unrecognized_extensions: Default::default(),
        };
        assert_eq!("{\"x\":true}", serde_json::to_string(&b).unwrap());
    }

    #[test]
    fn deserialize() {
        let a = serde_json::from_str::<Example>("{}").unwrap();
        assert!(!a.x);

        let b = serde_json::from_str::<Example>("{\"x\":false}").unwrap();
        assert!(!b.x);

        let c = serde_json::from_str::<Example>("{\"x\":true}").unwrap();
        assert!(c.x);
    }
}

mod option {
    #[derive(gltf_derive::Default, gltf_derive::Deserialize, gltf_derive::Serialize)]
    struct Example {
        pub required: i32,
        pub optional: Option<i32>,
        pub unrecognized_extensions: crate::UnrecognizedExtensions,
    }

    #[test]
    fn serialize() {
        let a = Example {
            required: 123,
            optional: None,
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(
            "{\"required\":123}",
            serde_json::to_string(&a).unwrap().as_str()
        );

        let b = Example {
            required: 123,
            optional: Some(456),
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(
            "{\"required\":123,\"optional\":456}",
            serde_json::to_string(&b).unwrap().as_str()
        );
    }

    #[test]
    fn deserialize() {
        let a = serde_json::from_str::<Example>("{\"required\":123}").unwrap();
        assert_eq!(a.required, 123);
        assert_eq!(a.optional, None);

        let b = serde_json::from_str::<Example>("{\"required\":123,\"optional\":456}").unwrap();
        assert_eq!(b.required, 123);
        assert_eq!(b.optional, Some(456));
    }
}

mod vec {
    #[derive(gltf_derive::Deserialize, gltf_derive::Serialize)]
    struct Example {
        pub xs: Vec<i32>,
        pub unrecognized_extensions: crate::UnrecognizedExtensions,
    }

    #[test]
    fn serialize() {
        let a = Example {
            xs: vec![123],
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(
            "{\"xs\":[123]}",
            serde_json::to_string(&a).unwrap().as_str()
        );

        let b = Example {
            xs: Vec::new(),
            unrecognized_extensions: Default::default(),
        };
        assert_eq!("{}", serde_json::to_string(&b).unwrap().as_str());
    }

    #[test]
    fn deserialize() {
        let a = serde_json::from_str::<Example>("{\"xs\":[123]}").unwrap();
        assert_eq!(a.xs.as_slice(), &[123]);

        let b = serde_json::from_str::<Example>("{}").unwrap();
        assert!(b.xs.is_empty());
    }
}

mod camel_case {
    #[derive(gltf_derive::Deserialize, gltf_derive::Serialize)]
    struct Example {
        /// This field should be converted to camel case automatically.
        pub the_quick_brown_fox: i32,
        /// This field has an explicit name override and should not be converted to camel case.
        #[serde(rename = "TheLazyDog")]
        pub the_lazy_dog: i32,
        pub unrecognized_extensions: crate::UnrecognizedExtensions,
    }

    #[test]
    fn serialize() {
        let a = Example {
            the_quick_brown_fox: 123,
            the_lazy_dog: 456,
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(
            "{\"theQuickBrownFox\":123,\"TheLazyDog\":456}",
            serde_json::to_string(&a).unwrap().as_str()
        );
    }

    #[test]
    fn deserialize() {
        let a = serde_json::from_str::<Example>("{\"theQuickBrownFox\":123,\"TheLazyDog\":456}")
            .unwrap();
        assert_eq!(a.the_quick_brown_fox, 123);
        assert_eq!(a.the_lazy_dog, 456);
    }
}

mod extensions {
    use serde_json::Value;

    #[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, PartialEq, Eq)]
    struct Y(pub i32);

    #[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, PartialEq, Eq)]
    struct Z(pub i32);

    #[derive(gltf_derive::Deserialize, gltf_derive::Serialize, Debug, PartialEq, Eq)]
    struct Example {
        /// Regular field.
        pub x: i32,
        /// Field corresponding to an extension named `EXT_y`.
        #[gltf(extension = "EXT_y")]
        pub y: Option<Y>,
        /// Field corresponding to an extension named `EXT_z`.
        #[gltf(extension = "EXT_z")]
        pub z: Option<Z>,
        /// Collects extension data that doesn't have built-in support.
        ///
        /// This field must appear on every data structure.
        pub unrecognized_extensions: crate::UnrecognizedExtensions,
    }

    #[test]
    fn serialize() {
        let a = Example {
            x: 123,
            y: Some(Y(456)),
            z: None,
            unrecognized_extensions: {
                let mut map = crate::UnrecognizedExtensions::default();
                map.insert("EXT_?".to_owned(), Value::Number(789.into()));
                map
            },
        };
        assert_eq!(
            r#"{"x":123,"extensions":{"EXT_y":456,"EXT_?":789}}"#,
            serde_json::to_string(&a).unwrap().as_str()
        );

        let b = Example {
            x: 123,
            y: None,
            z: Some(Z(789)),
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(
            r#"{"x":123,"extensions":{"EXT_z":789}}"#,
            serde_json::to_string(&b).unwrap().as_str()
        );

        let c = Example {
            x: 123,
            y: Some(Y(456)),
            z: Some(Z(789)),
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(
            r#"{"x":123,"extensions":{"EXT_y":456,"EXT_z":789}}"#,
            serde_json::to_string(&c).unwrap().as_str()
        );

        let d = Example {
            x: 123,
            y: None,
            z: None,
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(r#"{"x":123}"#, serde_json::to_string(&d).unwrap().as_str());
    }

    #[test]
    fn deserialize() {
        let a =
            serde_json::from_str::<Example>(r#"{"x":123,"extensions":{"EXT_y":456,"EXT_?":789}}"#)
                .unwrap();
        assert_eq!(a.x, 123);
        assert_eq!(a.y, Some(Y(456)));
        assert_eq!(a.z, None);
        assert_eq!(
            a.unrecognized_extensions.get("EXT_?").cloned(),
            Some(Value::Number(789.into()))
        );
        assert_eq!(a.unrecognized_extensions.len(), 1);

        let b = serde_json::from_str::<Example>(r#"{"x":123,"extensions":{"EXT_z":789}}"#).unwrap();
        assert_eq!(b.x, 123);
        assert_eq!(b.y, None);
        assert_eq!(b.z, Some(Z(789)));
        assert!(b.unrecognized_extensions.is_empty());

        let c =
            serde_json::from_str::<Example>(r#"{"x":123,"extensions":{"EXT_y":456,"EXT_z":789}}"#)
                .unwrap();
        assert_eq!(c.x, 123);
        assert_eq!(c.y, Some(Y(456)));
        assert_eq!(c.z, Some(Z(789)));
        assert!(c.unrecognized_extensions.is_empty());

        let d = serde_json::from_str::<Example>(r#"{"x":123}"#).unwrap();
        assert_eq!(d.x, 123);
        assert_eq!(d.y, None);
        assert_eq!(d.z, None);
        assert!(d.unrecognized_extensions.is_empty());
    }
}

mod default {
    #[derive(gltf_derive::Default, gltf_derive::Deserialize, gltf_derive::Serialize, Debug)]
    struct Example {
        #[gltf(default)]
        pub x: i32,
        #[gltf(default = 123)]
        pub y: i32,
        #[gltf(default = [1.2, 3.4])]
        pub z: [f32; 2],
        pub unrecognized_extensions: crate::UnrecognizedExtensions,
    }

    #[test]
    fn default() {
        let a: Example = Default::default();
        assert_eq!(a.x, 0);
        assert_eq!(a.y, 123);
        assert_eq!(a.z, [1.2, 3.4]);
    }

    #[test]
    fn serialize() {
        let a = Example {
            x: 123,
            y: 456,
            z: [7.8, 9.0],
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(
            r#"{"x":123,"y":456,"z":[7.8,9.0]}"#,
            serde_json::to_string(&a).unwrap(),
        );

        let b = Example {
            x: 123,
            y: 123,
            z: [7.8, 9.0],
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(
            r#"{"x":123,"z":[7.8,9.0]}"#,
            serde_json::to_string(&b).unwrap(),
        );

        let c = Example {
            x: 123,
            y: 456,
            z: [1.2, 3.4],
            unrecognized_extensions: Default::default(),
        };
        assert_eq!(r#"{"x":123,"y":456}"#, serde_json::to_string(&c).unwrap());

        let d = Example::default();
        assert_eq!(r#"{}"#, serde_json::to_string(&d).unwrap());
    }

    #[test]
    fn deserialize() {
        let a = serde_json::from_str::<Example>("{}").unwrap();
        assert_eq!(a.x, 0);
        assert_eq!(a.y, 123);
        assert_eq!(a.z, [1.2, 3.4]);

        let b = serde_json::from_str::<Example>(r#"{"x":1,"y":2,"z":[3.4,5.6]}"#).unwrap();
        assert_eq!(b.x, 1);
        assert_eq!(b.y, 2);
        assert_eq!(b.z, [3.4, 5.6]);
    }
}
