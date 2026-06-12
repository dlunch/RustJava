class Source {
    static int own = 7;

    static {
        Target.value = 42;
    }

    static void touch() {}
}
