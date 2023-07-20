//! Hexagon tile math and logic library for game development.
//! 
//! I wouldn't recommend using this library as anything other than an
//! educational example. I'm using it as a test bed for a more robust and useful
//! tile mapping library that's actually intended for use as a published crate.
//! 
//! Based on [the article *Hexagonal Grids* by Red Blob Games](https://www.redblobgames.com/grids/hexagons/).


mod coords; pub use coords::*;
mod map; pub use map::*;


pub enum HexOrientation
{
    PointyTop,
    FlatTop,
}


pub mod constants
{
    /// Square root of 3
    pub const SQRT_3: f32 = 1.73205080757;

    /// Distance between top and bottom flats
    pub const FLAT_TOP_HEIGHT: f32 = SQRT_3;

    /// Distance between left and right points
    pub const FLAT_TOP_WIDTH: f32 = 2.0;

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

    pub const POINTY_TOP_VERTICAL_SPACING: f32 = POINTY_TOP_HEIGHT / 4.0 * 3.0;
}
