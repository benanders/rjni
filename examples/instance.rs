
//
//  Instance
//  An example that creates an instance of a Java class and calls a method on it.
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
	let jvm = JavaVM::new(&[executable_path]).unwrap();

	// Load the `Test` class. The JVM will look for a `Test.class` file in
	// the classpath to find it.
	let class = jvm.class("Test").unwrap();

	// Create an instance of the `Test` class. The array of values are the arguments
	// to be passed to the class' constructor. In this case, the Test class'
	// constructor takes a single integer as an argument.
	//
	// A none value will be returned if the constructor's arguments are invalid,
	// a Java error occurred, or any other reason the JNI can come up with.
	let object = class.instance(&[Value::Int(5)]).unwrap();

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
	println!("{}", value);
}
