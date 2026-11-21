public class SuperclassInitAllocatesSubclass {
    static class Parent {
        // runs while Child is still being initialized, because initializing Child initializes Parent first
        static Child early = new Child();
        int inherited = 1;
    }

    static class Child extends Parent {
        static final String NAME = "child";
        int own = 2;
    }

    public static void main(String[] args) {
        Child child = new Child();
        System.out.println(child.inherited + child.own);
        System.out.println(Parent.early.inherited + Parent.early.own);
        Parent.early.own = 5;
        System.out.println(Parent.early.own);
        System.out.println(Child.NAME);
    }
}
