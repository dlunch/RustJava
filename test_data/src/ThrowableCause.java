public class ThrowableCause {
    public static void main(String[] args) {
        Throwable inner = new IllegalArgumentException("inner");

        Throwable withMsg = new RuntimeException("outer", inner);
        System.out.println(withMsg);
        System.out.println(withMsg.getCause());
        System.out.println(withMsg.getCause() == inner);

        Throwable causeOnly = new RuntimeException(inner);
        System.out.println(causeOnly);
        System.out.println(causeOnly.getCause() == inner);

        Throwable bare = new java.lang.Exception("bare");
        System.out.println(bare.getCause());

        Throwable chained = new java.lang.Exception("chained");
        chained.initCause(inner);
        System.out.println(chained.getCause() == inner);
    }
}
