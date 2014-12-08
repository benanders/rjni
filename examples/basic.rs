
//
//  Example
//  A simple example to load a test Java application (Test.java) and execute it.
//

extern crate jni;

use jni::{JavaVM, Type, Value};
use std::os;


fn main() {
	// Find the path to the executable, and look for the Test.class file in there.
	let executable_path = match os::self_exe_name() {
		Some(path) => path.dirname_str().unwrap().clone(),
		None => ".",
	};

	// Create the Java virtual machine, giving it the class path. The class path
	// is the location the JVM will look for any compiled .class and .jar files,
	// in order to locate a class.
	let jvm = JavaVM::new(executable_path).expect("Failed to create Java virtual machine!");

	// Load the `Test` class. The JVM will look for a Test.class file in the class
	// path to find it.
	let class = jvm.class("Test").expect("Couldn't find class!");

	// Create an instance of the `Test` class. The array of values is the arguments
	// to be passed to the class' constructor. In this case, the Test class'
	// constructor takes a single integer as its arguments.
	let object = class.instance(&[Value::Int(5)]).expect("Couldn't instantiate class!");

	// Call the method `incrementCurrent` on the instance we just created.
	// The empty array is the arguments to pass into the function, and the
	// last type is the return type of the function (in this case, void).
	object.call("incrementCurrent", &[], Type::Void);

	// Call the `getCurrent` method on the instance, and assign its
	// result to `value`. In this case, the function takes no arguments
	// and returns an integer.
	let value = object.call("getCurrent", &[], Type::Int).unwrap();

	// Print the value we just received.
	println!("{}", value);
}
