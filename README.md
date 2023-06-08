desmume-rs
==========

Bindings for the [DeSmuME interface](https://github.com/tasemulators/desmume/blob/master/desmume/README.INT).

The `desmume-rs` crate contains high level bindings, and the `desmume-sys` crate contains the FFI and 
links against DeSmuME, see its README for more information.

Note that under Windows binaries that use either `desmume-rs` or `desmume-sys` need to ship with a SDL2 DLL.
For more information and other dependencies, see the documentation of DeSmuME.

This is not an "official" crate provided by the DeSmuME maintainers, but instead maintained by the 
[SkyTemple](https://skytemple.org) project.
