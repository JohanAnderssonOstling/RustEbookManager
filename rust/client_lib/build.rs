fn main(){
    cxx_build::bridge("src/lib.rs").compile("client_lib");
    cxx_build::bridge("src/library_model.rs").compile("library_model");
}