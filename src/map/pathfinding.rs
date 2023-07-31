use std::{
    collections::{HashSet, HashMap},
    hash::Hash,
};

use crate::{HexCoords, HexMap};


/// Node used for pathfinding. The node graph of the [`PathMap`] struct uses this type for its nodes.
#[derive(Clone, Debug, PartialEq)]
pub struct PathNode<C>
{
    total_cost: f32,
    prev_coords: Option<C>
}

impl<C> Default for PathNode<C>
{
    fn default() -> Self {
        Self { total_cost: 0.0, prev_coords: None }
    }
}


/// Contains the data needed while calculating a path from a [`HexMap`](crate::HexMap)
/// 
/// Acts as a node graph of pathfinding nodes for the pathfinding algorithm, which for the moment
/// is [just Djikstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm)
#[derive(Clone, Debug)]
pub struct PathMap<C>
{
    /// Set of coordinates that still have yet to be searched and require evaluation
    /// 
    /// The `coords_to_search` set and `searched_coords` set are MUTUALLY EXCLUSIVE. A coordinate
    /// that's in one set should NOT be in the other.
    coords_to_search: HashSet<C>,

    /// Set of coordinates that have been searched and no longer need to be evaluated
    /// 
    /// The `coords_to_search` set and `searched_coords` set are MUTUALLY EXCLUSIVE. A coordinate
    /// that's in one set should NOT be in the other.
    searched_coords: HashSet<C>,

    /// Map of the actual pathfinding nodes, with their costs and references to their previous nodes
    nodes: HashMap<C, PathNode<C>>,
}

impl<C> PathMap<C>
where C: Clone + Copy + Eq + Hash + HexCoords
{
    /// Initializes the map with a single starting node to branch from
    pub fn starting_from(mut self, start_coords: C) -> Self
    {
        self.add_node(start_coords, PathNode::default());
        self
    }

    /// Adds a new pathfinding node to the pathmap, adding the coordinates of the new node to the
    /// `coords_to_search` set. If a node already exists at the given coordinates, it is overwritten.
    /// 
    /// Use this function ONLY if you know that the node at the given coordinates has NOT been
    /// searched yet. This could cause searched nodes to accidentally become unsearched during
    /// pathfinding, leading to an infinite loop. For example, this function is suitable for
    /// initializing an empty map with the starting or "seed" node at the beginning of pathfinding,
    /// or with nodes for unit testing.
    fn add_node(&mut self, coords: C, node: PathNode<C>)
    {
        self.coords_to_search.insert(coords);
        self.nodes.insert(coords, node);
    }

    /// Evaluates the given coordinates against its neighbors, updating any neighbors that can be
    /// reached from this coordinate for lower cost than their existing previous coords.
    pub fn eval_coords<F, T>(&mut self, source: C, map: &HexMap<C, T>, cost_fn: F)
    where F: Fn(C, C, &HexMap<C, T>) -> f32
    {
        let adjacent_coords = HexCoords::adjacent(source);
        let source_node = self.get_node(source).unwrap().clone();
        for neighbor_coord in adjacent_coords {
            if let Some(_neighbor_tile) = map.get(neighbor_coord) {
                let move_cost = source_node.total_cost + cost_fn(source, neighbor_coord, map);
                self.eval_move(source, neighbor_coord, move_cost);
            }
        }
    }

    /// Evaluates a single move from one tile to another. If the destination tile can be reached
    /// from the source tile for lower cost than its existing source tile, it will be updated to use
    /// the source node given here instead.
    fn eval_move(&mut self, source: C, dest: C, cost: f32)
    {
        if let Some(node) = self.nodes.get_mut(&dest) {
            if cost < node.total_cost {
                node.total_cost = cost;
                node.prev_coords = Some(source);
            }
        } else {
            let new_node = PathNode{ total_cost: cost, prev_coords: Some(source) };
            self.insert_node(dest, new_node);
        }
    }

    /// Traces a path to the given coordinates, so long as those coordinates have been given a path node
    pub fn trace_path(&self, dest: C) -> Vec<C>
    {
        let mut path = Vec::new();
        let mut next_coords = Some(dest);
        while let Some(c) = next_coords {
            let node = self.nodes.get(&c).unwrap();
            if node.prev_coords.is_some() {
                path.push(c);
            }
            next_coords = node.prev_coords;
        }
        path.reverse();
        path
    }

    /// Adds a new pathfinding node to the map if it does not exist. If it does exist, the existing
    /// node's total cost is compared with the new node's cost, and if the new node's is lower, the
    /// existing node is replaced.
    pub fn insert_node(&mut self, coords: C, new_node: PathNode<C>)
    {
        if let Some(existing_node) = self.get_node(coords)
        {
            if new_node.total_cost < existing_node.total_cost
            {
                self.nodes.insert(coords, new_node);
            }
        } else {
            self.add_node(coords, new_node);
        }
    }

    /// Retrieves the node at the given coordinates, if one exists. If there is no node at the
    /// coordinates, [`None`] is returned. 
    fn get_node(&self, coords: C) -> Option<&PathNode<C>>
    {
        self.nodes.get(&coords)
    }

    /// Returns the coordinates of the next node to be evaluated, chosen from the `coords_to_search`
    /// set.
    /// 
    /// If no nodes remain to be searched, this function returns `None`.
    pub fn get_next_node(&self) -> Option<C>
    {
        let mut best_coords = None;
        let mut lowest_cost = 0.0;

        for coords in self.coords_to_search.iter()
        {
            let node = self.nodes.get(coords).unwrap();
            if best_coords.is_none() || node.total_cost < lowest_cost
            {
                best_coords = Some(*coords);
                lowest_cost = node.total_cost;
            }
        }

        best_coords
    }

    /// Moves the given coords from the `coords_to_search` set to the `searched_coords` set
    pub fn set_coords_searched(&mut self, searched_coords: C)
    {
        self.coords_to_search.remove(&searched_coords);
        self.searched_coords.insert(searched_coords);
    }
}

