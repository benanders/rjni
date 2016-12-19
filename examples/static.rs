
//
//  Static
//  An example that calls a static method on a class.
//

extern crate rjni;

use std::path::PathBuf;
use std::env;

use rjni::{JavaVM, Version, Classpath, Options, Value, Type};

fn main() {
	// Find the path to the manifest folder, then append the examples directory
	// to it. This acts as our classpath, where the JVM will look for any .class
	// files that we want to load.
	let manifest = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string());
	let mut path = PathBuf::from(manifest);
	path.push("examples");
	let classpath = Classpath::new().add(path);

	// Create the list of options used to initialise the JVM, specifying the
	// version number
	let options = Options::new()
		.version(Version::V16)
		.classpath(classpath);

	// Create the Java virtual machine. The argument to this function
	// is a list of paths that will be combined to create the Java
	// classpath. The classpath is a list of directories the JVM
	// will look in when trying to locate a .class or .jar file.
	let jvm = JavaVM::new(options).unwrap();

	// Load the `Test` class. The JVM will look for a `Test.class` file in
	// the classpath to find it.
	let class = jvm.class("Test").unwrap();

	// Call a static method.
	// The first argument is the name of the static, the second is an array
	// containing the arguments to pass into the static, and the third
	// is the return type of the static.
	let value1 = class.call_static(
		"add",
		&[Value::Int(1), Value::Int(9)],
		Type::Int
	).unwrap();

	// Print the value that was returned by the static method.
	println!("Result of 1 + 9: {:?}", value1);

	// Call another static method, this time passing in some strings.
	let value2 = class.call_static(
		"append",
		&[Value::Str(String::from("hello"))],
		Type::Str
	).unwrap();

	// Print the value.
	println!("Result of string append function: {:?}", value2);
}
