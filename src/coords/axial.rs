use std::ops::{Add, Sub, Mul};
use crate::{CubeCoords, Orientation, HexCoords, axial};



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


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Axial coordinate system
/// 
/// Good for general use. Intuitive for humans, and cheaply converts to/from [`CubeCoords`] which are good for math and other operations
/// 
/// <https://www.redblobgames.com/grids/hexagons/#coordinates-axial>
pub struct AxialCoords
{
    pub q: isize,
    pub r: isize,
}


impl AxialCoords
{
    pub const ZERO: AxialCoords = AxialCoords{ q: 0, r: 0 };

    pub const Q: AxialCoords = AxialCoords{ q: 1, r: 0};

    pub const R: AxialCoords = AxialCoords{ q: 0, r: 1 };

    pub const S: AxialCoords = AxialCoords{ q: -1, r: 1 };

    pub fn new(q: isize, r: isize) -> Self
    {
        Self{ q, r }
    }

    pub fn distance(a: Self, b: Self) -> isize
    {
        CubeCoords::distance(CubeCoords::from(a), CubeCoords::from(b))
    }
}

impl HexCoords for AxialCoords
{
    fn line(a: Self, b: Self) -> Vec<Self> {
        let cube_line = CubeCoords::line(a.into(), b.into());
        cube_line.iter().map(|val|{AxialCoords::from(val)}).collect()
    }

    fn ring(center: Self, radius: usize) -> Vec<Self> {
        if radius == 0 {
            return vec![center];
        }
        let mut ring = Vec::new();
        for i in 0..radius {
            ring.push(center + AxialCoords::Q * radius + AxialCoords::S * i);
            ring.push(center + AxialCoords::R * radius - AxialCoords::Q * i);
            ring.push(center + AxialCoords::S * radius - AxialCoords::R * i);
            ring.push(center - AxialCoords::Q * radius - AxialCoords::S * i);
            ring.push(center - AxialCoords::R * radius + AxialCoords::Q * i);
            ring.push(center - AxialCoords::S * radius + AxialCoords::R * i);
        }
        ring
    }

    fn adjacent(center: Self) -> Vec<Self> {
        vec![
            center + axial!(0, -1),
            center + axial!(1, -1),
            center + axial!(1, 0),
            center + axial!(0, 1),
            center + axial!(-1, 1),
            center + axial!(-1, 0),
        ]
    }

    fn from_world(x: f32, y: f32, orientation: Orientation) -> Self {
        let sqrt_3 = 3.0f32.sqrt();
        match orientation
        {
            Orientation::PointyTop => {
                let q = sqrt_3 / 3.0 * x - 1.0 / 3.0 * y;
                let r = 2.0 / 3.0 * y;
                let s = -q - r;
                let cube = CubeCoords::round(q, r, s);
                AxialCoords::from(cube)
            },
            Orientation::FlatTop => todo!(),
        }
    }

    fn to_world(&self, orientation: Orientation) -> (f32, f32)
    {
        match orientation
        {
            Orientation::FlatTop => {
                todo!()
            },
            Orientation::PointyTop => {
                let x = self.q as f32 * orientation.tile_width() + self.r as f32 * orientation.tile_width() / 2.0;
                let y = self.r as f32 * orientation.tile_spacing_y();
                (x, y)
            },
        }
    }
}

// TRAITS: MATH OPERATIONS ---------------------------------------------------------------------- //

impl Add<Self> for AxialCoords
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{ q: self.q + rhs.q, r: self.r + rhs.r }
    }
}

impl Mul<usize> for AxialCoords
{
    type Output = AxialCoords;

    fn mul(self, rhs: usize) -> Self::Output
    {
        Self{ q: self.q * rhs as isize, r: self.r * rhs as isize }
    }
}

impl Sub<Self> for AxialCoords
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{ q: self.q - rhs.q, r: self.r - rhs.r }
    }
}

// TRAITS: CONVERSION --------------------------------------------------------------------------- //

impl From<CubeCoords> for AxialCoords
{
    /// Converts to [`AxialCoords`] from [`CubeCoords`]
    /// 
    /// <https://www.redblobgames.com/grids/hexagons/#conversions-axial>
    fn from(value: CubeCoords) -> Self {
        Self{ q: value.q, r: value.r }
    }
}

impl From<&CubeCoords> for AxialCoords
{
    /// Converts to [`AxialCoords`] from [`&CubeCoords`](CubeCoords)
    /// 
    /// <https://www.redblobgames.com/grids/hexagons/#conversions-axial>
    fn from(value: &CubeCoords) -> Self {
        Self{ q: value.q, r: value.r }
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

    mod ops
    {
        use super::*;

        #[test]
        fn add()
        {
            assert_eq!(axial!(0, 0), axial!(0, 0) + axial!(0, 0));
        }
    }

    mod traits
    {
        use super::*;

        mod hex_coords
        {
            use super::*;

            #[test]
            fn ring()
            {
                let ring = AxialCoords::ring(AxialCoords::ZERO, 0);
                assert_eq!(1, ring.len());
                assert!(ring.contains(&AxialCoords::ZERO));

                let ring = AxialCoords::ring(AxialCoords::ZERO, 1);
                assert_eq!(6, ring.len());
                assert!(!ring.contains(&AxialCoords::ZERO));
                assert!(ring.contains(&axial!(1, 0)));
                assert!(ring.contains(&axial!(0, 1)));
                assert!(ring.contains(&axial!(-1, 1)));
                assert!(ring.contains(&axial!(-1, 0)));
                assert!(ring.contains(&axial!(0, -1)));
                assert!(ring.contains(&axial!(1, -1)));

                let ring = AxialCoords::ring(AxialCoords::ZERO, 2);
                assert_eq!(12, ring.len());
                assert!(!ring.contains(&AxialCoords::ZERO));
                assert!(!ring.contains(&axial!(1, 0)));
                assert!(!ring.contains(&axial!(0, 1)));
                assert!(!ring.contains(&axial!(-1, 1)));
                assert!(!ring.contains(&axial!(-1, 0)));
                assert!(!ring.contains(&axial!(0, -1)));
                assert!(!ring.contains(&axial!(1, -1)));
                assert!(ring.contains(&axial!(2, -2)));
                assert!(ring.contains(&axial!(2, -1)));
                assert!(ring.contains(&axial!(2, 0)));

                let ring = AxialCoords::ring(axial!(1, -1), 0);
                assert_eq!(1, ring.len());
                assert!(ring.contains(&axial!(1, -1)));

                let ring = AxialCoords::ring(axial!(1, -1), 1);
                assert_eq!(6, ring.len());
                assert!(ring.contains(&axial!(0, 0)));
                assert!(ring.contains(&axial!(0, -1)));
                assert!(ring.contains(&axial!(1, -2)));
                assert!(ring.contains(&axial!(2, -2)));
                assert!(ring.contains(&axial!(2, -1)));
                assert!(ring.contains(&axial!(1, 0)));
            }
        }
    }
}