use crate::{json::Root, Document, Error, Material};

fn as_validation_err(err: &Error) -> Option<&[(json::Path, json::validation::Error)]> {
    match err {
        Error::Validation(errors) => Some(errors),
        _ => None,
    }
}

fn validation_errors(
    err: Option<&Error>,
) -> impl Iterator<Item = json::validation::Error> + use<'_> {
    err.and_then(as_validation_err)
        .into_iter()
        .flat_map(|errs| errs.iter().map(|(_, e)| e).copied())
}

fn expect_parse_err(raw_json: &str) -> Document {
    let root = Root::from_str(raw_json).unwrap();

    let parse_error = Document::from_json(root.clone()).err();
    let validation_errs = validation_errors(parse_error.as_ref());

    let expected = std::iter::once(json::validation::Error::IndexOutOfBounds);

    assert!(
        validation_errs.eq(expected),
        "should contain exactly one validation index out of bounds error: {parse_error:?} "
    );

    Document::from_json_without_validation(root)
}

fn assert_access_err<'a, T>(
    document: &'a Document,
    access: impl Fn(Material<'a>) -> Option<Result<T, json::validation::Error>>,
) where
    T: 'a,
{
    let material = document.nth_material(0).unwrap();
    let actual_err = access(material).map(Result::err).flatten();

    assert_eq!(actual_err, Some(json::validation::Error::IndexOutOfBounds));
}

#[test]
fn try_base_color_texture() {
    let raw = r#"
        {
            "materials" : [
                {
                    "pbrMetallicRoughness": {
                        "baseColorTexture": {
                            "index": 0
                        }
                    }
                }
            ],
            "asset": {
                "version": "2.0"
            }
        }"#;

    let document = expect_parse_err(raw);

    assert_access_err(&document, |m| {
        m.pbr_metallic_roughness().try_base_color_texture()
    });
}

#[test]
fn try_emissive_texture() {
    let raw = r#"
        {
            "materials" : [
                {
                    "emissiveTexture": {
                        "index": 0
                    }
                }
            ],
            "asset": {
                "version": "2.0"
            }
        }"#;

    let document = expect_parse_err(raw);

    assert_access_err(&document, |m| m.try_emissive_texture());
}

#[cfg(feature = "KHR_materials_specular")]
#[test]
fn try_specular_texture() {
    let raw = r#"
        {
            "materials" : [
                {
                    "extensions": {
                        "KHR_materials_specular": {
                            "specularTexture": {
                                "index": 0
                            }
                        }
                    }
                }
            ],
            "asset": {
                "version": "2.0"
            }
        }"#;

    let document = expect_parse_err(raw);

    assert_access_err(&document, |m| m.specular().unwrap().try_specular_texture());
}
