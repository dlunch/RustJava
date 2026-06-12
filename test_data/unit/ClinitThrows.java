class ClinitThrows {
    static int x;

    static {
        x = 1;
        if (x == 1) {
            throw new RuntimeException("boom");
        }
    }

    static void touch() {}
}
