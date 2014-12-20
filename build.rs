
extern crate gcc;

fn main() {
	let config = gcc::Config {
		include_directories: vec![Path::new("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.9.sdk/System/Library/Frameworks/JavaVM.framework/Versions/A/Headers")],
		definitions: Vec::new(),
		objects: Vec::new(),
		flags: Vec::new(),
	};

	gcc::compile_library("libinterface.a", &config, &["src/interface.c"]);
}
