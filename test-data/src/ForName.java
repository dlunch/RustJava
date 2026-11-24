public class ForName extends ClassLoader {
    static class Target {
        static {
            System.out.println("initialized");
        }
    }

    static class Failing {
        static int value = Integer.parseInt("bad");
    }

    public static void main(String[] args) throws Exception {
        System.out.println(Class.forName("java.util.Vector").getName());
        try {
            Class.forName("no.such.Clazz");
            System.out.println("no-ex");
        } catch (ClassNotFoundException e) {
            System.out.println("cnfe");
        }

        String target = Target.class.getName();
        System.out.println(target);
        Class.forName(target);
        Class.forName(target);
        System.out.println("returned");

        for (int i = 0; i < 2; i++) {
            try {
                Class.forName("ForName$Failing");
            } catch (LinkageError e) {
                System.out.println(e.getClass().getName());
            }
        }
    }
}
