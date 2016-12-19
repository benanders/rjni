
//
//  Simplified JNI FFI Bindings
//

//! The complete FFI bindings for the Java Native Interface.
//!
//! Not all of the features provided by the JNI are wrapped in a "safe" wrapper,
//! since I don't need most of them for the project I intend to use this for.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use libc;
use std::{mem, ptr};

pub const JNI_FALSE: jboolean = 0;
pub const JNI_TRUE: jboolean = 1;

pub type JavaVM = *mut JNIInvokeInterface;
pub type JNIEnv = *const JNINativeInterface;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum JNIVersion {
	JNI_VERSION_1_1 = 0x00010001,
	JNI_VERSION_1_2 = 0x00010002,
	JNI_VERSION_1_4 = 0x00010004,
	JNI_VERSION_1_5 = 0x00010005,
	JNI_VERSION_1_6 = 0x00010006,
	JNI_VERSION_1_7 = 0x00010007,
	JNI_VERSION_1_8 = 0x00010008,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum JNIError {
	JNI_OK          =  0, // Success
	JNI_ERR         = -1, // Unknown error
	JNI_EDETACHED   = -2, // Thread detached from VM
	JNI_EVERSION    = -3, // Version error
	JNI_ENOMEM      = -4, // Not enough memory
	JNI_EEXIST      = -5, // VM already created
	JNI_EINVAL      = -6, // Invalid arguments
}

#[repr(C)]
pub struct JavaVMInitArgs {
	pub version: JNIVersion,
	pub nOptions: jint,
	pub options: *mut JavaVMOption,
	pub ignoreUnrecognized: jboolean,
}

impl JavaVMInitArgs {
	pub fn empty() -> JavaVMInitArgs {
		JavaVMInitArgs {
			version: JNIVersion::JNI_VERSION_1_8,
			nOptions: 0,
			options: ptr::null_mut(),
			ignoreUnrecognized: JNI_TRUE,
		}
	}
}

#[repr(C)]
pub struct JavaVMOption {
	pub optionString: *const libc::c_char,
	pub extraInfo: *const libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JavaVMAttachArgs {
	pub version: JNIVersion,
	pub name: *const libc::c_char,
	pub group: jobject
}

pub type jvoid = libc::c_void;
pub type jboolean = libc::c_uchar;
pub type jbyte = libc::c_char;
pub type jchar = libc::c_ushort;
pub type jshort = libc::c_short;
pub type jint = libc::c_int;
pub type jsize = jint;
pub type jlong = i64;
pub type jfloat = libc::c_float;
pub type jdouble = libc::c_double;

pub type jobject = *mut libc::c_void;
pub type jclass = jobject;
pub type jthrowable = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jbooleanArray = jobject;
pub type jbyteArray = jobject;
pub type jcharArray = jobject;
pub type jshortArray = jobject;
pub type jintArray = jobject;
pub type jlongArray = jobject;
pub type jfloatArray = jobject;
pub type jdoubleArray = jobject;
pub type jobjectArray = jobject;
pub type jweak = jobject;

pub type jfieldID = *mut libc::c_void;
pub type jmethodID = *mut libc::c_void;

pub enum Empty {}
pub type MethodFn = extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jvalue;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct jvalue {
	pub data: u64,
}

impl jvalue {
	pub fn z(&self) -> jboolean { unsafe { mem::transmute(self.data as u8) } }
	pub fn b(&self) -> jbyte { unsafe { mem::transmute(self.data as u8) } }
	pub fn c(&self) -> jchar { unsafe { mem::transmute(self.data as u16) } }
	pub fn s(&self) -> jshort { unsafe { mem::transmute(self.data as u16) } }
	pub fn i(&self) -> jint { unsafe { mem::transmute(self.data as u32) } }
	pub fn j(&self) -> jlong { unsafe { mem::transmute(self.data) } }
	pub fn f(&self) -> jfloat { unsafe { mem::transmute(self.data as u32) } }
	pub fn d(&self) -> jdouble { unsafe { mem::transmute(self.data) } }
	pub fn l(&self) -> jobject { unsafe { mem::transmute(self.data) } }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jobjectRefType {
	JNIInvalidRefType = 0,
	JNILocalRefType = 1,
	JNIGlobalRefType = 2,
	JNIWeakGlobalRefType = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum JNIReleaseArrayElementsMode {
	JNI_ZERO = 0,
	JNI_COMMIT = 1,
	JNI_ABORT = 2,
}

#[repr(C)]
pub struct JNINativeMethod {
	name: *mut libc::c_char,
	signature: *mut libc::c_char,
	fnPtr: *mut jvoid,
}

#[repr(C)]
pub struct JNIInvokeInterface {
	reserved0: *mut jvoid,
	reserved1: *mut jvoid,
	reserved2: *mut jvoid,

	pub DestroyJavaVM: extern "C" fn(vm: *mut JavaVM) -> JNIError,
	pub AttachCurrentThread: extern "C" fn(vm: *mut JavaVM, penv: &mut *mut JNIEnv, args: *mut JavaVMAttachArgs) -> JNIError,
	pub DetachCurrentThread: extern "C" fn(vm: *mut JavaVM) -> JNIError,
	pub GetEnv: extern "C" fn(vm: *mut JavaVM, penv: &mut *mut JNIEnv, version: JNIVersion) -> JNIError,
	pub AttachCurrentThreadAsDaemon: extern "C" fn(vm: *mut JavaVM, penv: &mut *mut JNIEnv, args: *mut JavaVMAttachArgs) -> JNIError
}

#[repr(C)]
pub struct JNINativeInterface {
	reserved0: *mut jvoid,
	reserved1: *mut jvoid,
	reserved2: *mut jvoid,
	reserved3: *mut jvoid,

	// It's important all these functions stay in the order that they're
	// currently in, and that none are commented out, because each function
	// pointer corresponds to its original in the JNINativeInterface struct in
	// the `jni.h` header file.
	//
	// Since the Rust/C interface here relies on the location of each struct
	// field in memory, its important that the two versions of the struct
	// correspond.

	pub GetVersion: extern "C" fn(env: *mut JNIEnv) -> JNIVersion,

	pub DefineClass: extern "C" fn(env: *mut JNIEnv, name: *const libc::c_char, loader: jobject, buf: *const jbyte, len: jsize) -> jclass,
	pub FindClass:   extern "C" fn(env: *mut JNIEnv, name: *const libc::c_char) -> jclass,

	pub FromReflectedMethod: extern "C" fn(env: *mut JNIEnv, method: jobject) -> jmethodID,
	pub FromReflectedField:  extern "C" fn(env: *mut JNIEnv, field: jobject) -> jmethodID,

	pub ToReflectedMethod: extern "C" fn(env: *mut JNIEnv, cls: jclass, methodID: jmethodID, isStatic: jboolean) -> jmethodID,

	pub GetSuperclass:    extern "C" fn(env: *mut JNIEnv, sub: jclass) -> jclass,
	pub IsAssignableFrom: extern "C" fn(env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean,

	pub ToReflectedField: extern "C" fn(env: *mut JNIEnv, cls: jclass, fieldID: jfieldID, isStatic: jboolean) -> jobject,

	pub Throw:             extern "C" fn(env: *mut JNIEnv, obj: jthrowable) -> JNIError,
	pub ThrowNew:          extern "C" fn(env: *mut JNIEnv, class: jclass, msg: *const libc::c_char) -> JNIError,
	pub ExceptionOccurred: extern "C" fn(env: *mut JNIEnv) -> jthrowable,
	pub ExceptionDescribe: extern "C" fn(env: *mut JNIEnv),
	pub ExceptionClear:    extern "C" fn(env: *mut JNIEnv),
	pub FatalError:        extern "C" fn(env: *mut JNIEnv, msg: *const libc::c_char),

	pub PushLocalFrame: extern "C" fn(env: *mut JNIEnv, capacity: jint) -> JNIError,
	pub PopLocalFrame:  extern "C" fn(env: *mut JNIEnv, result: jobject) -> jobject,

	pub NewGlobalRef:        extern "C" fn(env: *mut JNIEnv, lobj: jobject) -> jobject,
	pub DeleteGlobalRef:     extern "C" fn(env: *mut JNIEnv, gref: jobject),
	pub DeleteLocalRef:      extern "C" fn(env: *mut JNIEnv, obj: jobject),
	pub IsSameObject:        extern "C" fn(env: *mut JNIEnv, obj1: jobject, obj2: jobject) -> jboolean,
	pub NewLocalRef:         extern "C" fn(env: *mut JNIEnv, lref: jobject) -> jobject,
	pub EnsureLocalCapacity: extern "C" fn(env: *mut JNIEnv, capacity: jint) -> JNIError,

	pub AllocObject: extern "C" fn(env: *mut JNIEnv, class: jclass) -> jobject,
	pub NewObject:   extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jobject,
	pub NewObjectV:  extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, Empty) -> jobject,
	pub NewObjectA:  extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jobject,

	pub GetObjectClass: extern "C" fn(env: *mut JNIEnv, obj: jobject) -> jclass,
	pub IsInstanceOf:   extern "C" fn(env: *mut JNIEnv, obj: jobject, class: jclass) -> jboolean,

	pub GetMethodID: extern "C" fn(env: *mut JNIEnv, class: jclass, name: *const libc::c_char, sig: *const ::libc::c_char) -> jmethodID,

	pub CallObjectMethod:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jobject,
	pub CallObjectMethodV:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jobject,
	pub CallObjectMethodA:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jobject,
	pub CallBooleanMethod:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jboolean,
	pub CallBooleanMethodV: extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jboolean,
	pub CallBooleanMethodA: extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jboolean,
	pub CallByteMethod:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jbyte,
	pub CallByteMethodV:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jbyte,
	pub CallByteMethodA:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jbyte,
	pub CallCharMethod:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jchar,
	pub CallCharMethodV:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jchar,
	pub CallCharMethodA:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jchar,
	pub CallShortMethod:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jshort,
	pub CallShortMethodV:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jshort,
	pub CallShortMethodA:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jshort,
	pub CallIntMethod:      extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jint,
	pub CallIntMethodV:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jint,
	pub CallIntMethodA:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jint,
	pub CallLongMethod:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jlong,
	pub CallLongMethodV:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jlong,
	pub CallLongMethodA:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jlong,
	pub CallFloatMethod:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jfloat,
	pub CallFloatMethodV:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jfloat,
	pub CallFloatMethodA:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jfloat,
	pub CallDoubleMethod:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jdouble,
	pub CallDoubleMethodV:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jdouble,
	pub CallDoubleMethodA:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jdouble,
	pub CallVoidMethod:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...),
	pub CallVoidMethodV:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty),
	pub CallVoidMethodA:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue),

	pub CallNonvirtualObjectMethod:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jobject,
	pub CallNonvirtualObjectMethodV:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jobject,
	pub CallNonvirtualObjectMethodA:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jobject,
	pub CallNonvirtualBooleanMethod:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jboolean,
	pub CallNonvirtualBooleanMethodV: extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jboolean,
	pub CallNonvirtualBooleanMethodA: extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jboolean,
	pub CallNonvirtualByteMethod:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jbyte,
	pub CallNonvirtualByteMethodV:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jbyte,
	pub CallNonvirtualByteMethodA:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jbyte,
	pub CallNonvirtualCharMethod:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jchar,
	pub CallNonvirtualCharMethodV:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jchar,
	pub CallNonvirtualCharMethodA:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jchar,
	pub CallNonvirtualShortMethod:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jshort,
	pub CallNonvirtualShortMethodV:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jshort,
	pub CallNonvirtualShortMethodA:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jshort,
	pub CallNonvirtualIntMethod:      extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jint,
	pub CallNonvirtualIntMethodV:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jint,
	pub CallNonvirtualIntMethodA:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jint,
	pub CallNonvirtualLongMethod:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jlong,
	pub CallNonvirtualLongMethodV:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jlong,
	pub CallNonvirtualLongMethodA:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jlong,
	pub CallNonvirtualFloatMethod:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jfloat,
	pub CallNonvirtualFloatMethodV:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jfloat,
	pub CallNonvirtualFloatMethodA:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jfloat,
	pub CallNonvirtualDoubleMethod:   extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) -> jdouble,
	pub CallNonvirtualDoubleMethodV:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty) -> jdouble,
	pub CallNonvirtualDoubleMethodA:  extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jdouble,
	pub CallNonvirtualVoidMethod:     extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...),
	pub CallNonvirtualVoidMethodV:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: Empty),
	pub CallNonvirtualVoidMethodA:    extern "C" fn(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue),

	pub GetFieldID: extern "C" fn(env: *mut JNIEnv, class: jclass, name: *const libc::c_char, sig: *const ::libc::c_char) -> jfieldID,

	pub GetObjectField:  extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jobject,
	pub GetBooleanField: extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jboolean,
	pub GetByteField:    extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jbyte,
	pub GetCharField:    extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jchar,
	pub GetShortField:   extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jshort,
	pub GetIntField:     extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jint,
	pub GetLongField:    extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jlong,
	pub GetFloatField:   extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jfloat,
	pub GetDoubleField:  extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jdouble,

	pub SetObjectField:  extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jobject),
	pub SetBooleanField: extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jboolean),
	pub SetByteField:    extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jbyte),
	pub SetCharField:    extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jchar),
	pub SetShortField:   extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jshort),
	pub SetIntField:     extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jint),
	pub SetLongField:    extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jlong),
	pub SetFloatField:   extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jfloat),
	pub SetDoubleField:  extern "C" fn(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jdouble),

	pub GetStaticMethodID: extern "C" fn(env: *mut JNIEnv, class: jclass, name: *const libc::c_char, sig: *const ::libc::c_char) -> jmethodID,

	pub CallStaticObjectMethod:   extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jobject,
	pub CallStaticObjectMethodV:  extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty) -> jobject,
	pub CallStaticObjectMethodA:  extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jobject,
	pub CallStaticBooleanMethod:  extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jboolean,
	pub CallStaticBooleanMethodV: extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty) -> jboolean,
	pub CallStaticBooleanMethodA: extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jboolean,
	pub CallStaticByteMethod:     extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jbyte,
	pub CallStaticByteMethodV:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty) -> jbyte,
	pub CallStaticByteMethodA:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jbyte,
	pub CallStaticCharMethod:     extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jchar,
	pub CallStaticCharMethodV:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty) -> jchar,
	pub CallStaticCharMethodA:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jchar,
	pub CallStaticShortMethod:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jshort,
	pub CallStaticShortMethodV:   extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty) -> jshort,
	pub CallStaticShortMethodA:   extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jshort,
	pub CallStaticIntMethod:      extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jint,
	pub CallStaticIntMethodV:     extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty) -> jint,
	pub CallStaticIntMethodA:     extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jint,
	pub CallStaticLongMethod:     extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jlong,
	pub CallStaticLongMethodV:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty) -> jlong,
	pub CallStaticLongMethodA:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jlong,
	pub CallStaticFloatMethod:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jfloat,
	pub CallStaticFloatMethodV:   extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty) -> jfloat,
	pub CallStaticFloatMethodA:   extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jfloat,
	pub CallStaticDoubleMethod:   extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...) -> jdouble,
	pub CallStaticDoubleMethodV:  extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty) -> jdouble,
	pub CallStaticDoubleMethodA:  extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue) -> jdouble,
	pub CallStaticVoidMethod:     extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, ...),
	pub CallStaticVoidMethodV:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: Empty),
	pub CallStaticVoidMethodA:    extern "C" fn(env: *mut JNIEnv, class: jclass, methodID: jmethodID, args: *const jvalue),

	pub GetStaticFieldID: extern "C" fn(env: *mut JNIEnv, class: jclass, name: *const libc::c_char, sig: *const ::libc::c_char) -> jfieldID,

	pub GetStaticObjectField:  extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID) -> jobject,
	pub GetStaticBooleanField: extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID) -> jboolean,
	pub GetStaticByteField:    extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID) -> jbyte,
	pub GetStaticCharField:    extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID) -> jchar,
	pub GetStaticShortField:   extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID) -> jshort,
	pub GetStaticIntField:     extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID) -> jint,
	pub GetStaticLongField:    extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID) -> jlong,
	pub GetStaticFloatField:   extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID) -> jfloat,
	pub GetStaticDoubleField:  extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID) -> jdouble,

	pub SetStaticObjectField:  extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID, val: jobject),
	pub SetStaticBooleanField: extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID, val: jboolean),
	pub SetStaticByteField:    extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID, val: jbyte),
	pub SetStaticCharField:    extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID, val: jchar),
	pub SetStaticShortField:   extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID, val: jshort),
	pub SetStaticIntField:     extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID, val: jint),
	pub SetStaticLongField:    extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID, val: jlong),
	pub SetStaticFloatField:   extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID, val: jfloat),
	pub SetStaticDoubleField:  extern "C" fn(env: *mut JNIEnv, class: jclass, fieldID: jfieldID, val: jdouble),

	pub NewString:          extern "C" fn(env: *mut JNIEnv, unicode: *const jchar, len: jsize) -> jstring,
	pub GetStringLength:    extern "C" fn(env: *mut JNIEnv, strg: jstring) -> jsize,
	pub GetStringChars:     extern "C" fn(env: *mut JNIEnv, strg: jstring, isCopy: *mut jboolean) -> *const jchar,
	pub ReleaseStringChars: extern "C" fn(env: *mut JNIEnv, strg: jstring, chars: *const jchar),

	pub NewStringUTF:          extern "C" fn(env: *mut JNIEnv, utf: *const libc::c_char) -> jstring,
	pub GetStringUTFLength:    extern "C" fn(env: *mut JNIEnv, strg: jstring) -> jsize,
	pub GetStringUTFChars:     extern "C" fn(env: *mut JNIEnv, strg: jstring, isCopy: *mut jboolean) -> *const libc::c_char,
	pub ReleaseStringUTFChars: extern "C" fn(env: *mut JNIEnv, strg: jstring, chars: *const libc::c_char),

	pub GetArrayLength: extern "C" fn(env: *mut JNIEnv, array: jarray) -> jsize,

	pub NewObjectArray:        extern "C" fn(env: *mut JNIEnv, len: jsize, class: jclass, init: jobject) -> jobjectArray,
	pub GetObjectArrayElement: extern "C" fn(env: *mut JNIEnv, array: jobjectArray, index: jsize) -> jobject,
	pub SetObjectArrayElement: extern "C" fn(env: *mut JNIEnv, array: jobjectArray, index: jsize, val: jobject),

	pub NewBooleanArray: extern "C" fn(env: *mut JNIEnv, len: jsize) -> jbooleanArray,
	pub NewByteArray:    extern "C" fn(env: *mut JNIEnv, len: jsize) -> jbyteArray,
	pub NewCharArray:    extern "C" fn(env: *mut JNIEnv, len: jsize) -> jcharArray,
	pub NewShortArray:   extern "C" fn(env: *mut JNIEnv, len: jsize) -> jshortArray,
	pub NewIntArray:     extern "C" fn(env: *mut JNIEnv, len: jsize) -> jintArray,
	pub NewLongArray:    extern "C" fn(env: *mut JNIEnv, len: jsize) -> jlongArray,
	pub NewFloatArray:   extern "C" fn(env: *mut JNIEnv, len: jsize) -> jfloatArray,
	pub NewDoubleArray:  extern "C" fn(env: *mut JNIEnv, len: jsize) -> jdoubleArray,

	pub GetBooleanArrayElements: extern "C" fn(env: *mut JNIEnv, array: jbooleanArray,   isCopy: *mut jboolean) -> *mut jboolean,
	pub GetByteArrayElements:    extern "C" fn(env: *mut JNIEnv, array: jbyteArray, isCopy: *mut jboolean) -> *mut jbyte,
	pub GetCharArrayElements:    extern "C" fn(env: *mut JNIEnv, array: jcharArray, isCopy: *mut jboolean) -> *mut jchar,
	pub GetShortArrayElements:   extern "C" fn(env: *mut JNIEnv, array: jshortArray, isCopy: *mut jboolean) -> *mut jshort,
	pub GetIntArrayElements:     extern "C" fn(env: *mut JNIEnv, array: jintArray, isCopy: *mut jboolean) -> *mut jint,
	pub GetLongArrayElements:    extern "C" fn(env: *mut JNIEnv, array: jlongArray, isCopy: *mut jboolean) -> *mut jlong,
	pub GetFloatArrayElements:   extern "C" fn(env: *mut JNIEnv, array: jfloatArray, isCopy: *mut jboolean) -> *mut jfloat,
	pub GetDoubleArrayElements:  extern "C" fn(env: *mut JNIEnv, array: jdoubleArray, isCopy: *mut jboolean) -> *mut jdouble,

	pub ReleaseBooleanArrayElements: extern "C" fn(env: *mut JNIEnv, array: jbooleanArray, elems: *mut jboolean, mode: JNIReleaseArrayElementsMode),
	pub ReleaseByteArrayElements:    extern "C" fn(env: *mut JNIEnv, array: jbyteArray, elems: *mut jbyte, mode: JNIReleaseArrayElementsMode),
	pub ReleaseCharArrayElements:    extern "C" fn(env: *mut JNIEnv, array: jcharArray, elems: *mut jchar, mode: JNIReleaseArrayElementsMode),
	pub ReleaseShortArrayElements:   extern "C" fn(env: *mut JNIEnv, array: jshortArray, elems: *mut jshort, mode: JNIReleaseArrayElementsMode),
	pub ReleaseIntArrayElements:     extern "C" fn(env: *mut JNIEnv, array: jintArray, elems: *mut jint, mode: JNIReleaseArrayElementsMode),
	pub ReleaseLongArrayElements:    extern "C" fn(env: *mut JNIEnv, array: jlongArray, elems: *mut jlong, mode: JNIReleaseArrayElementsMode),
	pub ReleaseFloatArrayElements:   extern "C" fn(env: *mut JNIEnv, array: jfloatArray, elems: *mut jfloat, mode: JNIReleaseArrayElementsMode),
	pub ReleaseDoubleArrayElements:  extern "C" fn(env: *mut JNIEnv, array: jdoubleArray, elems: *mut jdouble, mode: JNIReleaseArrayElementsMode),

	pub GetBooleanArrayRegion: extern "C" fn(env: *mut JNIEnv, array: jbooleanArray, start: jsize, l: jsize, buf: *mut jboolean),
	pub GetByteArrayRegion:    extern "C" fn(env: *mut JNIEnv, array: jbyteArray, start: jsize, l: jsize, buf: *mut jbyte),
	pub GetCharArrayRegion:    extern "C" fn(env: *mut JNIEnv, array: jcharArray, start: jsize, l: jsize, buf: *mut jchar),
	pub GetShortArrayRegion:   extern "C" fn(env: *mut JNIEnv, array: jshortArray, start: jsize, l: jsize, buf: *mut jshort),
	pub GetIntArrayRegion:     extern "C" fn(env: *mut JNIEnv, array: jintArray, start: jsize, l: jsize, buf: *mut jint),
	pub GetLongArrayRegion:    extern "C" fn(env: *mut JNIEnv, array: jlongArray, start: jsize, l: jsize, buf: *mut jlong),
	pub GetFloatArrayRegion:   extern "C" fn(env: *mut JNIEnv, array: jfloatArray, start: jsize, l: jsize, buf: *mut jfloat),
	pub GetDoubleArrayRegion:  extern "C" fn(env: *mut JNIEnv, array: jdoubleArray, start: jsize, l: jsize, buf: *mut jdouble),

	pub SetBooleanArrayRegion: extern "C" fn(env: *mut JNIEnv, array: jbooleanArray, start: jsize, l: jsize, buf: *const jboolean),
	pub SetByteArrayRegion:    extern "C" fn(env: *mut JNIEnv, array: jbyteArray, start: jsize, l: jsize, buf: *const jbyte),
	pub SetCharArrayRegion:    extern "C" fn(env: *mut JNIEnv, array: jcharArray, start: jsize, l: jsize, buf: *const jchar),
	pub SetShortArrayRegion:   extern "C" fn(env: *mut JNIEnv, array: jshortArray, start: jsize, l: jsize, buf: *const jshort),
	pub SetIntArrayRegion:     extern "C" fn(env: *mut JNIEnv, array: jintArray, start: jsize, l: jsize, buf: *const jint),
	pub SetLongArrayRegion:    extern "C" fn(env: *mut JNIEnv, array: jlongArray, start: jsize, l: jsize, buf: *const jlong),
	pub SetFloatArrayRegion:   extern "C" fn(env: *mut JNIEnv, array: jfloatArray, start: jsize, l: jsize, buf: *const jfloat),
	pub SetDoubleArrayRegion:  extern "C" fn(env: *mut JNIEnv, array: jdoubleArray, start: jsize, l: jsize, buf: *const jdouble),

	pub RegisterNatives:   extern "C" fn(env: *mut JNIEnv, class: jclass, methods: *const JNINativeMethod, nMethods: jint) -> JNIError,
	pub UnregisterNatives: extern "C" fn(env: *mut JNIEnv, class: jclass) -> JNIError,

	pub MonitorEnter: extern "C" fn(env: *mut JNIEnv, obj: jobject) -> JNIError,
	pub MonitorExit:  extern "C" fn(env: *mut JNIEnv, obj: jobject) -> JNIError,

	pub GetJavaVM: extern "C" fn(env: *mut JNIEnv, vm: *mut *mut JavaVM) -> JNIError,

	pub GetStringRegion:    extern "C" fn(env: *mut JNIEnv, st: jstring, start: jsize, len: jsize, buf: *mut jchar),
	pub GetStringUTFRegion: extern "C" fn(env: *mut JNIEnv, st: jstring, start: jsize, len: jsize, buf: *mut libc::c_char),

	pub GetPrimitiveArrayCritical:     extern "C" fn(env: *mut JNIEnv, array: jarray, isCopy: *mut jboolean),
	pub ReleasePrimitiveArrayCritical: extern "C" fn(env: *mut JNIEnv, array: jarray, carray: *mut jvoid, mode: JNIReleaseArrayElementsMode),

	pub GetStringCritical:     extern "C" fn(env: *mut JNIEnv, string: jstring, isCopy: *mut jboolean) -> *const jchar,
	pub ReleaseStringCritical: extern "C" fn(env: *mut JNIEnv, string: jstring, cstring: *const jchar),

	pub NewWeakGlobalRef:    extern "C" fn(env: *mut JNIEnv, rf: jobject) -> jweak,
	pub DeleteWeakGlobalRef: extern "C" fn(env: *mut JNIEnv, rf: jweak),

	pub ExceptionCheck: extern "C" fn(env: *mut JNIEnv) -> jboolean,

	pub NewDirectByteBuffer:     extern "C" fn(env: *mut JNIEnv, address: *mut jvoid, capacity: jlong) -> jobject,
	pub GetDirectBufferAddress:  extern "C" fn(env: *mut JNIEnv, buf: jobject) -> *mut jvoid,
	pub GetDirectBufferCapacity: extern "C" fn(env: *mut JNIEnv, buf: jobject) -> jlong,

	pub GetObjectRefType: extern "C" fn(env: *mut JNIEnv, obj: jobject) -> jobjectRefType,
}

// Link to the JavaVM framework on OSX, and to the jvm library on everything
// else
#[cfg_attr(target_os = "macos", link(name = "JavaVM", kind = "framework"))]
#[cfg_attr(not(target_os = "macos"), link(name = "jvm"))]
extern "C" {
	pub fn JNI_CreateJavaVM(vm: *mut *mut JavaVM, env: *mut *mut JNIEnv, args: *mut JavaVMInitArgs) -> JNIError;
	pub fn JNI_GetDefaultJavaVMInitArgs(args: *mut JavaVMInitArgs) -> JNIError;
	pub fn JNI_GetCreatedJavaVMs(vm: *mut *mut JavaVM, bufLen: jsize, nVMs: *mut jsize) -> JNIError;
}
