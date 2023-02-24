fn main(){
    cxx_build::bridge("src/home_model_cxx.rs").compile("client_lib");
    cxx_build::bridge("src/library_model_cxx.rs").compile("library_model");
	cxx_build::bridge("src/lib.rs").compile("lib");
}
