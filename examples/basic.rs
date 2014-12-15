
//
//  Example
//  A simple example to load a test Java application (Test.java) and execute it.
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

	// Create an instance of the `Test` class. The array of values are the arguments
	// to be passed to the class' constructor. In this case, the Test class'
	// constructor takes a single integer as an argument.
	//
	// A none value might be returned if the constructor's arguments are invalid,
	// or a Java error occurred, or any other reason the JNI can come up with.
	let object = class.instance(&[Value::Int(5)])
		.expect("Couldn't instantiate class!");

	// Call the method `incrementCurrent` on the object we just created.
	// The empty array specifies the arguments to pass into the function,
	// and the second argument is the return type of the function (in
	// this case, void).
	object.call("incrementCurrent", &[], Type::Void);

	// Call the `getCurrent` method on the object, and assign its
	// return value to `value`. In this case, the function takes no
	// arguments and returns an integer.
	let value = object.call("getCurrent", &[], Type::Int).unwrap();

	// Print the value we just fetched from the object.
	println!("{}", value);
}
