class ClinitThrowsError {
    static int x;

    static {
        x = 1;
        if (x == 1) {
            throw new LinkageError("boom");
        }
    }

    static void touch() {}
}
