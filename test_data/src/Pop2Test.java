class Pop2Test {
    public static void main(String[] args) {
        // pop2 on two category-1 values
        // Using method calls that return values we discard
        System.out.println("pop2:");

        // This exercises pop2 via discarding return values
        // long return (category-2) - pop2 pops one value
        returnLong();

        // double return (category-2) - pop2 pops one value
        returnDouble();

        // Verify stack is clean by doing normal operations after
        int a = 10;
        int b = 20;
        System.out.println(a + b);

        long c = 100L;
        long d = 200L;
        System.out.println(c + d);
    }

    static long returnLong() {
        return 42L;
    }

    static double returnDouble() {
        return 3.14;
    }
}
