desmume-rs
==========

Bindings for the [DeSmuME interface](https://github.com/tasemulators/desmume/blob/master/desmume/README.INT).

The `desmume-rs` crate contains high level bindings, and the `desmume-sys` crate contains the FFI and 
links against DeSmuME, see its README for more information.

Note that under Windows binaries that use either `desmume-rs` or `desmume-sys` need to ship with a SDL2 DLL.
For more information and other dependencies, see the documentation of DeSmuME.

At the time of writing, you may also need to pass the `/SAFESEH:NO` linker argument for 32-bit Windows builds in
your crates build script. See the `build.rs` of `desmume-rs`.

This is not an "official" crate provided by the DeSmuME maintainers, but instead maintained by the 
[SkyTemple](https://skytemple.org) project.
