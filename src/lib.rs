
//
//  JNI
//! A Rust wrapper around the Java Native Interface library.
//


extern crate libc;

use std::mem;
use std::ptr;

mod ffi;



//
//  Java VM
//

/// An instance of a Java virtual machine.
/// JNI does not allow multiple VMs to be created.
pub struct JavaVM;


impl JavaVM {

	/// Creates a new Java virtual machine using the given list
	/// of directories as the classpath.
	///
	/// Returns None when the JVM failed to be created.
	pub fn new(classpath_directories: &[Path]) -> Option<JavaVM> {
		let mut classpath = String::new();
		for dir in classpath_directories.iter() {
			let string = dir.as_str().expect("Path could not be converted into a string");
			classpath.push_str(string);
			classpath.push(':');
		}

		let success = unsafe {
			ffi::create_jvm(classpath.as_slice().to_c_str().as_mut_ptr())
		};

		if success != ffi::SUCCESS {
			None
		} else {
			Some(JavaVM)
		}
	}

	/// Creates a class from the given class name.
	///
	/// Note this doesn't instantiate an instance of this class.
	/// Returns None if the class couldn't be found.
	pub fn class(&self, name: &str) -> Option<Class> {
		let ptr = unsafe {
			ffi::class_from_name(name.to_c_str().as_mut_ptr())
		};

		if ptr.is_null() {
			None
		} else {
			Some(Class {
				java_class: ptr,
			})
		}
	}

}


impl Drop for JavaVM {

	fn drop(&mut self) {
		// Destroy the JVM on drop
		unsafe {
			ffi::destroy_jvm();
		}
	}

}



//
//  Types and Values
//

/// Constructs a value object from a void pointer and type.
/// Returns None if the type cannot be converted to a value (eg. if it is void).
fn value_from_ptr(value_type: Type, content: *mut libc::c_void) -> Option<Value> {
	unsafe {
		let result = match value_type {
			Type::Byte => Some(Value::Byte(*(content as *mut i8))),
			Type::Short => Some(Value::Short(*(content as *mut i16))),
			Type::Int => Some(Value::Int(*(content as *mut i32))),
			Type::Long => Some(Value::Long(*(content as *mut i64))),
			Type::Float => Some(Value::Float(*(content as *mut libc::c_float) as f32)),
			Type::Double => Some(Value::Double(*(content as *mut libc::c_double) as f64)),
			Type::Boolean => Some(Value::Boolean(*(content as *mut i32) == 1)),
			Type::Char => Some(Value::Char(*(content as *mut u8) as char)),
			Type::String => Some(Value::String(String::from_raw_buf(content as *const u8))),
			Type::Void => None,
		};

		libc::free(content);
		result
	}
}

/// A function argument or return value.
#[deriving(Show, Clone)]
pub enum Value {
	Byte(i8),
	Short(i16),
	Int(i32),
	Long(i64),
	Float(f32),
	Double(f64),
	Boolean(bool),
	Char(char),
	String(String),
}


impl Value {

	/// Returns the type of this value, dropping the
	/// value stored in the enum.
	pub fn to_type(&self) -> Type {
		match *self {
			Value::Byte(_) => Type::Byte,
			Value::Short(_) => Type::Short,
			Value::Int(_) => Type::Int,
			Value::Long(_) => Type::Long,
			Value::Float(_) => Type::Float,
			Value::Double(_) => Type::Double,
			Value::Boolean(_) => Type::Boolean,
			Value::Char(_) => Type::Char,
			Value::String(_) => Type::String,
		}
	}

	/// Returns the number of bytes this value requires when allocated.
	pub fn bytes(&self) -> u64 {
		match *self {
			Value::Byte(_) => mem::size_of::<u8>() as u64,
			Value::Short(_) => mem::size_of::<u16>() as u64,
			Value::Int(_) => mem::size_of::<u32>() as u64,
			Value::Long(_) => mem::size_of::<u64>() as u64,
			Value::Float(_) => mem::size_of::<f32>() as u64,
			Value::Double(_) => mem::size_of::<f64>() as u64,
			Value::Boolean(_) => mem::size_of::<u32>() as u64,
			Value::Char(_) => mem::size_of::<u8>() as u64,
			Value::String(ref string) =>
				(mem::size_of::<u8>() * (string.as_bytes().len() + 1)) as u64,
		}
	}

}


/// A function argument or return type.
#[deriving(Show)]
pub enum Type {
	Byte,
	Short,
	Int,
	Long,
	Float,
	Double,
	Boolean,
	Char,
	String,
	Void,
}


impl Type {

	/// Returns the type signature string for this type.
	fn signature(&self) -> &str {
		match *self {
			Type::Byte => "B",
			Type::Short => "S",
			Type::Int => "I",
			Type::Long => "J",
			Type::Float => "F",
			Type::Double => "D",
			Type::Boolean => "Z",
			Type::Char => "C",
			Type::String => "Ljava/lang/String;",
			Type::Void => "V",
		}
	}

}


