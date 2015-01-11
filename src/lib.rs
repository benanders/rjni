
//
//  JNI
//! A Rust wrapper around the Java Native Interface library.
//


extern crate libc;

use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

mod ffi;


/// Convert a variable into a pointer and copy it into `ptr`.
macro_rules! copy_into_ptr(
	($variable:expr, $ptr:expr, $size:expr) => (
		ptr::copy_memory(
			$ptr,
			&($variable) as *const _ as *const libc::c_void,
			($size) as usize
		);
	)
);



//
//  Java VM
//

/// An instance of a Java virtual machine.
/// JNI does not allow multiple VMs to be created.
pub struct JavaVM {
	should_call_destructor: bool,
}


impl JavaVM {

	/// Creates a new Java virtual machine using the given list
	/// of directories as the classpath.
	pub fn new(classpath_directories: &[Path]) -> Result<JavaVM, Error> {
		let mut classpath = String::new();
		for dir in classpath_directories.iter() {
			let string = dir.as_str().expect("Path could not be converted into a string");
			classpath.push_str(string);
			classpath.push(':');
		}

		let success = unsafe {
			let cstr = CString::from_slice(classpath.as_bytes());
			ffi::create_jvm(cstr.as_ptr())
		};

		if success != ffi::SUCCESS {
			Err(Error::from_status_code(ffi::error_status()))
		} else {
			Ok(JavaVM {
				should_call_destructor: true,
			})
		}
	}

	/// Creates a class from the given class name.
	///
	/// Note this doesn't instantiate an instance of this class.
	pub fn class(&self, name: &str) -> Result<Class, Error> {
		let ptr = unsafe {
			let cstr = CString::from_slice(name.as_bytes());
			ffi::class_from_name(cstr.as_ptr())
		};

		if ptr.is_null() {
			Err(Error::from_status_code(ffi::error_status()))
		} else {
			Ok(Class {
				java_class: ptr,
			})
		}
	}

	/// Sets whether the Java virtual machine will be explicitly destroyed
	/// when the JavaVM is dropped.
	pub fn set_calls_destructor(&mut self, should: bool) {
		self.should_call_destructor = should;
	}

}


impl Drop for JavaVM {

	fn drop(&mut self) {
		if self.should_call_destructor {
			// Destroy the JVM on drop
			unsafe {
				ffi::destroy_jvm();
			}
		}
	}

}



//
//  Types and Values
//

/// A function argument or return value.
#[derive(Show, Clone)]
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
	Void,
}


impl Value {

