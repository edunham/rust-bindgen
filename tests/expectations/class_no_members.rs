/* automatically generated by rust-bindgen */


#![feature(const_fn)]
#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_whatever;
impl ::std::clone::Clone for Struct_whatever {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_whatever_child {
    pub _base: Struct_whatever,
}
impl ::std::clone::Clone for Struct_whatever_child {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_whatever_child_with_member {
    pub _base: Struct_whatever,
    pub m_member: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for Struct_whatever_child_with_member {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_whatever_child_with_member() {
    assert_eq!(::std::mem::size_of::<Struct_whatever_child_with_member>() ,
               4usize);
    assert_eq!(::std::mem::align_of::<Struct_whatever_child_with_member>() ,
               4usize);
}
