// compiled against a non-final Constants so javac emits getstatic instead of
// inlining the constants; run against the final Constants.class with ConstantValue attributes
public class ConstantsReader {
    public static void main(String[] args) {
        System.out.println(Constants.FLAG);
        System.out.println(Constants.B);
        System.out.println(Constants.C);
        System.out.println(Constants.S);
        System.out.println(Constants.I);
        System.out.println(Constants.L);
        System.out.println(Constants.STR);
    }
}
