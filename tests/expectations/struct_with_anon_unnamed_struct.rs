/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_foo {
    pub foo_struct_with_anon_unnamed_struct_h_unnamed_1: Struct_foo_struct_with_anon_unnamed_struct_h_unnamed_1,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Struct_foo_struct_with_anon_unnamed_struct_h_unnamed_1 {
    pub a: ::std::os::raw::c_uint,
    pub b: ::std::os::raw::c_uint,
}
impl ::std::clone::Clone for
 Struct_foo_struct_with_anon_unnamed_struct_h_unnamed_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_foo_struct_with_anon_unnamed_struct_h_unnamed_1() {
    assert_eq!(::std::mem::size_of::<Struct_foo_struct_with_anon_unnamed_struct_h_unnamed_1>()
               , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_foo_struct_with_anon_unnamed_struct_h_unnamed_1>()
               , 4usize);
}
impl ::std::clone::Clone for Struct_foo {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_Struct_foo() {
    assert_eq!(::std::mem::size_of::<Struct_foo>() , 8usize);
    assert_eq!(::std::mem::align_of::<Struct_foo>() , 4usize);
}