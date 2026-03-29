class IntegerOverflow {
    public static void main(String[] args) {
        // idiv: MIN_VALUE / -1 = MIN_VALUE (no exception)
        System.out.println("idiv:");
        System.out.println(Integer.MIN_VALUE / -1);
        System.out.println(10 / 3);

        // idiv: division by zero
        System.out.println("idiv zero:");
        try {
            int a = 1;
            int b = 0;
            int c = a / b;
            System.out.println("should not reach");
        } catch (ArithmeticException e) {
            System.out.println("caught");
        }

        // irem: MIN_VALUE % -1 = 0
        System.out.println("irem:");
        System.out.println(Integer.MIN_VALUE % -1);
        System.out.println(10 % 3);

        // irem: division by zero
        System.out.println("irem zero:");
        try {
            int a = 1;
            int b = 0;
            int c = a % b;
            System.out.println("should not reach");
        } catch (ArithmeticException e) {
            System.out.println("caught");
        }

        // ldiv: LONG_MIN / -1 = LONG_MIN
        System.out.println("ldiv:");
        System.out.println(Long.MIN_VALUE / -1L);

        // ldiv: division by zero
        System.out.println("ldiv zero:");
        try {
            long a = 1L;
            long b = 0L;
            long c = a / b;
            System.out.println("should not reach");
        } catch (ArithmeticException e) {
            System.out.println("caught");
        }

        // lrem: LONG_MIN % -1 = 0
        System.out.println("lrem:");
        System.out.println(Long.MIN_VALUE % -1L);

        // lrem: division by zero
        System.out.println("lrem zero:");
        try {
            long a = 1L;
            long b = 0L;
            long c = a % b;
            System.out.println("should not reach");
        } catch (ArithmeticException e) {
            System.out.println("caught");
        }

        // ineg: -MIN_VALUE wraps to MIN_VALUE
        System.out.println("ineg:");
        System.out.println(-Integer.MIN_VALUE);

        // lneg: -LONG_MIN wraps to LONG_MIN
        System.out.println("lneg:");
        System.out.println(-Long.MIN_VALUE);

        // ladd: overflow wraps
        System.out.println("ladd:");
        System.out.println(Long.MAX_VALUE + 1L);

        // lmul: overflow wraps
        System.out.println("lmul:");
        System.out.println(Long.MAX_VALUE * 2L);

        // lsub: overflow wraps
        System.out.println("lsub:");
        System.out.println(Long.MIN_VALUE - 1L);

        // iinc: overflow wraps
        System.out.println("iinc:");
        int x = Integer.MAX_VALUE;
        x += 1;  // iinc or iadd
        System.out.println(x);
    }
}