//
//  Function
//

/// Returns the JNI function signature string for the given function arguments
/// and return type.
fn signature_for_function(arguments: &[Value], return_type: Type) -> String {
	let mut result = String::new();
	result.push('(');

	for argument in arguments.iter() {
		result.push_str(argument.to_type().signature());
	}

	result.push(')');
	result.push_str(return_type.signature());
	result
}


/// Converts each value in the arguments array into a void pointer.
fn arguments_to_void_pointers<T>(arguments: &[Value], callback: |Vec<*mut libc::c_void>| -> T)
		-> T {
	let mut values = Vec::new();
	for value in arguments.iter() {
		let ptr = unsafe {
			// Allocate heap space for the argument
			let size = value.bytes();
			let ptr = libc::malloc(size);

			// Convert to a void pointer
			let copy_ptr = match *value {
				Value::Byte(v) => &v as *const _ as *const libc::c_void,
				Value::Short(v) => &v as *const _ as *const libc::c_void,
				Value::Int(v) => &v as *const _ as *const libc::c_void,
				Value::Long(v) => &v as *const _ as *const libc::c_void,
				Value::Float(v) => &v as *const _ as *const libc::c_void,
				Value::Double(v) => &v as *const _ as *const libc::c_void,
				Value::Boolean(v) => {
					let as_int: i32 = if v { 1 } else { 0 };
					&as_int as *const _ as *const libc::c_void
				},
				Value::Char(v) => &v as *const _ as *const libc::c_void,
				Value::String(ref v) =>
					&v.to_c_str().as_mut_ptr() as *const _ as *const libc::c_void,
			};

			// Copy the pointer value across
			ptr::copy_memory(ptr, copy_ptr, size as uint);

			ptr
		};

		values.push(ptr);
	}

	callback(values)
}



//
//  Class
//

/// A Java class (not an instance of a class).
pub struct Class {
	java_class: *mut libc::c_void,
}


impl Class {

	/// Creates a new instance of this class.
	/// Returns None if the constructor's arguments list is incorrect.
	/// Create the constructor using `Function::constructor(arguments)`.
	pub fn instance(&self, constructor_arguments: &[Value]) -> Option<Object> {
		let mut types = ffi::arguments_to_type_list(constructor_arguments);
		let signature = signature_for_function(constructor_arguments, Type::Void);

		arguments_to_void_pointers(constructor_arguments, |mut values| {
			unsafe {
				let object_ptr = ffi::create_object(
					self.java_class,
					signature.to_c_str().as_mut_ptr(),
					constructor_arguments.len() as i32,
					types.as_mut_slice().as_mut_ptr(),
					values.as_mut_slice().as_mut_ptr(),
				);


				if object_ptr.is_null() {
					None
				} else {
					Some(Object {
						java_object: object_ptr,
					})
				}
			}
		})
	}

	/// Calls a static method on this class.
	/// Returns None if the method couldn't be found, the function signature
	/// is incorrect, or the method returns void.
	pub fn call_static_method(&self, name: &str, arguments: &[Value], return_type: Type)
			-> Option<Value> {
		let mut types = ffi::arguments_to_type_list(arguments);
		let signature = signature_for_function(arguments, return_type);

		arguments_to_void_pointers(arguments, |mut values| {
			unsafe {
				// Call the static method
				let return_value = ffi::call_static_method(
					self.java_class,
					name.to_c_str().as_mut_ptr(),
					signature.as_slice().to_c_str().as_mut_ptr(),
					ffi::type_to_integer(return_type),
					arguments.len() as i32,
					types.as_mut_slice().as_mut_ptr(),
					values.as_mut_slice().as_mut_ptr(),
				);

				if return_value.is_null() {
					None
				} else {
					value_from_ptr(return_type, return_value)
				}
			}
		})
	}

}



//
//  Object
//

/// An instance of a Java class.
pub struct Object {
	java_object: *mut libc::c_void,
}


impl Object {

	/// Calls a method on this object instance.
	/// Returns None if the method couldn't be found, the signature was incorrect,
	/// or the function returns void.
	pub fn call(&self, name: &str, arguments: &[Value], return_type: Type) -> Option<Value> {
		let mut types = ffi::arguments_to_type_list(arguments);
		let signature = signature_for_function(arguments, return_type);

		arguments_to_void_pointers(arguments, |mut values| {
			unsafe {
				// Call the static method
				let return_value = ffi::call_method(
					self.java_object,
					name.to_c_str().as_mut_ptr(),
					signature.as_slice().to_c_str().as_mut_ptr(),
					ffi::type_to_integer(return_type),
					arguments.len() as i32,
					types.as_mut_slice().as_mut_ptr(),
					values.as_mut_slice().as_mut_ptr(),
				);

				if return_value.is_null() {
					None
				} else {
					value_from_ptr(return_type, return_value)
				}
			}
		})
	}

}
