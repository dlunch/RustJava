public class StaticOrder {
    static class Helper {
        static {
            System.out.println("helper-init");
        }

        static void touch() {}
    }

    public static void main(String[] args) {
        Class<?> c = Helper.class;
        System.out.println("before");
        Helper.touch();
    }
}
