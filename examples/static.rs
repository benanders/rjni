
//
//  Static
//  An example that calls a static method on a class.
//

extern crate jni;

use jni::{JavaVM, Type, Value};
use std::os;


fn main() {
	// Find the path to this program's executable file, and look for the
	// Test.class file in there.
	let executable_path = match os::self_exe_name() {
		Some(path) => path,

		// If we can't find the executable's path, then just use
		// the current directory.
		None => Path::new("."),
	};

	// Create the Java virtual machine. The argument to this function
	// is a list of paths that will be combined to create the Java
	// classpath. The classpath is a list of directories the JVM
	// will look in when trying to locate a .class or .jar file.
	let jvm = JavaVM::new(&[executable_path])
		.expect("Failed to create Java virtual machine!");

	// Load the `Test` class. The JVM will look for a `Test.class` file in
	// the classpath to find it.
	let class = jvm.class("Test")
		.expect("Couldn't find class `Test`!");

	// Call the static method.
	// The first argument is the name of the static, the second is an array
	// containing the arguments to pass into the static, and the third
	// is the return type of the static.
	let value = class.call_static_method("add", &[Value::Int(1), Value::Int(9)], Type::Int);

	// Print the value that was returned by the static method.
	println!("{}", value);
}
