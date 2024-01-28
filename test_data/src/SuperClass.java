class SuperClass {
    class InnerClass {
        int field1;
        long field2;

        void test() {
            System.out.println("test");
        }
    }

    class InnerDerivedClass extends InnerClass {
        int field3;
        String field4;
    }

    void run() {
        InnerDerivedClass derived = null;

        derived = new InnerDerivedClass();

        derived.field1 = 0;
        derived.field2 = 1;
        derived.field3 = 2;
        derived.field4 = "test";

        InnerClass inner = derived;

        inner.field1 = 2;
        inner.field2 = 1234123412341234L;

        System.out.println(derived.field1);
        System.out.println(derived.field2);
        System.out.println(derived.field3);
        System.out.println(derived.field4);

        derived.test();
    }

    public static void main(String[] args) {
        new SuperClass().run();
    }
}
