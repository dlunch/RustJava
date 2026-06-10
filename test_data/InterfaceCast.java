public class InterfaceCast {
    interface IFace {
    }

    static class Base implements IFace {
    }

    static class Derived extends Base {
    }

    public static void main(String[] args) {
        Object value = new Derived();
        System.out.println(value instanceof IFace);

        IFace casted = (IFace) value;
        System.out.println(casted != null);
    }
}
