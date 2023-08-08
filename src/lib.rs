//! # Hexmap
//! 
//! Hexagon tile math and logic library for game development. Based on [the article *Hexagonal
//! Grids* by Red Blob Games](https://www.redblobgames.com/grids/hexagons/).
//! 
//! [![Static Badge](https://img.shields.io/badge/Patreon-NandoGamedev-FF424D?logo=patreon)](https://www.patreon.com/NandoGamedev)
//! [![Static Badge](https://img.shields.io/badge/Ko--Fi-nando__gamedev-FF5E5B?logo=ko-fi)](https://ko-fi.com/nando_gamedev)
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

    pub fn tile_spacing_x(&self) -> f32
    {
        match self {
            Self::PointyTop => constants::POINTY_TOP_X_SPACING,
            Self::FlatTop => constants::FLAT_TOP_X_SPACING,
        }
    }

    pub fn tile_spacing_y(&self) -> f32
    {
        match self {
            Self::PointyTop => constants::POINTY_TOP_Y_SPACING,
            Self::FlatTop => constants::FLAT_TOP_Y_SPACING,
        }
    }

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
pub mod constants
{
    /// Square root of 3
    pub const SQRT_3: f32 = 1.73205080757;

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
}
