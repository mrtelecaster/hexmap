//! Hexagon tile math and logic library for game development.
//! 
//! I wouldn't recommend using this library as anything other than an
//! educational example. I'm using it as a test bed for a more robust and useful
//! tile mapping library that's actually intended for use as a published crate.
//! 
//! Based on [the article *Hexagonal Grids* by Red Blob Games](https://www.redblobgames.com/grids/hexagons/).


mod coords; pub use coords::*;
mod map; pub use map::*;


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
