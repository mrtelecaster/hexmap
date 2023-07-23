mod axial_coords; pub use axial_coords::*;
mod cube_coords; pub use cube_coords::*;


/// Trait for a type that can represent a coordinate on a hexagonal grid.
/// 
/// Included are [`AxialCoords`] and [`CubeCoords`] which implement this trait.
pub trait HexCoords
where Self: Sized
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

    /// Generates a filled hexagonal area centered on the given `center` coordinates.
    /// 
    /// The `radius` argument represents how far away from the center the area will encompass. A
    /// radius of `0` will return only the center tile. A radius of `1` will return the center tile
    /// and the immediately adjacent tiles, one step away from the center. A radius of `2` will
    /// return all tiles 2 steps or less from the center, and so on.
    fn area(center: Self, radius: usize) -> Vec<Self>;

    /// Generates a list of adjacent hexagons to the given `center` hexagon. Order of the resulting
    /// list is not defined.
    fn adjacent(center: Self) -> Vec<Self>;
}
