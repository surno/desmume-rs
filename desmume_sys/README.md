desmume-sys
===========

FFI for the [DeSmuME interface](https://github.com/tasemulators/desmume/blob/master/desmume/README.INT).

Note that under Windows binaries that use this need to ship with a SDL2 DLL.
For more information and other dependencies, see the documentation of DeSmuME.

This crate builds a fork of DeSmuME with some minor adjustments and no concrete stable version
number by default. At the moment there is no option to skip this and link to a dynamic system
library instead. Pull Requests are welcome.

The version of this crate does currently not correspond to any specific or stable release of DeSmuME.

This is not an "official" crate provided by the DeSmuME maintainers, but instead maintained by the
[SkyTemple](https://skytemple.org) project.
