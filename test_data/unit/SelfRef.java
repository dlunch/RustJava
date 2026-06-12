class SelfRef {
    static int a = peek();
    static int b = 41;

    static int peek() {
        return b + 1;
    }

    static void touch() {}
}
