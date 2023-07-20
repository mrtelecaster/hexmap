use std::ops::{Add, Sub};
use crate::{CubeCoords, HexOrientation, constants::*, HexCoords};



#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Axial coordinate system
/// 
/// Good for general use. Intuitive for humans, and cheaply converts to/from [`CubeCoords`] which are good for math and other operations
pub struct AxialCoords
{
    pub q: isize,
    pub r: isize,
}

impl AxialCoords
{
    pub fn new(q: isize, r: isize) -> Self
    {
        Self{ q, r }
    }

    pub fn distance(a: Self, b: Self) -> isize
    {
        CubeCoords::distance(CubeCoords::from(a), CubeCoords::from(b))
    }

    pub fn from_world(x: f32, y: f32, orientation: HexOrientation) -> Self {
        let sqrt_3 = 3.0f32.sqrt();
        match orientation
        {
            HexOrientation::PointyTop => {
                let q = sqrt_3 / 3.0 * x - 1.0 / 3.0 * y;
                let r = 2.0 / 3.0 * y;
                let s = -q - r;
                let cube = CubeCoords::round(q, r, s);
                AxialCoords::from(cube)
            },
            HexOrientation::FlatTop => todo!(),
        }
    }

    pub fn to_world(&self, orientation: HexOrientation) -> (f32, f32)
    {
        match orientation
        {
            HexOrientation::FlatTop => {
                todo!()
            },
            HexOrientation::PointyTop => {
                let x = self.q as f32 * POINTY_TOP_WIDTH + self.r as f32 * POINTY_TOP_WIDTH / 2.0;
                let y = self.r as f32 * POINTY_TOP_VERTICAL_SPACING;
                (x, y)
            },
        }
    }

    pub fn corners(&self, orientation: HexOrientation) -> [(f32, f32);6]
    {
        match orientation
        {
            HexOrientation::FlatTop => {
                todo!()
            },
            HexOrientation::PointyTop => {
                todo!()
            },
        }
    }
}

impl HexCoords for AxialCoords
{
    fn line(a: Self, b: Self) -> Vec<Self> {
        let cube_coord_line: Vec<CubeCoords> = CubeCoords::line(a.into(), b.into());
        let axial_coord_line: Vec<AxialCoords> = cube_coord_line.iter()
            .map(|val| {AxialCoords::from(val)})
            .collect();
        axial_coord_line
    }

    fn ring(center: Self, radius: usize) -> Vec<Self> where Self: Sized {
        let cube_coord_ring: Vec<CubeCoords> = CubeCoords::ring(center.into(), radius);
        let axial_coord_ring: Vec<AxialCoords> = cube_coord_ring.iter()
            .map(|val|{AxialCoords::from(val)})
            .collect();
        axial_coord_ring
    }

    fn area(center: Self, radius: usize) -> Vec<Self> where Self: Sized {
        let cube_coord_area: Vec<CubeCoords> = CubeCoords::area(center.into(), radius);
        let axial_coord_area: Vec<AxialCoords> = cube_coord_area.iter()
            .map(|val|{AxialCoords::from(val)})
            .collect();
        axial_coord_area
    }
}

impl Add<Self> for AxialCoords
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{ q: self.q + rhs.q, r: self.r + rhs.r }
    }
}

impl From<CubeCoords> for AxialCoords
{
    fn from(value: CubeCoords) -> Self {
        Self{ q: value.q, r: value.r }
    }
}

impl From<&CubeCoords> for AxialCoords
{
    fn from(value: &CubeCoords) -> Self {
        Self{ q: value.q, r: value.r }
    }
}

impl Sub<Self> for AxialCoords
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{ q: self.q - rhs.q, r: self.r - rhs.r }
    }
}



#[cfg(test)]
mod tests
{
    use super::*;
    use crate::{axial, cube};

    #[test]
    fn add()
    {
        assert_eq!(axial!(0, 0), axial!(0, 0) + axial!(0, 0));
        assert_eq!(axial!(-1, 0), axial!(1, -1) + axial!(-2, 1));
        assert_eq!(axial!(2, -3), axial!(1, -1) + axial!(1, -2));
    }

    #[test]
    fn distance()
    {
        assert_eq!(0, AxialCoords::distance(axial!(0, 0), axial!(0, 0)));
        assert_eq!(1, AxialCoords::distance(axial!(1, -1), axial!(0, 0)));
        assert_eq!(1, AxialCoords::distance(axial!(1, 0), axial!(0, 0)));
        assert_eq!(2, AxialCoords::distance(axial!(1, 1), axial!(0, 0)));
        assert_eq!(2, AxialCoords::distance(axial!(-1, -1), axial!(0, 0)));
        assert_eq!(1, AxialCoords::distance(axial!(-1, 0), axial!(0, 0)));
        assert_eq!(1, AxialCoords::distance(axial!(-1, 1), axial!(0, 0)));
    }

    #[test]
    fn from_cube()
    {
        assert_eq!(axial!(0, 0), AxialCoords::from(cube!(0, 0, 0)));

        assert_eq!(axial!(1, 0), AxialCoords::from(cube!(1, 0, -1)));
        assert_eq!(axial!(0, 1), AxialCoords::from(cube!(0, 1, -1)));
        assert_eq!(axial!(-1, 1), AxialCoords::from(cube!(-1, 1, 0)));
        assert_eq!(axial!(-1, 0), AxialCoords::from(cube!(-1, 0, 1)));
        assert_eq!(axial!(0, -1), AxialCoords::from(cube!(0, -1, 1)));
        assert_eq!(axial!(1, -1), AxialCoords::from(cube!(1, -1, 0)));
    }
}