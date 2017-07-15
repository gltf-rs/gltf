### Changelog

0.6.1 (15/07/17)

 * Removed the Send and Sync requirements from the Source trait

0.6.0 (15/07/17)

 * Added the wrapper interface.
 * Added the `Source` trait, which allows for customizing the import process.
 * Added the reference `Source` trait implementation, namely `FromPath`, that can
   read from the file system and decode embedded base64 data URIs.
 * Added support for binary glTF.
 * Added the `Validate` trait, which validates glTF JSON metadata.
 * Added the `Import` struct which drives the asynchronous loading of glTF data.
 * Implemented "poor man's zero-copy deserialization".
 * Moved all extension data structures into a new `extensions` module.
 * Removed the `v1` module.
 * Made the `Get` trait behave the same as the `TryGet` trait.
 * Removed the `TryGet` trait.

0.5.0 (10/06/17)

 * Added the `v2` module, containing all glTF 2.0 data structures.
 * Initial implementation of the glTF 2.0 data structures.

0.4.1 (06/05/17)

 * Internal improvements to decrease the crate build time.

0.4.0 (24/04/17)

 * Added the `v1` module, containing all glTF 1.0 data structures.
 * glTF objects are now categorised in submodules.
 * Completed implementation of the glTF 1.0 data structures.
 * Removed the `gl` crate dependency.

0.3.1 (17/03/17)

 * Updated hyperlinks to the official glTF documentation.

0.3.0 (16/02/17)

 * Allowed the crate to build on the latest stable `rustc` (1.15) 
   using the new `serde` frontend, i.e. with the serde `proc_macro`.

0.2.1 (17/11/16)

 * Allowed the crate to build on the latest stable `rustc` (1.14) 
   using the old `serde` frontend, i.e. with `serde_codegen`.

0.2.0 (15/11/16)

 * Added `Technique` for glTF 1.0.

0.1.1 (13/11/16)

 * Added documentation for glTF 1.0.

0.1.0 (13/11/16)

 * Initial (incomplete) glTF 1.0 implementation.
