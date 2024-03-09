import java.io.OutputStream;
import java.io.PrintStream;

class Instanceof {
    public static void main(String[] args) {
        Instanceof a = new Instanceof();

        if (a instanceof Object) {
            System.out.println("a is an Object");
        } else {
            System.out.println("a is not an Object");
        }

        if (System.out instanceof PrintStream) {
            System.out.println("System.out is a PrintStream");
        } else {
            System.out.println("System.out is not a PrintStream");
        }

        if (System.out instanceof OutputStream) {
            System.out.println("System.out is an OutputStream");
        } else {
            System.out.println("System.out is not an OutputStream");
        }

        if (System.out instanceof Object) {
            System.out.println("System.out is an Object");
        } else {
            System.out.println("System.out is not an Object");
        }

        if (args instanceof Object) {
            System.out.println("args is an Object");
        } else {
            System.out.println("args is not an Object");
        }

        if (args instanceof String[]) {
            System.out.println("args is a String[]");
        } else {
            System.out.println("args is not a String[]");
        }

        Object b = a;
        if (b instanceof Instanceof) {
            System.out.println("b is an Instanceof");
        } else {
            System.out.println("b is not an Instanceof");
        }
    }
}
