public class StaticInherit {
    static class Base {
        static int field = 7;

        static int method() {
            return 9;
        }
    }

    static class Sub extends Base {
    }

    static int compute() {
        return 11;
    }

    interface IConst {
        int IC = compute();
    }

    static class Impl implements IConst {
    }

    public static void main(String[] args) {
        System.out.println(Sub.field);
        System.out.println(Sub.method());
        System.out.println(Impl.IC);
    }
}
