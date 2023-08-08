# Hexmap RS

Rust library for handling hexagon map logic and math for games. Intended for use with [the Bevy game engine](https://bevyengine.org/), but the crate itself is designed to be engine-agnostic.

[![Static Badge](https://img.shields.io/badge/Patreon-NandoGamedev-FF424D?logo=patreon)](https://www.patreon.com/NandoGamedev)
[![Static Badge](https://img.shields.io/badge/Ko--Fi-nando__gamedev-FF5E5B?logo=ko-fi)](https://ko-fi.com/nando_gamedev)

This library is not intended for long term use on its own - it is a test bed for concepts for a more advanced tilemapping library, as well as a foundation for me to start working on my own hexagon based games before this more advanced library is ready for use. If you can't wait to start using my code to include tilemapping in your own game, migration from this crate to the new one when it eventually becomes available should be easy, as its design is based on and informed by this crate, as well as being designed with migrating my own game over in mind. Plus, the crate isn't too complicated at the moment that it could cause major architectural problems in a game that uses it.

## Copyright/License

Copyright 2023 Fernando A. Fraticelli

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 
