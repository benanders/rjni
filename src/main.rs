
extern crate libc;
extern crate jni;

use jni::{JavaVM, Type, Value};


fn main() {
	let jvm = JavaVM::new(".").expect("Failed to create Java virtual machine!");
	let class = jvm.class("Test").expect("Couldn't find class!");
	let object = class.instance(&[Value::Int(5)]).expect("Couldn't instantiate class!");

	object.call("incrementCurrent", &[], Type::Void);
	let value = object.call("getCurrent", &[], Type::Int);

	println!("{}", value);
}
