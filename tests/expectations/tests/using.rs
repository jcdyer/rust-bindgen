/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
pub type IntPoint2D = Point<::std::os::raw::c_int>;
pub type IntVec2D = Point<::std::os::raw::c_int>;
