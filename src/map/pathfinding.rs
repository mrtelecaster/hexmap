use std::{collections::{HashMap, HashSet}, hash::Hash};

use crate::{HexCoords, HexMap};


pub trait PathfindingTile
{
    fn pathfind_cost(&self) -> f32 {
        0.05
    }
}



/// A node in the pathfinding algorithm
struct PathNode<C>
{
    total_cost: f32,
    prev_node: Option<C>,
}


pub struct Pathfinder<C>
{
    coords_to_search: HashSet<C>,
    searched_coords: HashSet<C>,
    path_nodes: HashMap<C, PathNode<C>>,
}

impl<C> Pathfinder<C>
where C: Clone + Copy + Eq + Hash + HexCoords
{
    pub fn find_path<T>(&mut self, start: C, dest: C, map: &HexMap<C, T>) -> Option<Vec<C>>
    where T: PathfindingTile
    {
        if start == dest { return Some(Vec::new()); }
        self.coords_to_search.clear();
        self.searched_coords.clear();
        self.path_nodes.clear();
        self.insert(start, PathNode{ total_cost: 0.0, prev_node: None });
        while let Some(test_coords) = self.get_next_coords()
        {
            if test_coords == dest {
                return Some(self.unroll_path(test_coords));
            }
            self.search_node(test_coords, map);
        }
        None
    }

    /// Gets the next coordinates to search at the beginning of a new loop iteration.
    /// 
    /// Only a coordinate in the [`coords_to_search`] set can be returned. The specific coordinate
    /// returned will correspond to whichever `path_node` has the lowest total cost so far.
    fn get_next_coords(&mut self) -> Option<C>
    {
        let mut best_coords = None;
        let mut best_cost = 0.0;
        for coords in self.coords_to_search.iter()
        {
            let node = self.get(&coords).unwrap();
            if best_coords.is_none() || node.total_cost < best_cost
            {
                best_coords = Some(coords.clone());
                best_cost = node.total_cost;
            }
        }
        if let Some(c) = best_coords
        {
            self.coords_to_search.remove(&c);
        }
        best_coords
    }

    /// Inserts a new node at the given coordinates. Also adds the coords to the
    /// `coords_to_search` set.
    fn insert(&mut self, coord: C, node: PathNode<C>)
    {
        self.path_nodes.insert(coord, node);
        self.coords_to_search.insert(coord);
    }

    fn get(&self, coord: &C) -> Option<&PathNode<C>>
    {
        self.path_nodes.get(&coord)
    }

    /// Searches the path node at the given coordinates. This will search all nodes adjacent to this
    /// one, and, if they can be reached for a lower cost from this node than whatever their
    /// previous node is, they will be updated to now have this as their previous node.
    /// 
    /// Once the pathfinding logic is complete, the coords will be removed from the
    /// `coords_to_search` set and added to the `searched_coords` set
    fn search_node<T>(&mut self, coord: C, map: &HexMap<C, T>)
    where C: HexCoords, T: PathfindingTile
    {
        let source_cost = self.path_nodes.get(&coord).unwrap().total_cost;
        for adjacent_coord in C::adjacent(coord)
        {
            if let Some(dest_tile) = map.get(adjacent_coord)
            {
                let new_cost = source_cost + dest_tile.pathfind_cost();
                self.test_node(adjacent_coord, coord, new_cost);
            }
        }
        self.searched_coords.insert(coord);
    }

    /// Tests a single node to see if it can be reached from the source node provided by this
    /// function for a lower cost than whatever its previous node is.
    fn test_node(&mut self, test_coords: C, source_coords: C, cost_from_source: f32)
    {
        if let Some(mut dest_node) = self.path_nodes.get_mut(&test_coords)
        {
            if cost_from_source < dest_node.total_cost
            {
                dest_node.total_cost = cost_from_source;
                dest_node.prev_node = Some(source_coords);
            }
        } else {
            self.coords_to_search.insert(test_coords);
            self.path_nodes.insert(test_coords, PathNode{ total_cost: cost_from_source, prev_node: Some(source_coords) });
        }
    }

    fn unroll_path(&self, unroll_from: C) -> Vec<C>
    {
        let mut path = Vec::new();
        let mut next_coords = Some(unroll_from);
        while let Some(c) = next_coords
        {
            
            let node = self.path_nodes.get(&c).unwrap();
            if node.prev_node.is_some()
            {
                path.push(c);
            }
            next_coords = node.prev_node;
        }
        path.reverse();
        path
    }
}

impl<C> Default for Pathfinder<C>
where C: Clone + Copy + Eq + Hash
{
    fn default() -> Self {
        Pathfinder{
            coords_to_search: HashSet::new(),
            searched_coords: HashSet::new(),
            path_nodes: HashMap::new(),
        }
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    use crate::{axial, AxialCoords, HexMap};

    pub struct TestTile;

    impl PathfindingTile for TestTile {}

    /// Ensures that the function returns the lowest cost coords, and removes
    /// them from the `coords_to_search` set.
    /// 
    /// Tests that the returned coords are the lowest cost UNSEARCHED
    /// coordinates so far, ensuring that a higher cost unsearched coordinate or
    /// a lower cost but already searched coordinate is not returned.
    #[test]
    fn get_next_coords()
    {
        let mut pathfinder: Pathfinder<AxialCoords> = Pathfinder::default();
        pathfinder.insert(axial!(0, 1), PathNode{ total_cost: 2.0, prev_node: None });
        pathfinder.insert(axial!(2, 0), PathNode{ total_cost: 1.0, prev_node: Some(axial!(0, 0)) });
        pathfinder.insert(axial!(0, 0), PathNode{ total_cost: 0.0, prev_node: None });
        pathfinder.coords_to_search.remove(&axial!(0, 0));
        pathfinder.searched_coords.insert(axial!(0, 0));
        assert_eq!(Some(axial!(2, 0)), pathfinder.get_next_coords());
        assert_eq!(Some(axial!(0, 1)), pathfinder.get_next_coords());
        assert_eq!(None, pathfinder.get_next_coords());
    }

    #[test]
    #[ignore]
    fn search_node()
    {
        todo!()
    }

    #[test]
    fn test_node()
    {
        let mut pathfinder: Pathfinder<AxialCoords> = Pathfinder::default();
        let mut map: HexMap<AxialCoords, TestTile> = HexMap::new();
        pathfinder.insert(axial!(0, 0), PathNode{ total_cost: 0.0, prev_node: None });
        map.insert(axial!(0, 0), TestTile);
        pathfinder.insert(axial!(1, 0), PathNode{ total_cost: 1.0, prev_node: None });
        map.insert(axial!(1, 0), TestTile);
        pathfinder.test_node(axial!(0, 0), axial!(-1, 0), 0.5);
        pathfinder.test_node(axial!(1, 0), axial!(-1, 0), 0.5);
        let unchanged_node = pathfinder.get(&axial!(0, 0)).unwrap();
        assert_eq!(None, unchanged_node.prev_node);
        assert_eq!(0.0, unchanged_node.total_cost);
        let changed_node = pathfinder.get(&axial!(1, 0)).unwrap();
        assert_eq!(Some(axial!(-1, 0)), changed_node.prev_node);
        assert_eq!(0.5, changed_node.total_cost);
    }
}