use std::{collections::HashMap, hash::Hash};
use crate::HexCoords;


mod pathfinding; pub use pathfinding::PathfindingTile;


pub struct HexMap<C, T>
{
    map: HashMap<C, T>
}

impl<C, T> HexMap<C, T>
where C: Copy + Eq + PartialEq + Hash + HexCoords
{
    pub fn new() -> Self
    {
        Self{ map: HashMap::new() }
    }

    pub fn get(&self, coords: C) -> Option<&T>
    {
        self.map.get(&coords)
    }

    pub fn get_mut(&mut self, coords: C) -> Option<&mut T>
    {
        self.map.get_mut(&coords)
    }

    pub fn insert(&mut self, coords: C, tile: T)
    {
        self.map.insert(coords, tile);
    }

    pub fn insert_area(&mut self, center: C, radius: usize, tile: T)
    where C: HexCoords, T: Clone
    {
        let area = C::area(center, radius);
        for coord in area
        {
            self.insert(coord, tile.clone());
        }
    }

    pub fn find_path(&self, start: C, destination: C) -> Option<Vec<C>>
    where C: Copy + PartialEq, T: pathfinding::PathfindingTile
    {
        let mut pathfinder = pathfinding::Pathfinder::default();
        pathfinder.find_path(start, destination, self)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<C, T>
    {
        self.map.iter()
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
        use crate::map::pathfinding::PathfindingTile;

        #[derive(Clone)]
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
            let path = map.find_path(start, end).expect("Expected to return path, got None instead");
            assert_eq!(0, path.len());
        }

        /// Ensures that when the start and end are adjacent, a path is returned
        /// with only the end coordinates in it
        #[test]
        fn start_adjacent_end()
        {
            let start: CubeCoords = cube!(0, 0, 0);
            let end: CubeCoords = cube!(0, 1, -1);
            let mut map: HexMap<CubeCoords, PathTestTile> = HexMap::new();
            map.insert_area(CubeCoords::ZERO, 2, PathTestTile::Cheap);
            let path = map.find_path(start, end).expect("Expected to return path, got None instead");
            assert_eq!(1, path.len());
            assert!(path.contains(&end));
            assert!(!path.contains(&start));
        }

        /// Ensures that a straight path is drawn between tiles when there is no
        /// pathfinding cost factor
        #[test]
        #[ignore]
        fn straight_path()
        {
            let start: CubeCoords = cube!(-2, 0, 2);
            let end: CubeCoords = cube!(2, 0, -2);
            let mut map: HexMap<CubeCoords, PathTestTile> = HexMap::new();
            map.insert_area(CubeCoords::ZERO, 2, PathTestTile::Cheap);
            let path = map.find_path(start, end).expect("Expected to find bath between start and end, but `None` was returned");
            assert_eq!(4, path.len());
            assert_eq!(cube!(-1, 0, 1), path[0]);
            assert_eq!(cube!(0, 0, 0), path[0]);
            assert_eq!(cube!(1, 0, -1), path[0]);
            assert_eq!(cube!(2, 0, -2), path[0]);
        }

        /// Ensures that the most cost efficient path is chosen between tiles,
        /// even when it isn't the straight path
        #[test]
        fn cost_efficient_path()
        {
            let start: CubeCoords = cube!(-2, 0, 2);
            let end: CubeCoords = cube!(2, 0, -2);
            // initialize map filled with expensive tiles
            let mut map: HexMap<CubeCoords, PathTestTile> = HexMap::new();
            map.insert_area(CubeCoords::ZERO, 2, PathTestTile::Expensive);
            // insert squiggly path of cheaper to move through tiles
            map.insert(cube!(-2, 0, 2), PathTestTile::Cheap);
            map.insert(cube!(-1, -1, 2), PathTestTile::Cheap);
            map.insert(cube!(0, -1, 1), PathTestTile::Cheap);
            map.insert(cube!(0, 0, 0), PathTestTile::Cheap);
            map.insert(cube!(0, 1, -1), PathTestTile::Cheap);
            map.insert(cube!(1, 1, -2), PathTestTile::Cheap);
            map.insert(cube!(2, 0, -2), PathTestTile::Cheap);
            
            let path = map.find_path(start, end).expect("Expected to find path between start and end, but `None` was returned");
            assert_eq!(6, path.len());
            assert_eq!(cube!(-1, -1, 2), path[0]);
            assert_eq!(cube!(0, -1, 1), path[1]);
            assert_eq!(cube!(0, 0, 0), path[2]);
            assert_eq!(cube!(0, 1, -1), path[3]);
            assert_eq!(cube!(1, 1, -2), path[4]);
            assert_eq!(cube!(2, 0, -2), path[5]);
        }

        /// Ensures that [`None`] is returned when no path can be found between
        /// the start and end
        #[test]
        fn no_path()
        {
            let start: CubeCoords = cube!(-1, 0, 1);
            let end: CubeCoords = cube!(1, 0, -1);
            let mut map: HexMap<CubeCoords, PathTestTile> = HexMap::new();
            // insert tile at start and end, but with no connecting tile between
            map.insert(start, PathTestTile::Cheap);
            map.insert(end, PathTestTile::Cheap);
            // find path - should be `None`
            let path = map.find_path(start, end);
            assert_eq!(None, path);
        }
    }
}