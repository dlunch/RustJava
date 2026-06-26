public class StaticInitTrigger {
    static class Base {
        static int field = 7;

        static {
            System.out.println("base-init");
        }
    }

    static class Sub extends Base {
        static {
            System.out.println("sub-init");
        }
    }

    public static void main(String[] args) {
        System.out.println("start");
        // accessing an inherited static field initializes only the declaring class (Base), not Sub
        System.out.println(Sub.field);
        System.out.println("end");
    }
}
