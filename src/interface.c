
//
//  Interface wrapper around JNI
//

#include <jni.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#define TYPE_BYTE		0
#define TYPE_SHORT	1
#define TYPE_INT		2
#define TYPE_LONG		3
#define TYPE_FLOAT	4
#define TYPE_DOUBLE	5
#define TYPE_BOOLEAN	6
#define TYPE_CHAR		7
#define TYPE_STRING	8
#define TYPE_VOID		9


JavaVM *jvm = NULL;
JNIEnv *env = NULL;


/// Creates the Java virtual machine.
/// Returns 1 on success and 0 on failure.
int create_jvm(char *classpath) {
	if (jvm == NULL) {
		JavaVMOption options[1];
		JavaVMInitArgs vm_args;

		char destination[200];
		strcpy(destination, "-Djava.class.path=");
		strcat(destination, classpath);
		options[0].optionString = destination;

		vm_args.version = JNI_VERSION_1_2;
		vm_args.nOptions = 1;
		vm_args.options = options;
		long status = JNI_CreateJavaVM(&jvm, (void **) &env, &vm_args);

		if (status != JNI_OK) {
			return 0;
		}
	}

	return 1;
}


/// Destroys the Java VM once its finished being used.
void destroy_jvm(void) {
	(*jvm)->DestroyJavaVM(jvm);
}


/// Returns the class from the class name.
/// Returns NULL on failure.
void * class_from_name(char *name) {
	jclass java_class = (*env)->FindClass(env, name);
	if (java_class != NULL) {
		return (void *) java_class;
	}

	return NULL;
}


/// Converts a void pointer and type into a JNI value.
jvalue value_to_jni_value(int type, void *content) {
	jvalue result;

	if (type == TYPE_BYTE) {
		result.b = *((jbyte *) content);
	} else if (type == TYPE_SHORT) {
		result.s = *((jshort *) content);
	} else if (type == TYPE_INT) {
		result.i = *((jint *) content);
	} else if (type == TYPE_LONG) {
		result.j = *((jlong *) content);
	} else if (type == TYPE_FLOAT) {
		result.f = *((jfloat *) content);
	} else if (type == TYPE_DOUBLE) {
		result.d = *((jdouble *) content);
	} else if (type == TYPE_BOOLEAN) {
		result.z = *((jboolean *) content);
	} else if (type == TYPE_CHAR) {
		result.c = *((jchar *) content);
	} else if (type == TYPE_STRING) {
		result.l = (*env)->NewStringUTF(env, (char *) content);
	}

	return result;
}


/// Convert a list of arguments into an array of JNI values.
/// Note: Return value must be freed!
jvalue * args_to_jni_args(int count, int *types, void **values) {
	jvalue *args = malloc(sizeof(jvalue) * count);
	if (args == NULL) {
		return NULL;
	}

	for (int i = 0; i < count; i++) {
		int type = types[i];
		void *value = values[i];
		if (value != NULL && type != TYPE_VOID) {
			args[i] = value_to_jni_value(type, value);
		}

		free(value);
	}

	return args;
}


/// Calls a static method on a class.
/// Returns NULL on failure.
void * call_static_method(void *java_class, char *name, char *signature, int return_type,
		int arg_count, int *arg_types, void **arg_values) {
	// Validate class
	jclass cast_class = (jclass) java_class;
	if (cast_class == NULL) {
		return NULL;
	}

	// Get method ID
	jmethodID method_id = (*env)->GetStaticMethodID(env, cast_class, name, signature);
	if (method_id == NULL) {
		return NULL;
	}

	jvalue *args = args_to_jni_args(arg_count, arg_types, arg_values);
	if (args == NULL) {
		return NULL;
	}

	void *result = NULL;
	if (return_type != TYPE_VOID && return_type != TYPE_STRING) {
		result = malloc(sizeof(jvalue));
		if (result == NULL) {
			return NULL;
		}

		// Call the method
		if (return_type == TYPE_BYTE) {
			((jvalue *) result)->b =
				(*env)->CallStaticByteMethodA(env, cast_class, method_id, args);
		} else if (return_type == TYPE_SHORT) {
			((jvalue *) result)->s =
				(*env)->CallStaticShortMethodA(env, cast_class, method_id, args);
		} else if (return_type == TYPE_INT) {
			((jvalue *) result)->i =
				(*env)->CallStaticIntMethodA(env, cast_class, method_id, args);
		} else if (return_type == TYPE_LONG) {
			((jvalue *) result)->j =
				(*env)->CallStaticLongMethodA(env, cast_class, method_id, args);
		} else if (return_type == TYPE_FLOAT) {
			((jvalue *) result)->f =
				(*env)->CallStaticFloatMethodA(env, cast_class, method_id, args);
		} else if (return_type == TYPE_DOUBLE) {
			((jvalue *) result)->d =
				(*env)->CallStaticDoubleMethodA(env, cast_class, method_id, args);
		} else if (return_type == TYPE_BOOLEAN) {
			((jvalue *) result)->z =
				(*env)->CallStaticBooleanMethodA(env, cast_class, method_id, args);
		} else if (return_type == TYPE_CHAR) {
			((jvalue *) result)->c =
				(*env)->CallStaticCharMethodA(env, cast_class, method_id, args);
		}
	} else if (return_type == TYPE_STRING) {
		jstring value = (*env)->CallStaticObjectMethodA(env, cast_class, method_id, args);

		// Allocate the results buffer
		int size = (*env)->GetStringUTFLength(env, value);
		result = malloc(sizeof(char) * (size + 1));

		// Get the string and copy it into the results buffer
		const char *str = (*env)->GetStringUTFChars(env, value, NULL);
		memcpy(result, str, size + 1);

		// Free the Java string
		(*env)->ReleaseStringUTFChars(env, value, str);
	} else {
		(*env)->CallStaticVoidMethodA(env, cast_class, method_id, args);
	}

	free(args);
	return result;
}


