public class DoubleInit {
    static {
        System.out.println("init");
    }

    static void touch() {}

    public static void main(String[] args) {
        touch();
        touch();
    }
}
