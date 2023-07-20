mod axial_coords; pub use axial_coords::*;
mod cube_coords; pub use cube_coords::*;


pub trait HexCoords
{
    /// Generates a contiguous line of coordinates `a` to `b`.
    /// 
    /// Resulting vector should include `a` as the first element in the array and `b` as the last
    /// element. All remaining elements in between should be adjacent coordinates and in order from
    /// `a` to `b`.
    fn line(a: Self, b: Self) -> Vec<Self> where Self: Sized;

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
    fn ring(center: Self, radius: usize) -> Vec<Self> where Self: Sized;

    /// Generates a filled hexagonal area centered on the given `center` coordinates.
    /// 
    /// The `radius` argument represents how far away from the center the area will encompass. A
    /// radius of `0` will return only the center tile. A radius of `1` will return the center tile
    /// and the immediately adjacent tiles, one step away from the center. A radius of `2` will
    /// return all tiles 2 steps or less from the center, and so on.
    fn area(center: Self, radius: usize) -> Vec<Self> where Self: Sized;
}


/// Shortcut for [`AxialCoords::new`](crate::AxialCoords::new). Creates a new set of
/// [axial coordinates](crate::AxialCoords) with the provided values.
/// 
/// ```
/// use hexmap::{AxialCoords, axial};
/// assert_eq!(AxialCoords::new(1, 2), axial!(1, 2));
/// ```
#[macro_export]
macro_rules! axial {
    ($q:literal, $r:literal) => { AxialCoords::new($q, $r) }
}

/// Creates a new set of [`CubeCoords`](crate::CubeCoords) with the provided values. Acts as a
/// shortcut for [`CubeCoords::new`](crate::CubeCoords::new)
/// 
/// ```
/// use hexmap::{CubeCoords, cube};
/// assert_eq!(CubeCoords::new(1, 2, -3), cube!(1, 2, -3));
/// ```
#[macro_export]
macro_rules! cube {
    ($q:literal, $r:literal, $s:literal) => { CubeCoords::new($q, $r, $s) }
}
