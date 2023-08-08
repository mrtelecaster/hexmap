mod axial; pub use axial::*;
mod cube; pub use cube::*;
mod macros; pub use macros::*;

#[cfg(feature="bevy")]
use bevy::prelude::Vec3;

use crate::{Orientation, constants::{FLAT_TOP_CORNERS, POINTY_TOP_CORNERS}};


/// Trait for a type that can represent a coordinate on a hexagonal grid.
/// 
/// Included are [`AxialCoords`] and [`CubeCoords`] which implement this trait.
pub trait HexCoords
where Self: Clone + Copy + Sized
{
    /// Generates a contiguous line of coordinates `a` to `b`.
    /// 
    /// Resulting vector should include `a` as the first element in the array and `b` as the last
    /// element. All remaining elements in between should be adjacent coordinates and in order from
    /// `a` to `b`.
    fn line(a: Self, b: Self) -> Vec<Self>;

    /// Generates a "ring" of hexagon coordinates centered around the given point.
    /// 
    /// The radius represents how many tiles out from the center the ring will be created. For
    /// example, with a radius of `1`, the ring of tiles immediately adjacent to the center point
    /// will be returned. These tiles can be thought of as being one step away from the center, as
    /// it requires only one move from the center to reach each of these tiles. With a radius of
    /// `2`, the ring of tiles will be two steps away from center, the next ring outwards adjacent
    /// to the ring of radius `1`. Radius `3` will get the ring of tiles adjacent to a ring of
    /// radius `2`, all 3 steps from center, and so on.
    /// 
    /// A radius of `0` returns only the center tile.
    fn ring(center: Self, radius: usize) -> Vec<Self>;

    /// Generates a list of adjacent hexagons to the given `center` hexagon. Order of the resulting
    /// list is not defined.
    fn adjacent(center: Self) -> Vec<Self>;

	/// Gets the position of the center of this tile on the X/Y plane
    fn to_world(&self, orientation: Orientation) -> (f32, f32);

	/// Gets the tile coordinates closest to the given position on the X/Y plane
    fn from_world(x: f32, y: f32, orientation: Orientation) -> Self;

    /// Generates a filled hexagonal area centered on the given `center` coordinates.
    /// 
    /// The `radius` argument represents how far away from the center the area will encompass. A
    /// radius of `0` will return only the center tile. A radius of `1` will return the center tile
    /// and the immediately adjacent tiles, one step away from the center. A radius of `2` will
    /// return all tiles 2 steps or less from the center, and so on.
    fn area(center: Self, radius: usize) -> Vec<Self>
    {
        let mut output = Vec::new();
        for i in 0..radius+1
        {
            let mut ring = Self::ring(center, i);
            output.append(&mut ring);
        }
        output
    }

    fn corners(&self, orientation: Orientation) -> [(f32, f32);6]
    {
        let (center_x, center_y) = self.to_world(orientation);
        let corners = orientation.tile_corners();
        let output_corners = [
            (center_x + corners[0].0, center_y + corners[0].1),
            (center_x + corners[1].0, center_y + corners[1].1),
            (center_x + corners[2].0, center_y + corners[2].1),
            (center_x + corners[3].0, center_y + corners[3].1),
            (center_x + corners[4].0, center_y + corners[4].1),
            (center_x + corners[5].0, center_y + corners[5].1),
        ];
        output_corners
    }

	/// Gets the tile nearest to the given position in Bevy Engine space
    #[cfg(feature="bevy")]
    fn from_vec3(vec: Vec3, orientation: Orientation) -> Self {
        Self::from_world(vec.x, vec.z, orientation)
    }

	/// Gets the center position of this tile as a Bevy Engine [`Vec3`]
    #[cfg(feature="bevy")]
    fn to_vec3(&self, orientation: Orientation) -> Vec3 {
        let (x, z) = self.to_world(orientation);
        bevy::prelude::Vec3::new(x, 0.0, z)
    }
}
