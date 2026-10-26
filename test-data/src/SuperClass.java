class SuperClass {
    interface BaseDefault {
        default int value() { return 1; }
    }

    interface SpecificDefault extends BaseDefault {
        default int value() { return 2; }
    }

    interface PrivateDefault {
        private int value() { return 3; }
    }

    static class DefaultParent implements SpecificDefault, BaseDefault, PrivateDefault {}

    static class DefaultChild extends DefaultParent {
        int call() { return super.value(); }
    }

    static class ReverseDefaultParent implements BaseDefault, SpecificDefault, PrivateDefault {}

    static class ReverseDefaultChild extends ReverseDefaultParent {
        int call() { return super.value(); }
    }

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

    class InnerLeafClass extends InnerDerivedClass {
        @Override
        void test() {
            System.out.println("leaf");
        }

        void testSuper() {
            super.test();
        }
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

        InnerLeafClass leaf = new InnerLeafClass();
        leaf.test();
        leaf.testSuper();

        System.out.println(new DefaultChild().call());
        System.out.println(new ReverseDefaultChild().call());
    }

    public static void main(String[] args) {
        new SuperClass().run();
    }
}
