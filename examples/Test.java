
//
//  Test Class
//

// We compile this class for the version of the JVM we're using, which is
// specified in the C interface code as JVM_VERSION_1_6 (ie. JVM 1.6).
//
// Thus we need to compile this file so it is compatible with this version.
// We do this by specifying the -source and -target arguments to javac:
//
//   javac -source 1.6 -target 1.6 Test.java

public class Test {
	public int current = 0;

	public Test(int current) {
		this.current = current;
	}

	public void incrementCurrent() {
		this.current += 1;
	}

	public int getCurrent() {
		return this.current;
	}

	public static int add(int a, int b) {
		return a + b;
	}

	public static String append(String input) {
		return input + " there from java";
	}
}
