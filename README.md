
Java Native Interface Bindings for Rust
=======================================

This library provides complete FFI bindings to the Java Native Interface, as
well as a safe and intuitive wrapper around most these bindings (lacking array
support for now).

Features include:

* Creating and configuring an instance of a Java Virtual Machine
* Loading classes
* Calling static methods on classes
* Setting and retrieving public static fields on classes
* Instantiating objects from a class
* Calling methods on objects
* Setting and retrieving public fields on objects
* Using all primitive Java types and other Java objects as arguments and
  return values (no support for arrays yet)


### Documentation

Documentation can be found [here](https://benanderson.io/docs/rjni).


### Usage

First you'll need to compile your Java source code, either as separate `.class`
files, or package them together as a `.jar` archive.

You need to make sure you target the Java compiler to the JVM version you plan
to use. This is done through the `-target` and `-source` command line arguments
to `javac`.

For example, if you have a `/path/to/project/com/me/Test.java` file (ie. the
class `com.me.Test`) and you intend to target the 1.6 JVM:

```bash
$ javac -target 1.6 -source 1.6 /path/to/project/com/me/Test.java
```

This will create a `/path/to/project/com/me/Test.class` file.

Then when you create the JVM in Rust, you need to add `/path/to/project` (ie.
the directory containing the root of your Java code) to the classpath, and
specify the correct JVM version:

```rust
use rjni::{Jvm, Version, Classpath, Options};

fn main() {
	// Create a custom classpath, pointing to the directory containing the root
	// of your Java code
	let mut classpath = Classpath::new();
	classpath.add(&Path::new("/path/to/project"));

	// Create a series of configuration options for the JVM, specifying the
	// version of the JVM we want to use (1.6), and our custom classpath
	let mut options = Options::new();
	options.version(Version::V16);
	options.classpath(classpath);

	// Create the JVM with these options
	let jvm = Jvm::new(options).unwrap();

	// Get the `com.me.Test` class using the JVM
	let class = jvm.class("com/me/Test").unwrap();

	// ...
}
```

See the `examples` folder for more example code on how to call static methods
on classes, instantiate objects, call methods on objects, and access object
fields.
