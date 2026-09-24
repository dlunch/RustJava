class Field {
    static class Base {
        private boolean active = true;
        public int value = 11;
        public Object reference = "base";

        void clear() {
            active = false;
        }

        boolean baseActive() {
            return active;
        }
    }

    static class Derived extends Base {
        private boolean active = true;
        public int value = 22;
        public Object reference = "derived";

        boolean derivedActive() {
            return active;
        }
    }

    static class Leaf extends Derived {
    }

    public int int_field;
    public String string_field;
    public long long_field;

    public static int static_field;

    public static void main(String[] args) {
        Field.static_field = 1234;
        Field field = new Field();

        field.int_field = 1;
        field.string_field = "test1";

        System.out.println(field.int_field);
        System.out.println(field.string_field);
        System.out.println(field.static_field);

        Derived child = new Derived();
        Base parent = child;
        parent.clear();
        System.out.println(parent.baseActive());
        System.out.println(child.derivedActive());

        parent.value = 31;
        child.value = 42;
        parent.reference = "parent";
        child.reference = "child";
        System.out.println(parent.value);
        System.out.println(child.value);
        System.out.println(parent.reference);
        System.out.println(child.reference);

        Leaf leaf = new Leaf();
        System.out.println(leaf.value);
        System.out.println(((Base) leaf).value);
    }
}