	/// Constructs a value object from a void pointer and type.
	///
	/// Panics if content is null.
	fn from_ptr(value_type: Type, content: *mut libc::c_void) -> Value {
		unsafe {
			if content.is_null() {
				panic!("Passing in NULL pointer to Value::from_ptr.");
			}

			let result = match value_type {
				Type::Byte =>
					Value::Byte(*(content as *mut i8)),
				Type::Short =>
					Value::Short(*(content as *mut i16)),
				Type::Int =>
					Value::Int(*(content as *mut i32)),
				Type::Long =>
					Value::Long(*(content as *mut i64)),
				Type::Float =>
					Value::Float(*(content as *mut libc::c_float) as f32),
				Type::Double =>
					Value::Double(*(content as *mut libc::c_double) as f64),
				Type::Boolean =>
					Value::Boolean(*(content as *mut i32) == 1),
				Type::Char =>
					Value::Char(*(content as *mut u8) as char),
				Type::String => {
					let str_ptr = content as *const libc::c_char;
					let bytes = std::ffi::c_str_to_bytes(&str_ptr);
					Value::String(str::from_utf8(bytes).unwrap().to_string())
				},
				Type::Void =>
					Value::Void,
			};

			libc::free(content);
			result
		}
	}

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
			Value::Void => Type::Void,
		}
	}

	/// Returns the number of bytes this value requires when allocated.
	pub fn bytes(&self) -> u64 {
		(match *self {
			Value::Byte(_) => mem::size_of::<i8>(),
			Value::Short(_) => mem::size_of::<i16>(),
			Value::Int(_) => mem::size_of::<i32>(),
			Value::Long(_) => mem::size_of::<i64>(),
			Value::Float(_) => mem::size_of::<f32>(),
			Value::Double(_) => mem::size_of::<f64>(),
			Value::Boolean(_) => mem::size_of::<u32>(),
			Value::Char(_) => mem::size_of::<u8>(),
			Value::String(ref string) =>
				(mem::size_of::<u8>() * (string.as_bytes().len() + 1)),
			Value::Void => 0,
		}) as u64
	}

	/// Converts the value from a byte to an i8. Panics if the value is not a byte.
	pub fn to_i8(&self) -> i8 {
		match *self {
			Value::Byte(v) => v,
			_ => panic!("Calling `to_i8` on value"),
		}
	}

	/// Converts the value from a short to an i16. Panics if the value is not a short.
	pub fn to_i16(&self) -> i16 {
		match *self {
			Value::Short(v) => v,
			_ => panic!("Calling `to_i16` on value"),
		}
	}

	/// Converts the value from an int to an i32. Panics if the value is not an int.
	pub fn to_i32(&self) -> i32 {
		match *self {
			Value::Int(v) => v,
			_ => panic!("Calling `to_i32` on value"),
		}
	}

	/// Converts the value from a long to an i64. Panics if the value is not a long.
	pub fn to_i64(&self) -> i64 {
		match *self {
			Value::Long(v) => v,
			_ => panic!("Calling `to_i64` on value"),
		}
	}

	/// Converts the value from a float to an f32. Panics if the value is not a float.
	pub fn to_f32(&self) -> f32 {
		match *self {
			Value::Float(v) => v,
			_ => panic!("Calling `to_f32` on value"),
		}
	}

	/// Converts the value from a double to an f64. Panics if the value is not a double.
	pub fn to_f64(&self) -> f64 {
		match *self {
			Value::Double(v) => v,
			_ => panic!("Calling `to_f64` on value"),
		}
	}

	/// Converts the value from a boolean to a bool. Panics if the value is not a boolean.
	pub fn to_bool(&self) -> bool {
		match *self {
			Value::Boolean(v) => v,
			_ => panic!("Calling `to_bool` on value"),
		}
	}

	/// Converts the value to a char. Panics if the value is not a char.
	pub fn to_char(&self) -> char {
		match *self {
			Value::Char(v) => v,
			_ => panic!("Calling `to_char` on value"),
		}
	}

	/// Converts the value to a string. Panics if the value is not a string.
	pub fn to_string(&self) -> String {
		match *self {
			Value::String(ref v) => v.clone(),
			_ => panic!("Calling `to_string` on value"),
		}
	}

}


