public class InterfaceCast {
    interface IBase {
    }

    interface IFace extends IBase {
    }

    static class Base implements IFace {
    }

    static class Derived extends Base {
    }

    public static void main(String[] args) {
        Object value = new Derived();
        System.out.println(value instanceof IFace);
        System.out.println(value instanceof IBase);

        IFace casted = (IFace) value;
        System.out.println(casted != null);
    }
}
