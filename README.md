Java Native Interface Bindings for Rust
=======================================

This library provides somewhat primitive bindings for the Java Native Interface in Rust. Arrays and custom classes are not supported yet, just simple static method calling, and object instantiation and method calling.

An example calling a static method:
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

use jni::{JavaVM, Class, Function, Value, Type};

fn main() {
	// Create a new Java virtual machine, specifying the class path.
	let jvm = JavaVM::new(".").expect("Failed to create Java virtual machine!");

	// Create the class the static method we want to call is on.
	// Note this doesn't instantiate an instance of the class.
	let class = jvm.class("Test").expect("Couldn't find the class! Check your classpath.");

	// Create a function call object. The first argument is the function name, the
	// second is the arguments list, and the third is the return type.
	let call = Function::new("square", &[Value::Int(5)], Type::Int);

	// Call the function and get the return value.
	// The function will return None if you've specified the wrong argument/return types,
	// or the function couldn't be found.
	let value = class.call_static_method(&call).unwrap();

	// Print out the value.
	println!("{}", value);
}
```
