fn main() {
	std::env::set_var("CXXFLAGS", "-std=c++11");
    cc::Build::new()
		.cpp(true)
		.cpp_link_stdlib("stdc++")
        .file("src/cpp/graph.cpp")
        .file("src/cpp/GraphGridLayout.cpp")
        .include("src/cpp")
        .compile("libgraph.a");
  println!("cargo:rustc-flags=-l dylib=c++");
}
