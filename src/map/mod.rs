use std::{collections::HashMap, hash::Hash};

use crate::HexCoords;


pub trait PathfindingTile
{
    fn pathfind_cost(&self) -> f32 {
        0.05
    }
}


pub struct HexMap<C, T>
{
    map: HashMap<C, T>
}

impl<C, T> HexMap<C, T>
{
    pub fn new() -> Self
    {
        Self{ map: HashMap::new() }
    }

    pub fn get(&self, coords: C) -> Option<&T>
    where C: Eq + Hash
    {
        self.map.get(&coords)
    }

    pub fn insert(&mut self, coords: C, tile: T)
    where C: Eq + Hash
    {
        self.map.insert(coords, tile);
    }

    pub fn insert_area(&mut self, center: C, radius: usize, tile: T)
    where C: Eq + Hash + HexCoords, T: Clone
    {
        let area = C::area(center, radius);
        for coord in area
        {
            self.insert(coord, tile.clone());
        }
    }

    pub fn find_path(&self, start: C, destination: C) -> Option<Vec<C>>
    where T: PathfindingTile
    {
        None
    }
}


#[cfg(test)]
mod tests
{
    use crate::{AxialCoords, CubeCoords, axial, cube};
    use super::*;

    /// Ensures that [`HexMap`] can be constructed using the coordinate types
    #[test]
    fn type_compatability()
    {
        let mut axial_map: HexMap<AxialCoords, ()> = HexMap::new();
        axial_map.insert(axial!(1, 0), ());
        assert_eq!(Some(&()), axial_map.get(axial!(1, 0)));
        assert_eq!(None, axial_map.get(axial!(0, 0)));

        let mut cube_map: HexMap<CubeCoords, ()> = HexMap::new();
        cube_map.insert(cube!(1, 0, -1), ());
        assert_eq!(Some(&()), cube_map.get(cube!(1, 0, -1)));
        assert_eq!(None, cube_map.get(cube!(0, 0, 0)))
    }

    mod pathfinding
    {
        use super::*;

        enum PathTestTile
        {
            Cheap,
            Expensive,
        }

        impl PathfindingTile for PathTestTile
        {
            fn pathfind_cost(&self) -> f32 {
                match self {
                    Self::Cheap => 0.5,
                    Self::Expensive => 2.0,
                }
            }
        }

        /// Ensures that when the start and end are the same tile, a path is
        /// returned with no elements in it
        #[test]
        fn start_is_end()
        {
            let start: CubeCoords = CubeCoords::ZERO;
            let end: CubeCoords = CubeCoords::ZERO;
            let map: HexMap<CubeCoords, PathTestTile> = HexMap::new();
            let path = map.find_path(start, end).unwrap();
            assert_eq!(0, path.len());
        }

        /// Ensures that when the start and end are adjacent, a path is returned
        /// with only the end coordinates in it
        #[test]
        fn start_adjacent_end()
        {

        }

        /// Ensures that a straight path is drawn between tiles when there is no
        /// pathfinding cost factor
        #[test]
        fn straight_path()
        {

        }

        /// Ensures that the most cost efficient path is chosen between tiles,
        /// even when it isn't the straight path
        #[test]
        fn cost_efficient_path()
        {

        }

        /// Ensures that [`None`] is returned when no path can be found between
        /// the start and end
        #[test]
        fn no_path()
        {

        }
    }
}