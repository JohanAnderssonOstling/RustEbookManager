pub mod library_model_cxx;
pub mod home_model_cxx;
#[cxx::bridge]
pub mod ffi {
    pub struct test{
        pub(crate) a: u32,
        pub(crate) b: u32,
    }
}