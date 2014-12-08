
Java Native Interface Bindings for Rust
=======================================

This library provides a somewhat primitive binding to the Java Native Interface in Rust. Arrays and custom classes are not supported yet, just simple static method calling, object instantiation, and method calling on instantiated objects.

I've only tested this on OSX, and you'll almost certainly get a linking error on any other OS due to the compiler not being able to find the jni.h file. I've only included the OSX include directory in the build script, but it should be fairly easy to change.

### Examples

_Calling a static method:_

```java
// Test.java

public class Test {
	public static int square(int number) {
		return number * number;
	}
}
```

```rust
// main.rs

extern crate jni;

use jni::{JavaVM, Value, Type};

fn main() {
	// Create a new Java virtual machine, specifying the class path.
	let jvm = JavaVM::new(".").expect("Failed to create Java virtual machine!");

	// Load the Test class. Note this doesn't create an instance of the class!
	let class = jvm.class("Test").expect("Couldn't find the class! Check your classpath.");

	// Call the static function and get the return value.
	// The first argument is the name of the static method, the second
	// is the arguments that will be passed into the static function,
	// and the third is the return type of the function.
	// The function will return None if you've specified the wrong argument/return types,
	// or the function couldn't be found.
	let value = class.call_static_method("square", &[Value::Int(5)], Type::Int).unwrap();

	// Print out the value that we received from the static method.
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
	// Create the Java virtual machine, giving it the class path. The class path
	// is the location the JVM will look for any compiled .class and .jar files,
	// in order to locate a class.
	let jvm = JavaVM::new(".").expect("Failed to create Java virtual machine!");

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
```

Compile the Java with `javac Test.java` and put the generated `Test.class` file in the root of the project. You need to use cargo to build the library (run `cargo build`) because of the custom build script, which handles the compilation of a C interface layer.
