use std::{ops::{Add, Mul, Neg, Sub}, fmt::Display};
use lerp::Lerp;
use crate::{HexCoords, cube};

use super::AxialCoords;


/// Cube coordinates
/// 
/// Good for math, but can be annoying to work with from a human perspective as well as having an "unnecessary" third coordinate compared to [`AxialCoords`]
/// 
/// <https://www.redblobgames.com/grids/hexagons/#coordinates-cube>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CubeCoords
{
    pub q: isize,
    pub r: isize,
    pub s: isize,
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

impl CubeCoords
{
    // Constants ------------------------------------------------------------ //

    pub const ZERO: Self = Self{ q: 0, r: 0, s: 0 };

    // Constructors --------------------------------------------------------- //

    pub fn new(q: isize, r: isize, s: isize) -> Self
    {
        let coords = Self{ q, r, s };
        if !coords.is_valid()
        {
            panic!("Sum of coordinates must equal 0. {}+{}+{}!=0", coords.q, coords.r, coords.s);
        }
        coords
    }

    pub fn round(q: f32, r: f32, s: f32) -> Self
    {
        let mut output = Self{ q: q.round() as isize, r: r.round() as isize, s: s.round() as isize };
        // Sometimes straight rounding doesn't produce valid coordinates. Correct them if they are invalid
        if !output.is_valid() {
            // Compute difference between the rounded output of each coordinate and the original input
            let diff_q: f32 = (q - output.q as f32).abs();
            let diff_r: f32 = (r - output.r as f32).abs();
            let diff_s: f32 = (s - output.s as f32).abs();
            // Recompute the coordinate with the greatest difference
            if diff_q > diff_r && diff_q > diff_s {
                output.q = -output.r - output.s;
            } else if diff_r > diff_s {
                output.r = -output.q - output.s;
            } else {
                output.s = -output.q - output.r;
            }
            // If coordinates are still invalid, panic
            if !output.is_valid()
            {
                panic!(
                    "Unable to round fractional coordinates ({}, {}, {}) to valid cube coords. Computed output coords were {}, which are invalid",
                    q, r, s, output
                );
            }
        }
        output
    }

    // Set generators ------------------------------------------------------- //
    // Functions that generate sets of coordinates representing common shapes

    /// Generates a contiguous line of coordinates from `(0, 0, 0)` to the argument `end`.
    /// 
    /// The resulting vector includes the `(0, 0, 0)` coord as the first element in
    /// the array, and `end` as the last element in the array, with all interim points
    /// adjacent and in order between `(0, 0, 0)` and `end`.
    pub fn line_from_center(end: Self) -> Vec<Self>
    {
        Self::line(Self::new(0, 0, 0), end)
    }

    // Static methods ------------------------------------------------------- //

    pub fn distance(a: Self, b: Self) -> isize {
        let vec: Self = a - b;
        (vec.q.abs() + vec.r.abs() + vec.s.abs()) / 2
    }

    // Instance methods ----------------------------------------------------- //

    pub fn is_valid(&self) -> bool
    {
        self.q + self.r + self.s == 0
    }
}

impl HexCoords for CubeCoords
{
    fn line(a: Self, b: Self) -> Vec<Self> {
        let tiles = Self::distance(a, b)+1;
        let mut output = Vec::default();
        for i in 0..tiles
        {
            let t = i as f32 / (tiles-1) as f32;
            output.push(a.lerp(b, t));
        }
        output
    }

    fn ring(center: Self, radius: usize) -> Vec<Self> where Self: Sized {
        if radius == 0 {
            return vec![center];
        }
        let mut output = Vec::new();
        let corner_q = center + cube!(0, -1, 1) * radius;
        let corner_r = center + cube!(1, 0, -1) * radius;
        let corner_s = center + cube!(-1, 1, 0) * radius;
        for i in 0..radius
        {
            output.push(corner_q + cube!(1, 0, -1) * i);
            output.push(-corner_q - cube!(1, 0, -1) * i);
            output.push(corner_r + cube!(-1, 1, 0) * i);
            output.push(-corner_r - cube!(-1, 1, 0) * i);
            output.push(corner_s + cube!(0, -1, 1) * i);
            output.push(-corner_s - cube!(0, -1, 1) * i);
        }
        output
    }

    fn area(center: Self, radius: usize) -> Vec<Self> where Self: Sized {
        let mut output = Vec::new();
        for i in 0..radius
        {
            let mut ring = CubeCoords::ring(center, i);
            output.append(&mut ring);
        }
        output
    }

    fn adjacent(center: Self) -> Vec<Self> {
        vec![
            center + cube!(0, -1, 1),
            center + cube!(1, -1, 0),
            center + cube!(1, 0, -1),
            center + cube!(0, 1, -1),
            center + cube!(-1, 1, 0),
            center + cube!(-1, 0, 1),
        ]
    }
}

impl Add<CubeCoords> for CubeCoords
{
    type Output = CubeCoords;

    fn add(self, rhs: CubeCoords) -> Self::Output {
        CubeCoords::from(AxialCoords::from(self) + AxialCoords::from(rhs))
    }
}

impl Add<&CubeCoords> for CubeCoords
{
    type Output = CubeCoords;

    fn add(self, rhs: &CubeCoords) -> Self::Output {
        self + *rhs
    }
}

impl Add<CubeCoords> for &CubeCoords
{
    type Output = CubeCoords;

    fn add(self, rhs: CubeCoords) -> Self::Output {
        CubeCoords::new(self.q + rhs.q, self.r + rhs.r, self.s + rhs.s)
    }
}

impl Add<&CubeCoords> for &CubeCoords
{
    type Output = CubeCoords;

    fn add(self, rhs: &CubeCoords) -> Self::Output {
        CubeCoords::new(self.q + rhs.q, self.r + rhs.r, self.s + rhs.s)
    }
}

impl Display for CubeCoords
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.q, self.r, self.s)
    }
}

