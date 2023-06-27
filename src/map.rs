use std::{collections::HashMap, hash::Hash};


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
}