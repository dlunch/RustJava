public class StringIntern {
    static final String CONST = "x";

    public static void main(String[] args) {
        String a = "x";
        String b = "x";
        System.out.println(a == b);

        String c = new String("x");
        System.out.println(c == a);
        System.out.println(c.intern() == a);

        // "zq" never appears as a literal, so intern() pools and returns the receiver itself
        String fresh = new String(new char[] { 'z', 'q' });
        System.out.println(fresh.intern() == fresh);

        // static final String constant is interned, equal to the literal
        System.out.println(CONST == "x");
    }
}
