
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
