
//
//  Instance
//  An example that creates an instance of a Java class and calls a method on it.
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

	// Create an instance of the `Test` class. The array of values are the
	// arguments to be passed to the class' constructor. In this case, the Test
	// class' constructor takes a single integer as an argument.
	let object = class.instantiate(&[Value::Int(5)]).unwrap();

	// Call the method `incrementCurrent` on the object we just created.
	// The empty array specifies the arguments to pass into the function,
	// and the second argument is the return type of the function (in
	// this case, void).
	object.call("incrementCurrent", &[], Type::Void).unwrap();

	// Call the `getCurrent` method on the object, and assign its
	// return value to `value`. In this case, the function takes no
	// arguments and returns an integer.
	let value = object.call("getCurrent", &[], Type::Int).unwrap();

	// Print the value we just fetched from the object.
	println!("result: {:?}", value);
}
