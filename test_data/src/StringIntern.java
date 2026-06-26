public class StringIntern {
    public static void main(String[] args) {
        String a = "x";
        String b = "x";
        System.out.println(a == b);

        String c = new String("x");
        System.out.println(c == a);
        System.out.println(c.intern() == a);
    }
}
