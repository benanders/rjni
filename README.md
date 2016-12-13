
Java Native Interface Bindings for Rust
=======================================

This library provides a somewhat primitive binding to the Java Native Interface in Rust. Arrays and custom classes are not supported yet, just simple static method calling, object instantiation, and method calling on instantiated objects.

It's really just a proof of concept at the moment, and shouldn't be used for anything too serious. It'd be a useful starting point for someone if they wanted to make a more production ready version.

I've only tested this on OSX, and you will get a linker error on any other platform due to a hard-coded include path specific to OSX in the build script (for the `jni.h` header). It should be fairly easy to change for other platforms though.

### Examples

**Calling a static method:**

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
	let jvm = JavaVM::new(&[Path::new(".")]).unwrap();

	// Load the `Test` class. The JVM will look for a `Test.class` file in
	// the classpath to find it.
	let class = jvm.class("Test").unwrap();

	// Call a static method.
	// The first argument is the name of the static, the second is an array
	// containing the arguments to pass into the static, and the third
	// is the return type of the static.
	let value = class.call_static_method("add", &[Value::Int(1), Value::Int(9)], Type::Int).unwrap();

	// Print the value that was returned by the static method.
	println!("Result of 1 + 9: {:?}", value);
}
```

**Instantiating an object and calling a method:**

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
	let jvm = JavaVM::new(&[Path::new(".")]).unwrap();

	// Load the `Test` class. The JVM will look for a `Test.class` file in
	// the classpath to find it.
	let class = jvm.class("Test").unwrap();

	// Create an instance of the `Test` class. The array of values are the arguments
	// to be passed to the class' constructor. In this case, the Test class'
	// constructor takes a single integer as an argument.
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
	println!("{:?}", value);
}
```

Compile the Java with `javac Test.java` and put the generated `Test.class` file in the root of the project. You need to use cargo to build the library (run `cargo build`) because of the custom build script, which handles the compilation of a C interface layer.