/// Creates a new instance of a class.
/// The arguments relate to the arguments passed to the constructor.
void * create_object(void *java_class, char *signature, int arg_count, int *arg_types,
		void **arg_values) {
	// Validate class
	jclass cast_class = (jclass) java_class;
	if (cast_class == NULL) {
		return NULL;
	}

	// Get constructor method ID
	jmethodID constructor_id = (*env)->GetMethodID(env, cast_class, "<init>", signature);
	if (constructor_id == NULL) {
		return NULL;
	}

	jvalue *args = args_to_jni_args(arg_count, arg_types, arg_values);
	if (args == NULL) {
		return NULL;
	}

	jobject instance = (*env)->NewObjectA(env, cast_class, constructor_id, args);

	free(args);
	return (void *) instance;
}


/// Calls a method on a class.
void * call_method(void *java_object, char *name, char *signature, int return_type,
		int arg_count, int *arg_types, void **arg_values) {
	// Validate object
	jobject cast_object = (jobject) java_object;
	if (cast_object == NULL) {
		return NULL;
	}

	// Get class
	jclass object_class = (*env)->GetObjectClass(env, cast_object);
	if (object_class == NULL) {
		return NULL;
	}

	// Get method ID
	jmethodID method_id = (*env)->GetMethodID(env, object_class, name, signature);
	if (method_id == NULL) {
		return NULL;
	}

	jvalue *args = args_to_jni_args(arg_count, arg_types, arg_values);
	if (args == NULL) {
		return NULL;
	}

	jvalue *result = NULL;
	if (return_type != TYPE_VOID) {
		result = malloc(sizeof(jvalue));
		if (result == NULL) {
			return NULL;
		}

		// Call the method
		if (return_type == TYPE_BYTE) {
			result->b = (*env)->CallByteMethodA(env, cast_object, method_id, args);
		} else if (return_type == TYPE_SHORT) {
			result->s = (*env)->CallShortMethodA(env, cast_object, method_id, args);
		} else if (return_type == TYPE_INT) {
			result->i = (*env)->CallIntMethodA(env, cast_object, method_id, args);
		} else if (return_type == TYPE_LONG) {
			result->j = (*env)->CallLongMethodA(env, cast_object, method_id, args);
		} else if (return_type == TYPE_FLOAT) {
			result->f = (*env)->CallFloatMethodA(env, cast_object, method_id, args);
		} else if (return_type == TYPE_DOUBLE) {
			result->d = (*env)->CallDoubleMethodA(env, cast_object, method_id, args);
		} else if (return_type == TYPE_BOOLEAN) {
			result->z = (*env)->CallBooleanMethodA(env, cast_object, method_id, args);
		} else if (return_type == TYPE_CHAR) {
			result->c = (*env)->CallCharMethodA(env, cast_object, method_id, args);
		} else if (return_type == TYPE_STRING) {
			result->l = (*env)->CallObjectMethodA(env, cast_object, method_id, args);
		}
	} else {
		(*env)->CallVoidMethodA(env, cast_object, method_id, args);
	}

	free(args);
	return result;
}
