# Notes

- (lygia has .wgsl files in a legacy format, we just want the .wesl ones)
  - probably the packaging api should take a glob or list of globs? (+ wesl_root..)
  - for now, modified wesl-rs to ignore .wgsl files
- added directory level .wesl files
  - underlying issue requiring the .wesl files looks fixable in package.rs, but glob syntax might be preferable.
- found a handful of errors in wesl validation
  - need to push those upstream into lygia
  - consider whether wgsl-js packager should do more validation
- package is lygia::lygia::Mod in rust, with 'lygia' duplicated. why?
- importing cubic in wesl fails with module not found. Fixed now.
  - perhaps a handy error message about alternate nearby names would be nice to have here?
- lexing failed on `Björn` due to the umlauts in validation
  - temporarily patched the lygia source to remove the validation
  - added some logging in validation to figure out which source file it was
    - probably should add some more error reporting to wesl-rs here
- dir 'common' fails, due to name conflict in rust
- module 'mod' fails, due to name conflict in rust

TODO:

- feat: add filtering support to find files in rust packager, e.g. glob syntax
- fix -rs issue where directory level .wesl files are required
- (lee) feat: add more validation to -js packager
- (lee) chore: push lygia wesl fixes upstream
- (lee) chore: file WESL issue for lygia re: plugging in raymarch map function
- fix: -rs parsing of umlauts
- feat: -rs error reporting on parsing failure
- fix: -rs support for package paths that conflict with rust keywords.