/// A function argument or return type.
#[derive(Show, Clone, Copy)]
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
fn arguments_to_void_pointers<T, F>(arguments: &[Value], callback: F) -> T
		where F: Fn(Vec<*mut libc::c_void>) -> T {
	let mut values = Vec::new();
	for value in arguments.iter() {
		// Protect against passing in void
		match *value {
			Value::Void => panic!("Cannot pass `Value::Void` as an argument to a function."),
			_ => {}
		}

		let ptr = unsafe {
			// Allocate heap space for the argument
			let size = value.bytes();
			let ptr = libc::malloc(size);

			// Convert to a void pointer
			match *value {
				Value::Byte(v) => copy_into_ptr!(v, ptr, size),
				Value::Short(v) => copy_into_ptr!(v, ptr, size),
				Value::Int(v) => copy_into_ptr!(v, ptr, size),
				Value::Long(v) => copy_into_ptr!(v, ptr, size),
				Value::Float(v) => copy_into_ptr!(v, ptr, size),
				Value::Double(v) => copy_into_ptr!(v, ptr, size),
				Value::Char(v) => copy_into_ptr!(v, ptr, size),
				Value::Boolean(v) => {
					let as_int: i32 = if v { 1 } else { 0 };
					copy_into_ptr!(as_int, ptr, size)
				},
				Value::String(ref v) => {
					let string = CString::from_slice(v.as_bytes());
					let str_ptr = string.as_ptr();
					ptr::copy_memory(
						ptr,
						str_ptr as *const libc::c_void,
						size as usize
					);
				},
				_ => {},
			}

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
#[derive(Show, Copy)]
pub struct Class {
	java_class: *mut libc::c_void,
}


impl Class {

	/// Creates a new instance of this class.
	pub fn instance(&self, constructor_arguments: &[Value]) -> Result<Object, Error> {
		let signature = signature_for_function(constructor_arguments, Type::Void);

		arguments_to_void_pointers(constructor_arguments, |mut values| {
			let mut types = ffi::arguments_to_type_list(constructor_arguments);
			unsafe {
				let signature_cstr = CString::from_slice(signature.as_bytes());
				let object_ptr = ffi::create_object(
					self.java_class,
					signature_cstr.as_ptr(),
					constructor_arguments.len() as i32,
					types.as_mut_slice().as_mut_ptr(),
					values.as_mut_slice().as_mut_ptr(),
				);


				if object_ptr.is_null() {
					Err(Error::from_status_code(ffi::error_status()))
				} else {
					Ok(Object {
						java_object: object_ptr,
					})
				}
			}
		})
	}

	/// Calls a static method on this class.
	pub fn call_static_method(&self, name: &str, arguments: &[Value], return_type: Type)
			-> Result<Value, Error> {
		let signature = signature_for_function(arguments, return_type);

		arguments_to_void_pointers(arguments, |mut values| {
			let mut types = ffi::arguments_to_type_list(arguments);
			unsafe {
				// Call the static method
				let name_cstr = CString::from_slice(name.as_bytes());
				let signature_cstr = CString::from_slice(signature.as_bytes());
				let return_value = ffi::call_static_method(
					self.java_class,
					name_cstr.as_ptr(),
					signature_cstr.as_ptr(),
					ffi::type_to_integer(return_type),
					arguments.len() as i32,
					types.as_mut_slice().as_mut_ptr(),
					values.as_mut_slice().as_mut_ptr(),
				);

				if return_value.is_null() {
					let status = ffi::error_status();
					if status == ffi::ERROR_NONE {
						Ok(Value::Void)
					} else {
						Err(Error::from_status_code(status))
					}
				} else {
					Ok(Value::from_ptr(return_type, return_value))
				}
			}
		})
	}

}



//
//  Object
//

/// An instance of a Java class.
#[derive(Show, Copy)]
pub struct Object {
	java_object: *mut libc::c_void,
}


impl Object {

	/// Calls a method on this object instance.
	pub fn call(&self, name: &str, arguments: &[Value], return_type: Type)
			-> Result<Value, Error> {
		let signature = signature_for_function(arguments, return_type);

		arguments_to_void_pointers(arguments, |mut values| {
			let mut types = ffi::arguments_to_type_list(arguments);
			unsafe {

				// Call the static method
				let name_cstr = CString::from_slice(name.as_bytes());
				let signature_cstr = CString::from_slice(signature.as_bytes());
				let return_value = ffi::call_method(
					self.java_object,
					name_cstr.as_ptr(),
					signature_cstr.as_ptr(),
					ffi::type_to_integer(return_type),
					arguments.len() as i32,
					types.as_mut_slice().as_mut_ptr(),
					values.as_mut_slice().as_mut_ptr(),
				);

				if return_value.is_null() {
					let status = ffi::error_status();
					if status == ffi::ERROR_NONE {
						Ok(Value::Void)
					} else {
						Err(Error::from_status_code(status))
					}
				} else {
					Ok(Value::from_ptr(return_type, return_value))
				}
			}
		})
	}

}



//
//  Errors
//

/// Possible errors that may occur.
#[derive(Show, Copy)]
pub enum Error {
	/// Triggered when the creation of the Java virtual machine
	/// in the `JavaVM::new` function fails.
	VirtualMachineCreationFailed,

	/// Triggered if a second Java virtual machine is created.
	VirtaulMachineAlreadyExists,

	/// Triggered if an error occurred when attempting to allocate
	/// heap memory.
	MemoryAllocationFailure,

	/// Triggered when a class could not be found from its name.
	ClassNotFound,

	/// Triggered when a method could not be found, either because
	/// the function with the given name doesn't exist, or the
	/// method signature does not match.
	MethodNotFound,

	/// Triggered if another internal error occurred.
	InternalError,
}


impl Error {

	/// Creates an error from a status code.
	fn from_status_code(code: i32) -> Error {
		match code {
			ffi::ERROR_COULD_NOT_CREATE_VM => Error::VirtualMachineCreationFailed,
			ffi::ERROR_VM_ALREADY_EXISTS => Error::VirtaulMachineAlreadyExists,
			ffi::ERROR_COULD_NOT_ALLOCATE_MEMORY => Error::MemoryAllocationFailure,
			ffi::ERROR_CLASS_NOT_FOUND => Error::ClassNotFound,
			ffi::ERROR_METHOD_NOT_FOUND => Error::MethodNotFound,
			_ => Error::InternalError,
		}
	}

}