impl<C> Default for PathMap<C>
{
    fn default() -> Self {
        Self{
            coords_to_search: HashSet::new(),
            searched_coords: HashSet::new(),
            nodes: HashMap::new(),
        }
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    use crate::{axial, AxialCoords};

    #[test]
    fn add_node()
    {
        let coords = axial!(1, 0);
        let mut map = PathMap::default();
        assert_eq!(false, map.coords_to_search.contains(&coords));
        assert_eq!(false, map.searched_coords.contains(&coords));
        assert_eq!(false, map.nodes.contains_key(&coords));

        let new_node = PathNode{
            total_cost: 1.0,
            prev_coords: Some(axial!(0, 0))
        };
        map.add_node(coords, new_node.clone());
        assert_eq!(true, map.coords_to_search.contains(&coords));
        assert_eq!(false, map.searched_coords.contains(&coords));
        assert_eq!(true, map.nodes.contains_key(&coords));
        assert_eq!(&new_node, map.get_node(coords).unwrap());
    }

    #[test]
    fn contains_node_at()
    {
        let mut map = PathMap::default();
        map.add_node(axial!(0, 0), PathNode{ total_cost: 0.0, prev_coords: None });
        assert!(map.get_node(axial!(0, 0)).is_some());
        assert!(map.get_node(axial!(1, 0)).is_none());
    }

    #[test]
    fn eval_move()
    {
        let mut pathmap = PathMap::default();
        let source_coords = axial!(1, 0);
        let dest_coords = axial!(0, 1);
        pathmap.add_node(dest_coords, PathNode{ total_cost: 2.0, prev_coords: Some(axial!(0, 0)) });

        // expect the map to be unchanged, as the cost of this move was greater than the existing one
        pathmap.eval_move(source_coords, dest_coords, 3.0);
        let node = pathmap.get_node(dest_coords).unwrap();
        assert_eq!(2.0, node.total_cost);
        assert_eq!(Some(axial!(0, 0)), node.prev_coords);

        // expect the map to be changed, as the cost of this move was less than than the existing one
        pathmap.eval_move(source_coords, dest_coords, 1.0);
        let node = pathmap.get_node(dest_coords).unwrap();
        assert_eq!(1.0, node.total_cost);
        assert_eq!(Some(source_coords), node.prev_coords);

        // expect that a new node is added if there isn't already one
        let new_coords = axial!(-1, 0);
        assert!(pathmap.get_node(new_coords).is_none());
        pathmap.eval_move(source_coords, new_coords, 2.0);
        let node = pathmap.get_node(new_coords).unwrap();
        assert_eq!(2.0, node.total_cost);
        assert_eq!(Some(source_coords), node.prev_coords);
    }

    #[test]
    fn eval_coords()
    {
        let mut pathmap = PathMap::default();
        let mut map = HexMap::new();
        pathmap.insert_node(axial!(0, 0), PathNode{total_cost: 0.0, prev_coords: None});
        map.insert(axial!(0, 0), ());
        pathmap.insert_node(axial!(1, 0), PathNode{ total_cost: 0.5, prev_coords: None });
        map.insert(axial!(1, 0), ());
        pathmap.insert_node(axial!(0, 1), PathNode{ total_cost: 3.0, prev_coords: Some(axial!(0, 0)) });
        map.insert(axial!(0, 1), ());

        pathmap.eval_coords(axial!(0, 0), &map, |_,_,_|{ 2.0 });
        assert_eq!(
            Some(&PathNode{ total_cost: 0.5, prev_coords: None }),
            pathmap.get_node(axial!(1, 0)),
        );
        assert_eq!(
            Some(&PathNode{ total_cost: 2.0, prev_coords: Some(axial!(0, 0)) }),
            pathmap.get_node(axial!(0, 1)),
        );

        pathmap.eval_coords(axial!(1, 0), &map, |_,_,_|{ 1.0 });
        assert_eq!(
            Some(&PathNode{ total_cost: 0.0, prev_coords: None }),
            pathmap.get_node(axial!(0, 0)),
        );
        assert_eq!(
            Some(&PathNode{ total_cost: 1.5, prev_coords: Some(axial!(1, 0)) }),
            pathmap.get_node(axial!(0, 1)),
        );
    }

    #[test]
    fn get_next_node()
    {
        let mut map = PathMap::default();
        assert_eq!(None, map.get_next_node());

        let searched_coords = axial!(1, -1);
        let searched_node = PathNode{ total_cost: 2.0, prev_coords: Some(axial!(0, 0)) };
        let unsearched_coords_cheap = axial!(2, -2);
        let unsearched_node_cheap = PathNode{ total_cost: 3.0, prev_coords: Some(searched_coords) };
        let unsearched_coords_expensive = axial!(3, -2);
        let unsearched_node_expensive = PathNode{ total_cost: 4.0, prev_coords: Some(searched_coords) };

        map.insert_node(searched_coords, searched_node);
        map.set_coords_searched(searched_coords);
        map.insert_node(unsearched_coords_cheap, unsearched_node_cheap);
        map.insert_node(unsearched_coords_expensive, unsearched_node_expensive);

        assert_eq!(Some(unsearched_coords_cheap), map.get_next_node());
        map.set_coords_searched(unsearched_coords_cheap);
        assert_eq!(Some(unsearched_coords_expensive), map.get_next_node());
        map.set_coords_searched(unsearched_coords_expensive);
        assert_eq!(None, map.get_next_node());
    }

    #[test]
    fn insert_node()
    {
        let coords = axial!(0, 0);
        let mut map = PathMap::default();
        assert_eq!(false, map.coords_to_search.contains(&coords));
        assert_eq!(false, map.searched_coords.contains(&coords));
        assert_eq!(false, map.nodes.contains_key(&coords));

        // Test adding a node to an empty map
        let node = PathNode{
            total_cost: 2.0,
            prev_coords: None,
        };
        map.insert_node(coords, node.clone());
        assert_eq!(true, map.coords_to_search.contains(&coords), "`map.coords_to_search` set did not contain the added node");
        assert_eq!(false, map.searched_coords.contains(&coords), "`map.searched_coords` set contains the added node when it should not");
        assert_eq!(true, map.nodes.contains_key(&coords), "`map.nodes` did not contain the added node");
        assert_eq!(&node, map.nodes.get(&coords).unwrap(), "Node at coordinates did not match the node added");

        // New node has greater cost than the existing node, so it does NOT replace the existing node
        let new_node = PathNode{
            total_cost: 3.0,
            prev_coords: Some(axial!(1, 0)),
        };
        map.insert_node(coords, new_node.clone());
        assert_eq!(true, map.coords_to_search.contains(&coords));
        assert_eq!(false, map.searched_coords.contains(&coords), "`map.searched_coords` set contains the added node when it should not");
        assert_eq!(true, map.nodes.contains_key(&coords));
        assert_eq!(&node, map.nodes.get(&coords).unwrap(), "Node was updated with new higher cost node when it should not have been");

        // New node has less cost than the existing node, so it DOES replace the existing node
        let new_node = PathNode{
            total_cost: 1.0,
            prev_coords: Some(axial!(1, 0)),
        };
        map.insert_node(coords, new_node.clone());
        assert_eq!(true, map.coords_to_search.contains(&coords));
        assert_eq!(false, map.searched_coords.contains(&coords), "`map.searched_coords` set contains the added node when it should not");
        assert_eq!(true, map.nodes.contains_key(&coords));
        assert_eq!(&new_node, map.nodes.get(&coords).unwrap(), "Existing node was not updated with the new lower cost node");
    }
}