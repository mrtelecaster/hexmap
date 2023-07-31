/// Shortcut for [`AxialCoords::new`](crate::AxialCoords::new). Creates a new set of
/// [axial coordinates](crate::AxialCoords) with the provided values.
/// 
/// ```
/// use hexmap::{AxialCoords, axial};
/// assert_eq!(AxialCoords::new(1, 2), axial!(1, 2));
/// ```
#[macro_export]
macro_rules! axial {
    ($q:literal, $r:literal) => { AxialCoords{q: $q, r: $r} }
}

/// Creates a new set of [`CubeCoords`](crate::CubeCoords) with the provided values. Acts as a
/// shortcut for [`CubeCoords::new`](crate::CubeCoords::new)
/// 
/// ```
/// use hexmap::{CubeCoords, cube};
/// assert_eq!(CubeCoords::new(1, -4, 3), cube!(1, -4, 3));
/// ```
#[macro_export]
macro_rules! cube {
    ($q:literal, $r:literal, $s:literal) => { CubeCoords::new($q, $r, $s) }
}