class Interface {

    static interface IInterface {
        void test();
    }

    static class Implementation implements IInterface {
        public void test() {
            System.out.println("Implementation");
        }
    }

    public static void main(String[] args) {
        IInterface b = new Implementation();
        b.test();
    }
}
