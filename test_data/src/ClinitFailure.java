public class ClinitFailure {
    static boolean fail = true;

    static class Bad {
        static {
            if (fail) {
                throw new RuntimeException("boom");
            }
        }

        static void touch() {}
    }

    static class BadError {
        static {
            if (fail) {
                throw new LinkageError("boom");
            }
        }

        static void touch() {}
    }

    public static void main(String[] args) {
        try {
            Bad.touch();
        } catch (Throwable t) {
            System.out.println(t.getClass().getName());
            System.out.println(t.getCause().getClass().getName());
            System.out.println(t.getCause());
            System.out.println(t);
        }
        try {
            Bad.touch();
        } catch (Throwable t) {
            System.out.println(t.getClass().getName());
        }
        try {
            BadError.touch();
        } catch (Throwable t) {
            System.out.println(t.getClass().getName());
        }
    }
}
