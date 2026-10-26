class Abstract {

    static abstract class Base {
        Base() {}

        Base(int value) {}

        abstract void test();
    }

    static class Derived extends Base {
        void test() {
            System.out.println("Derived");
        }
    }

    public static void main(String[] args) {
        Base b = new Derived();
        b.test();
    }
}
