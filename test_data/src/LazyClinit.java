public class LazyClinit {
    static class GetstaticTarget {
        static int x = 5;

        static {
            System.out.println("getstatic-init");
        }
    }

    static class PutstaticTarget {
        static int x;

        static {
            System.out.println("putstatic-init");
        }
    }

    static class NewTarget {
        static {
            System.out.println("new-init");
        }
    }

    static class Base {
        static {
            System.out.println("base-init");
        }
    }

    static class Derived extends Base {
        static {
            System.out.println("derived-init");
        }

        static void touch() {}
    }

    static int mark() {
        System.out.println("iface-init");
        return 1;
    }

    interface IFace {
        int X = mark();
    }

    static class Impl implements IFace {
    }

    static class SelfRef {
        static int a = peek();
        static int b = 41;

        static int peek() {
            return b + 1;
        }
    }

    public static void main(String[] args) {
        System.out.println("start");
        System.out.println(GetstaticTarget.x);
        PutstaticTarget.x = 9;
        System.out.println(PutstaticTarget.x);
        new NewTarget();
        Derived.touch();
        new Impl();
        System.out.println("impl-created");
        System.out.println(IFace.X);
        System.out.println(SelfRef.a);
        System.out.println(SelfRef.b);
    }
}
