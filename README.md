Java Native Interface Bindings for Rust
=======================================

This library provides somewhat primitive bindings for the Java Native Interface in Rust. Arrays and custom classes are not supported yet, just simple static method calling, and object instantiation and method calling.

Currently only OSX is supported, as I haven't got around to testing/making the API on other OSes.

### Examples

*Calling a static method:*

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

	// Create the class the static method we want to call is on.
	// Note this doesn't instantiate an instance of the class.
	let class = jvm.class("Test").expect("Couldn't find the class! Check your classpath.");

	// Call the static function and get the return value.
	// The arguments in order are: the function name, the arguments to the function, and
	// the return type.
	// The function will return None if you've specified the wrong argument/return types,
	// or the function couldn't be found.
	let value = class.call_static_method("square", &[Value::Int(5)], Type::Int).unwrap();

	// Print out the value.
	println!("{}", value);
}
```

*Instantiating an object and calling an object method:*

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
	let jvm = JavaVM::new(".").expect("Failed to create JVM");
	let class = jvm.class("Test").expect("Failed to find class");

	// Create an instance of the class. The only argument is the list
	// of values to pass as arguments to the object's constructor.
	let object = class.instance(&[Value::Int(2)]).expect("Failed to create instance");

	// Call the increment method (a void method).
	object.call("incrementCurrent", &[], Type::Void).expect("Failed to increment current");

	// Get the value of current
	let value = object.call("getCurrent", &[], Type::Int).expect("Failed to get current");

	// Print out the value (should say 3)
	println!("Value: {}", value);
}
```

Compile the Java with `javac Test.java` and put the generated `Test.class` file in the root of the project. You need to use cargo to build the library because of the custom build script, which handles the compilation of a C interface layer.
