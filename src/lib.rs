//! # Hexmap
//! 
//! Hexagon tile math and logic library for game development. Based on [the article *Hexagonal
//! Grids* by Red Blob Games](https://www.redblobgames.com/grids/hexagons/).
//! 
//! [![Static Badge](https://img.shields.io/badge/Patreon-NandoGamedev-FF424D?logo=patreon)](https://www.patreon.com/NandoGamedev)
//! [![Static Badge](https://img.shields.io/badge/Ko--Fi-nando__gamedev-FF5E5B?logo=ko-fi)](https://ko-fi.com/nando_gamedev)
//! 
//! This library is not intended for long term use on its own - it is a test bed for concepts for a
//! more advanced tilemapping library, as well as a foundation for me to start working on my own
//! hexagon based games before this more advanced library is ready for use. If you can't wait to
//! start using my code to include tilemapping in your own game, migration from this crate to the
//! new one when it eventually becomes available should be easy, as its design is based on and
//! informed by this crate, as well as being designed with migrating my own game over in mind. Plus,
//! the crate isn't too complicated at the moment that it could cause major architectural problems
//! in a game that uses it.
//! 
//! ## Copyright/License
//! 
//! Copyright 2023 Fernando A. Fraticelli
//! 
//! This program is free software: you can redistribute it and/or modify it under the terms of the
//! GNU General Public License as published by the Free Software Foundation, either version 3 of the
//! License, or (at your option) any later version.
//! 
//! This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
//! without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
//! the GNU General Public License for more details.
//! 
//! You should have received a copy of the GNU General Public License along with this program. If
//! not, see <https://www.gnu.org/licenses/>. 


mod coords; pub use coords::*;
mod map; pub use map::*;


/// Certain algorithms require an orientation that determines how the hex grid is oriented on the X/Y plane
#[derive(Clone, Copy, Debug)]
pub enum Orientation
{
    PointyTop,
    FlatTop,
}

impl Orientation
{
    /// Width of a tile in this orientation along the X-axis
    pub fn tile_width(&self) -> f32
    {
        match self {
            Self::PointyTop => constants::POINTY_TOP_WIDTH,
            Self::FlatTop => constants::FLAT_TOP_WIDTH,
        }
    }

    /// Height of a tile along this orientation along the Y-axis
    pub fn tile_height(&self) -> f32
    {
        match self {
            Self::PointyTop => constants::POINTY_TOP_HEIGHT,
            Self::FlatTop => constants::FLAT_TOP_HEIGHT,
        }
    }

	/// X-axis or horizontal spacing between tiles of a given orientation
    pub fn tile_spacing_x(&self) -> f32
    {
        match self {
            Self::PointyTop => constants::POINTY_TOP_X_SPACING,
            Self::FlatTop => constants::FLAT_TOP_X_SPACING,
        }
    }

	/// Y-axis or vertical spacing between tiles of a given orientation
    pub fn tile_spacing_y(&self) -> f32
    {
        match self {
            Self::PointyTop => constants::POINTY_TOP_Y_SPACING,
            Self::FlatTop => constants::FLAT_TOP_Y_SPACING,
        }
    }

	/// The corner positions of the hexagon representing a tile of a given orientation
    pub fn tile_corners(&self) -> [(f32, f32);6]
    {
        match self {
            Self::PointyTop => constants::POINTY_TOP_CORNERS,
            Self::FlatTop => constants::FLAT_TOP_CORNERS,
        }
    }
}


/// Constants used by the crate. Mostly used for hexagon sizes and corner positions.
/// 
/// <https://www.redblobgames.com/grids/hexagons/#spacing>
mod constants
{
    /// Distance between top and bottom flats
    pub const FLAT_TOP_HEIGHT: f32 = SQRT_3;

    /// Distance between left and right points
    pub const FLAT_TOP_WIDTH: f32 = 2.0;

    pub const FLAT_TOP_X_SPACING: f32 = FLAT_TOP_WIDTH * 3.0 / 4.0;

    pub const FLAT_TOP_Y_SPACING: f32 = FLAT_TOP_HEIGHT;

    /// Corner positions of a flat topped hexagon
    pub const FLAT_TOP_CORNERS: [(f32, f32);6] = [
        (-FLAT_TOP_WIDTH / 4.0, FLAT_TOP_HEIGHT / 2.0),
        (FLAT_TOP_WIDTH / 4.0, FLAT_TOP_HEIGHT / 2.0),
        (FLAT_TOP_WIDTH / 2.0, 0.0),
        (FLAT_TOP_WIDTH / 4.0, -FLAT_TOP_HEIGHT / 2.0),
        (-FLAT_TOP_WIDTH / 4.0, -FLAT_TOP_HEIGHT / 2.0),
        (-FLAT_TOP_WIDTH / 2.0, 0.0),
    ];
    
    /// Distance between top and bottom points
    pub const POINTY_TOP_HEIGHT: f32 = 2.0;

    /// Distance between left and right flats
    pub const POINTY_TOP_WIDTH: f32 = SQRT_3;

    /// Corner positions of a pointy topped hexagon
    pub const POINTY_TOP_CORNERS: [(f32, f32);6] = [
        (0.0, POINTY_TOP_HEIGHT / 2.0),
        (POINTY_TOP_WIDTH / 2.0, POINTY_TOP_HEIGHT / 4.0),
        (POINTY_TOP_WIDTH / 2.0, -POINTY_TOP_HEIGHT / 4.0),
        (0.0, -POINTY_TOP_HEIGHT / 2.0),
        (-POINTY_TOP_WIDTH / 2.0, -POINTY_TOP_HEIGHT / 4.0),
        (-POINTY_TOP_WIDTH / 2.0, POINTY_TOP_HEIGHT / 4.0),
    ];

    pub const POINTY_TOP_X_SPACING: f32 = POINTY_TOP_WIDTH;

    pub const POINTY_TOP_Y_SPACING: f32 = POINTY_TOP_HEIGHT / 4.0 * 3.0;

	/// Square root of 3
    const SQRT_3: f32 = 1.73205080757;
}
