
//
//  JNI
//! Direct foreign functions wrapper around C interface.
//

extern crate libc;

use {Value, Type};

pub const SUCCESS: i32 = 1;

pub const TYPE_BYTE: i32 = 0;
pub const TYPE_SHORT: i32 = 1;
pub const TYPE_INT: i32 = 2;
pub const TYPE_LONG: i32 = 3;
pub const TYPE_FLOAT: i32 = 4;
pub const TYPE_DOUBLE: i32 = 5;
pub const TYPE_BOOLEAN: i32 = 6;
pub const TYPE_CHAR: i32 = 7;
pub const TYPE_STRING: i32 = 8;
pub const TYPE_VOID: i32 = 9;

pub const ERROR_NONE: i32 = -1;
pub const ERROR_COULD_NOT_CREATE_VM: i32 = 0;
pub const ERROR_VM_ALREADY_EXISTS: i32 = 1;
pub const ERROR_COULD_NOT_ALLOCATE_MEMORY: i32 = 3;
pub const ERROR_CLASS_NOT_FOUND: i32 = 4;
pub const ERROR_METHOD_NOT_FOUND: i32 = 5;


/// Converts a type to an integer for passing into function call methods.
pub fn type_to_integer(t: Type) -> i32 {
	match t {
		Type::Byte => TYPE_BYTE,
		Type::Short => TYPE_SHORT,
		Type::Int => TYPE_INT,
		Type::Long => TYPE_LONG,
		Type::Float => TYPE_FLOAT,
		Type::Double => TYPE_DOUBLE,
		Type::Boolean => TYPE_BOOLEAN,
		Type::Char => TYPE_CHAR,
		Type::String => TYPE_STRING,
		Type::Void => TYPE_VOID,
	}
}


/// A safe wrapper to fetch the latest error code.
pub fn error_status() -> i32 {
	unsafe {
		get_error() as i32
	}
}


/// Converts a list of arguments into a list of function argument integer types.
pub fn arguments_to_type_list(arguments: &[Value]) -> Vec<i32> {
	arguments.iter().map(|arg| type_to_integer(arg.to_type())).collect()
}


#[link(name = "interface", kind = "static")]
#[link(name = "JavaVM", kind = "framework")]
extern {
	/// Creates the Java virtual machine instance.
	pub fn create_jvm(classpath: *const libc::c_char) -> libc::c_int;

	/// Destroys the Java virtual machine instance.
	pub fn destroy_jvm();

	/// Fetches a class from the given name, returning NULL on failure.
	pub fn class_from_name(name: *const libc::c_char) -> *mut libc::c_void;

	/// Fetches the error code from the last function call.
	pub fn get_error() -> libc::c_int;

	/// Calls a static method on a class.
	/// Must free the return value!
	pub fn call_static_method(
		java_class: *mut libc::c_void,
		method_name: *const libc::c_char,
		method_signature: *const libc::c_char,
		return_type: libc::c_int,
		argument_count: libc::c_int,
		argument_types: *mut libc::c_int,
		argument_values: *mut *mut libc::c_void,
	) -> *mut libc::c_void;

	/// Creates an instance of a new class.
	pub fn create_object(
		java_class: *mut libc::c_void,
		constructor_signature: *const libc::c_char,
		argument_count: libc::c_int,
		argument_types: *mut libc::c_int,
		argument_values: *mut *mut libc::c_void,
	) -> *mut libc::c_void;

	/// Call a method on an object.
	/// Must free the return value!
	pub fn call_method(
		java_object: *mut libc::c_void,
		method_name: *const libc::c_char,
		method_signature: *const libc::c_char,
		return_type: libc::c_int,
		argument_count: libc::c_int,
		argument_types: *mut libc::c_int,
		argument_values: *mut *mut libc::c_void,
	) -> *mut libc::c_void;
}
