#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        4usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        4usize,
        concat!("Alignment of ", stringify!(A))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A>()))._x as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(_x))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN1A13some_functionEv"]
    pub fn A_some_function(this: *mut A);
}
extern "C" {
    #[link_name = "\u{1}_ZN1A19some_other_functionEv"]
    pub fn A_some_other_function(this: *mut A);
}
extern "C" {
    #[link_name = "\u{1}_ZN1AC1Ei"]
    pub fn A_A(this: *mut A, x: ::std::os::raw::c_int);
}
impl A {
    #[inline]
    pub unsafe fn some_function(&mut self) {
        A_some_function(self)
    }
    #[inline]
    pub unsafe fn some_other_function(&mut self) {
        A_some_other_function(self)
    }
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        A_A(__bindgen_tmp.as_mut_ptr(), x);
        __bindgen_tmp.assume_init()
    }
}
extern crate libloading;
pub struct TestLib {
    __library: ::libloading::Library,
    foo: Result<
        unsafe extern "C" fn(
            x: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    bar: Result<unsafe extern "C" fn(), ::libloading::Error>,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let foo = __library.get("foo".as_bytes()).map(|sym| *sym);
        let bar = __library.get("bar".as_bytes()).map(|sym| *sym);
        Ok(TestLib {
            __library: __library,
            foo,
            bar,
        })
    }
    pub fn can_call(&self) -> CheckTestLib {
        CheckTestLib { __library: self }
    }
    pub unsafe fn foo(
        &self,
        x: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        let sym = self.foo.as_ref().expect("Expected function, got error.");
        (sym)(x)
    }
    pub unsafe fn bar(&self) -> () {
        let sym = self.bar.as_ref().expect("Expected function, got error.");
        (sym)()
    }
}
pub struct CheckTestLib<'a> {
    __library: &'a TestLib,
}
impl<'a> CheckTestLib<'a> {
    pub fn foo(&self) -> Result<(), &'a ::libloading::Error> {
        self.__library.foo.as_ref().map(|_| ())
    }
    pub fn bar(&self) -> Result<(), &'a ::libloading::Error> {
        self.__library.bar.as_ref().map(|_| ())
    }
}
