
//
//  Static
//  An example that calls a static method on a class.
//

extern crate jni;

use std::path::{Path, PathBuf};
use jni::{JavaVM, Type, Value};
use std::env;


fn main() {
	// Find the path to this program's executable file, and look for the
	// Test.class file in there.
	let executable_path = env::current_exe()
		.unwrap_or(Path::new(".").to_path_buf());

	// Create the Java virtual machine. The argument to this function
	// is a list of paths that will be combined to create the Java
	// classpath. The classpath is a list of directories the JVM
	// will look in when trying to locate a .class or .jar file.
	let jvm = JavaVM::new(&[executable_path]).unwrap();

	// Load the `Test` class. The JVM will look for a `Test.class` file in
	// the classpath to find it.
	let class = jvm.class("Test").unwrap();

	// Call a static method.
	// The first argument is the name of the static, the second is an array
	// containing the arguments to pass into the static, and the third
	// is the return type of the static.
	let value1 = class.call_static_method(
		"add",
		&[Value::Int(1), Value::Int(9)],
		Type::Int
	).unwrap();

	// Print the value that was returned by the static method.
	println!("Result of 1 + 9: {:?}", value1);

	// Call another static method, this time passing in some strings.
	let value2 = class.call_static_method(
		"append",
		&[Value::String("hello".to_string())],
		Type::String
	).unwrap();

	// Print the value.
	println!("Result of string append function: {:?}", value2);
}
