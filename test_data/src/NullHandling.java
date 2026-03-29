class NullHandling {
    int field;

    public static void main(String[] args) {
        // instanceof on null should return false (not throw)
        System.out.println("instanceof null:");
        Object obj = null;
        System.out.println(obj instanceof String);
        System.out.println(obj instanceof Object);

        // athrow null should throw NullPointerException
        System.out.println("athrow null:");
        try {
            throw null;
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        // getfield on null should throw NullPointerException
        System.out.println("getfield null:");
        try {
            NullHandling n = null;
            int v = n.field;
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        // putfield on null should throw NullPointerException
        System.out.println("putfield null:");
        try {
            NullHandling n = null;
            n.field = 42;
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }
    }
}
