class InitDerived extends InitBase {
    static {
        InitLog.derivedOrder = ++InitLog.counter;
    }

    static void touch() {}
}
