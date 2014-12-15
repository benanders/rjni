
Java Native Interface Bindings for Rust
=======================================

This library provides a somewhat primitive binding to the Java Native Interface in Rust. Arrays and custom classes are not supported yet, just simple static method calling, object instantiation, and method calling on instantiated objects.

I've only tested this on OSX, and you'll almost certainly get a linking error on any other OS due to the compiler not being able to find the jni.h file. I've only added the OSX include directory in the build script, but it should be fairly easy to change.

### Examples

_Calling a static method:_

```java
// Test.java

public class Test {
	public static int add(int a, int b) {
		return a + b;
	}
}
```

```rust
// main.rs

extern crate jni;

use jni::{JavaVM, Value, Type};

fn main() {
	// Create the Java virtual machine. The argument to this function
	// is a list of paths that will be combined to create the Java
	// classpath. The classpath is a list of directories the JVM
	// will look in when trying to locate a .class or .jar file.
	let jvm = JavaVM::new(&[Path::new(".")])
		.expect("Failed to create Java virtual machine!");

	// Load the `Test` class. The JVM will look for a `Test.class` file in
	// the classpath to find it.
	let class = jvm.class("Test")
		.expect("Couldn't find class `Test`!");

	// Call the static method.
	// The first argument is the name of the static, the second is an array
	// containing the arguments to pass into the static, and the third
	// is the return type of the static.
	let result = class.call_static_method("add", &[Value::Int(5), Value::Int(7)], Type::Int);

	// Print the value that was returned by the static method.
	println!("{}", value);
}
```

_Instantiating an object and calling a method:_

```java
// Test.java

public class Test {
	public int current;

	// The constructor
	public Test(int newCurrent) {
		this.current = newCurrent;
	}

	public void incrementCurrent() {
		this.current += 1;
	}

	public int getCurrent() {
		return this.current;
	}
}
```

```rust
// main.rs

extern crate jni;

use jni::{JavaVM, Value, Type};

fn main() {
	// Create the Java virtual machine. The argument to this function
	// is a list of paths that will be combined to create the Java
	// classpath. The classpath is a list of directories the JVM
	// will look in when trying to locate a .class or .jar file.
	let jvm = JavaVM::new(&[Path::new(".")])
		.expect("Failed to create Java virtual machine!");

	// Load the `Test` class. The JVM will look for a `Test.class` file in
	// the classpath to find it.
	let class = jvm.class("Test")
		.expect("Couldn't find class `Test`!");

	// Create an instance of the `Test` class. The array of values are the arguments
	// to be passed to the class' constructor. In this case, the Test class'
	// constructor takes a single integer as an argument.
	//
	// A none value will be returned if the constructor's arguments are invalid,
	// a Java error occurred, or any other reason the JNI can come up with.
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
```

Compile the Java with `javac Test.java` and put the generated `Test.class` file in the root of the project. You need to use cargo to build the library (run `cargo build`) because of the custom build script, which handles the compilation of a C interface layer.
