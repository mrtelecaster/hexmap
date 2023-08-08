use std::{collections::HashMap, hash::Hash};
use serde::{Deserialize, Serialize};
use crate::{HexCoords, AxialCoords, CubeCoords};

#[cfg(feature="bevy")]
use bevy::prelude::Resource;

mod pathfinding; pub use pathfinding::*;

pub type AxialMap<T> = HexMap<AxialCoords, T>;
pub type CubeMap<T> = HexMap<CubeCoords, T>;


/// A map of hexagonal tiles
/// 
/// `C` should be a hexagonal coordinate type and represents the "key" of the map. `T` can be any
/// type supplied by the user, and is the type of the tiles stored in the map, indexed by coordinates `C`
#[cfg_attr(feature="bevy", derive(Resource))]
#[derive(Deserialize, Serialize)]
pub struct HexMap<C, T>
where C: Eq + Hash
{
    map: HashMap<C, T>
}

impl<C, T> HexMap<C, T>
where C: Copy + Eq + PartialEq + Hash + HexCoords
{
	/// Creates a new, empty map
    pub fn new() -> Self
    {
        Self{ map: HashMap::new() }
    }

	/// Gets the tile at the given coordinates, if there is one.
	/// 
	/// If there is no tile at the given coordinates, [`None`] is returned.
    pub fn get(&self, coords: C) -> Option<&T>
    {
        self.map.get(&coords)
    }

	/// Gets the tile from the given coordinates mutably, allowing the retrieved tile to be modified
	/// with the changes saved in the map.
	/// 
	/// If no tile exists at the given coordinates, [`None`] is returned.
    pub fn get_mut(&mut self, coords: C) -> Option<&mut T>
    {
        self.map.get_mut(&coords)
    }

	/// Inserts a tile at the given coordinates
    pub fn insert(&mut self, coords: C, tile: T)
    {
        self.map.insert(coords, tile);
    }

	/// Inserts a hexagonal area of tiles into the map with the given radius, centered around the
	/// given tile coords.
    pub fn insert_area(&mut self, center: C, radius: usize, tile: T)
    where C: HexCoords, T: Clone
    {
        let area = C::area(center, radius);
        for coord in area
        {
            self.insert(coord, tile.clone());
        }
    }

    /// Finds a path from the `start` coords to the `destination` coords on this map, using
	/// Djikstra's algorithm with the provided cost function
    pub fn find_path<F>(&self, start: C, destination: C, cost_fn: F) -> Option<Vec<C>>
    where C: Copy + PartialEq, F: Fn(C, C, &HexMap<C, T>) -> f32
    {
        let mut pathfinder = PathMap::default().starting_from(start);
        while let Some(next_coords) = pathfinder.get_next_node()
        {
            if next_coords == destination {
                return Some(pathfinder.trace_path(destination));
            }
            pathfinder.eval_coords(next_coords, self, &cost_fn );
            pathfinder.set_coords_searched(next_coords);
        }
        None
    }

	/// Returns an iterator of all the Coord/Tile (Key/Value) pairs in this map
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

        #[derive(Clone)]
        enum PathTestTile
        {
            Cheap,
            Expensive,
        }

        fn cost_fn(_start: CubeCoords, end: CubeCoords, map: &HexMap<CubeCoords, PathTestTile>) -> f32
        {
            match map.get(end).unwrap()
            {
                PathTestTile::Cheap => 0.5,
                PathTestTile::Expensive => 2.0,
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
            let path = map.find_path(start, end, cost_fn).expect("Expected to return path, got None instead");
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
            let path = map.find_path(start, end, cost_fn).expect("Expected to return path, got None instead");
            assert_eq!(1, path.len());
            assert!(path.contains(&end));
            assert!(!path.contains(&start));
        }

        /// Ensures that a straight path is drawn between tiles when there is no
        /// pathfinding cost factor
        #[test]
        fn straight_path()
        {
            let mut map: HexMap<CubeCoords, PathTestTile> = HexMap::new();
            map.insert_area(CubeCoords::ZERO, 3, PathTestTile::Cheap);

            let path = map.find_path(cube!(0, 0, 0), cube!(1, 0, -1), cost_fn).unwrap();
            assert_eq!(1, path.len());
            assert_eq!(cube!(1, 0, -1), path[0]);

            let path = map.find_path(cube!(-1, 0, 1), cube!(1, 0, -1), cost_fn).unwrap();
            assert_eq!(2, path.len());
            assert_eq!(cube!(0, 0, 0), path[0]);
            assert_eq!(cube!(1, 0, -1), path[1]);

            let path = map.find_path(cube!(-1, 0, 1), cube!(2, 0, -2), cost_fn).unwrap();
            assert_eq!(3, path.len());
            assert_eq!(cube!(0, 0, 0), path[0]);
            assert_eq!(cube!(1, 0, -1), path[1]);
            assert_eq!(cube!(2, 0, -2), path[2]);

            let path = map.find_path(cube!(-2, 0, 2), cube!(2, 0, -2), cost_fn).unwrap();
            assert_eq!(4, path.len());
            assert_eq!(cube!(-1, 0, 1), path[0]);
            assert_eq!(cube!(0, 0, 0), path[1]);
            assert_eq!(cube!(1, 0, -1), path[2]);
            assert_eq!(cube!(2, 0, -2), path[3]);
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
            
            let path = map.find_path(start, end, cost_fn).expect("Expected to find path between start and end, but `None` was returned");
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
            let path = map.find_path(start, end, cost_fn);
            assert_eq!(None, path);
        }
    }
}