impl From<[f32;3]> for CubeCoords
{
    fn from(value: [f32;3]) -> Self {
        Self::new(value[0].round() as isize, value[1].round() as isize, value[2].round() as isize)
    }
}

impl From<AxialCoords> for CubeCoords
{
    fn from(value: AxialCoords) -> Self {
        Self{ q: value.q, r: value.r, s: -value.q - value.r }
    }
}

impl Lerp<f32> for CubeCoords
{
    fn lerp(self, other: Self, t: f32) -> Self {
        let q = (self.q as f32).lerp(other.q as f32, t);
        let r = (self.r as f32).lerp(other.r as f32, t);
        let s: f32 = (self.s as f32).lerp(other.s as f32, t);
        Self::round(q, r, s)
    }
}

impl Mul<Self> for CubeCoords
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.q * rhs.q, self.r * rhs.r, self.s * rhs.s)
    }
}

impl Mul<isize> for CubeCoords
{
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.q * rhs, self.r * rhs, self.s * rhs)
    }
}

impl Mul<usize> for CubeCoords
{
    type Output = CubeCoords;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs as isize
    }
}

impl Neg for CubeCoords
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.q, -self.r, -self.s)
    }
}

impl Sub<Self> for CubeCoords
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        CubeCoords::from(AxialCoords::from(self) - AxialCoords::from(rhs))
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::{axial, cube};

    #[test]
    #[ignore]
    fn area()
    {
        let center = cube!(1, 0, -1);
        let area = CubeCoords::area(center, 0);
        assert_eq!(1, area.len());
        assert!(area.contains(&center));

        let area = CubeCoords::area(center, 1);
        assert_eq!(7, area.len());
        assert!(area.contains(&center));
        assert!(area.contains(&cube!(2, -1, -1)));

        let area = CubeCoords::area(center, 2);
        assert_eq!(19, area.len());
        assert!(area.contains(&center));
        assert!(area.contains(&cube!(2, -1, -1)));
        assert!(area.contains(&cube!(-1, 0, 1)));
    }

    #[test]
    #[ignore]
    fn ring()
    {
        let ring = CubeCoords::ring(CubeCoords::ZERO, 0);
        assert_eq!(1, ring.len());
        assert!(ring.contains(&CubeCoords::ZERO));

        let ring = CubeCoords::ring(CubeCoords::ZERO, 1);
        assert_eq!(6, ring.len());
        assert!(ring.contains(&cube!(0, -1, 1)));
        assert!(ring.contains(&cube!(1, -1, 0)));
        assert!(ring.contains(&cube!(1, 0, -1)));
        assert!(ring.contains(&cube!(0, 1, -1)));
        assert!(ring.contains(&cube!(-1, 1, 0)));
        assert!(ring.contains(&cube!(-1, 0, 1)));

        let ring = CubeCoords::ring(CubeCoords::ZERO, 2);
        assert_eq!(12, ring.len());
        assert!(ring.contains(&cube!(0, -2, 2)));
        assert!(ring.contains(&cube!(2, -2, 0)));
        assert!(ring.contains(&cube!(2, 0, -2)));
        assert!(ring.contains(&cube!(0, 2, -2)));
        assert!(ring.contains(&cube!(-2, 2, 0)));
        assert!(ring.contains(&cube!(-2, 0, 2)));
        assert!(ring.contains(&cube!(1, -2, 1)));
        assert!(ring.contains(&cube!(2, -1, -1)));
        assert!(ring.contains(&cube!(1, 1, -2)));
        assert!(ring.contains(&cube!(-1, 2, -1)));
        assert!(ring.contains(&cube!(-2, 1, 1)));
        assert!(ring.contains(&cube!(-1, -1, 2)));

        let center = cube!(1, -1, 0);
        let ring = CubeCoords::ring(center, 0);
        assert_eq!(1, ring.len());
        assert!(ring.contains(&center));

        let ring = CubeCoords::ring(center, 1);
        assert_eq!(6, ring.len());
        assert!(!ring.contains(&center));
        assert!(ring.contains(&cube!(0, 0, 0)));
        assert!(ring.contains(&cube!(1, -2, 1)));
        assert!(ring.contains(&cube!(2, -1, -1)));
        assert!(ring.contains(&cube!(1, 0, -1)), "Ring: {:?}", ring);
        assert!(ring.contains(&cube!(0, -1, 1)));
        assert!(ring.contains(&cube!(2, -2, 0)));
    }

    #[test]
    #[ignore]
    fn distance()
    {
        assert_eq!(0, CubeCoords::distance(cube!(0, 0, 0), cube!(0, 0, 0)));

        assert_eq!(1, CubeCoords::distance(cube!(0, 0, 0), cube!(0, 1, -1)));
        assert_eq!(1, CubeCoords::distance(cube!(0, 0, 0), cube!(1, -1, 0)));
        assert_eq!(1, CubeCoords::distance(cube!(0, 0, 0), cube!(1, 0, -1)));
        assert_eq!(1, CubeCoords::distance(cube!(0, 0, 0), cube!(0, 1, -1)));
        assert_eq!(1, CubeCoords::distance(cube!(0, 0, 0), cube!(-1, 1, 0)));
        assert_eq!(1, CubeCoords::distance(cube!(0, 0, 0), cube!(-1, 0, 1)));

        assert_eq!(2, CubeCoords::distance(cube!(0, 0, 0), cube!(0, -2, 2)));
        assert_eq!(2, CubeCoords::distance(cube!(0, 0, 0), cube!(1, -2, 1)));
        assert_eq!(2, CubeCoords::distance(cube!(0, 0, 0), cube!(2, -2, 0)));
        assert_eq!(2, CubeCoords::distance(cube!(-1, 1, 0), cube!(0, -1, 1)));
    }

    #[test]
    fn from_axial()
    {
        assert_eq!(cube!(0, 0, 0), CubeCoords::from(axial!(0, 0)));

        assert_eq!(cube!(1, 0, -1), CubeCoords::from(axial!(1, 0)));
        assert_eq!(cube!(0, 1, -1), CubeCoords::from(axial!(0, 1)));
        assert_eq!(cube!(-1, 1, 0), CubeCoords::from(axial!(-1, 1)));
        assert_eq!(cube!(-1, 0, 1), CubeCoords::from(axial!(-1, 0)));
        assert_eq!(cube!(0, -1, 1), CubeCoords::from(axial!(0, -1)));
        assert_eq!(cube!(1, -1, 0), CubeCoords::from(axial!(1, -1)));
    }

    #[test]
    fn is_valid()
    {
        // valid coords
        assert!(CubeCoords{ q: 0, r: 0, s: 0 }.is_valid());
        assert!(CubeCoords{ q: 0, r: 1, s: -1 }.is_valid());
        assert!(CubeCoords{ q: 1, r: -1, s: 0 }.is_valid());
        assert!(CubeCoords{ q: 1, r: 0, s: -1 }.is_valid());
        assert!(CubeCoords{ q: 0, r: 1, s: -1 }.is_valid());
        assert!(CubeCoords{ q: -1, r: 1, s: 0 }.is_valid());
        assert!(CubeCoords{ q: -1, r: 0, s: 1 }.is_valid());
        // invalid coords
        assert!(!CubeCoords{ q: 1, r: 0, s: 0 }.is_valid());
        assert!(!CubeCoords{ q: -1, r: 0, s: 0 }.is_valid());
        assert!(!CubeCoords{ q: 0, r: 1, s: 0 }.is_valid());
        assert!(!CubeCoords{ q: 0, r: -1, s: 0 }.is_valid());
        assert!(!CubeCoords{ q: 0, r: 0, s: 1 }.is_valid());
        assert!(!CubeCoords{ q: 0, r: 0, s: -1 }.is_valid());
    }

    #[test]
    fn lerp()
    {
        let start = cube!(0, 1, -1);
        let end = cube!(1, -1, 0);
        let result = start.lerp(end, 0.0);
        assert!(result == start);
        let result = start.lerp(end, 0.5);
        assert!(result == cube!(0, 0, 0) || result == cube!(1, 0, -1));
        let result = start.lerp(end, 1.0);
        assert!(result == end);

        let start = cube!(0, -1, 1);
        let end = cube!(1, 1, -2);
        let result = start.lerp(end, 0.0);
        assert!(result == start);
        let result = start.lerp(end, 0.333);
        assert!(result == cube!(0, 0, 0), "Expected (0, 0, 0), but result was {}", result);
        let result = start.lerp(end, 0.667);
        assert!(result == cube!(1, 0, -1));
        let result = start.lerp(end, 1.0);
        assert!(result == end);

        let start = cube!(0, -1, 1);
        let end = cube!(2, 0, -2);
        let result = start.lerp(end, 0.0);
        assert!(result == start);
        let result = start.lerp(end, 0.333);
        assert!(result == cube!(1, -1, 0), "Expected (0, 0, 0), but result was {}", result);
        let result = start.lerp(end, 0.667);
        assert!(result == cube!(1, 0, -1));
        let result = start.lerp(end, 1.0);
        assert!(result == end);
    }


    #[test]
    fn line()
    {
        let start = cube!(-1, -1, 2);
        let end = cube!(2, -1, -1);
        let line = CubeCoords::line(start, end);
        assert_eq!(4, line.len());
        assert_eq!(cube!(-1, -1, 2), line[0]);
        assert_eq!(cube!(0, -1, 1), line[1]);
        assert_eq!(cube!(1, -1, 0), line[2]);
        assert_eq!(cube!(2, -1, -1), line[3]);

        let start = cube!(-1, 0, 1);
        let end = cube!(2, -1, -1);
        let line = CubeCoords::line(start, end);
        assert_eq!(4, line.len());
        assert_eq!(cube!(-1, 0, 1), line[0]);
        assert_eq!(cube!(0, 0, 0), line[1]);
        assert_eq!(cube!(1, -1, 0), line[2]);
        assert_eq!(cube!(2, -1, -1), line[3]);
    }

    mod ops
    {
        use super::*;

        #[test]
        #[ignore]
        fn add()
        {
            assert_eq!(cube!(0, 0, 0), cube!(0, 0, 0) + cube!(0, 0, 0));
            assert_eq!(cube!(3, 0, -3), cube!(1, 1, -2) + cube!(2, -1, -1));
            assert_eq!(cube!(16, -3, -9), cube!(1, 2, -3) + cube!(11, -5, -6));
        }
    }
}