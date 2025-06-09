# Notes

- (lygia has .wgsl files in a legacy format, we just want the .wesl ones)
  - probably the packaging api should take a glob or list of globs? (+ wesl_root..)
  - for now, modified wesl-rs to ignore .wgsl files
- started to add directory level .wesl files, starting with math.wesl
  - underlying issue requiring the .wesl files looks fixable in package.rs, but glob syntax might be preferable...
- found a handful of errors in wesl validation
  - need to push those upstream into lygia
  - consider whether wgsl-js packager should do more validation
- package is lygia::lygia::math::cubic in rust, with 'lygia' duplicated. why?
- importing cubic in wesl fails with module not found.
  - need to debug further..
  - perhaps a handy error message about alternate nearby names would be nice to have here?
- lexing failed on `Björn` due to the umlauts in validation
  - temporarily patched the lygia source
  - added some logging in validation to figure out which source file it was
