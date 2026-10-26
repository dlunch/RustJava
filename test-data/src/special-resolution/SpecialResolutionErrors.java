// Rebuild from the repository root with JDK 22 or newer:
// classes=$(mktemp -d)
// javac --release 22 -d test-data test-data/src/special-resolution/SpecialResolutionErrors.java
// javac --release 22 -d "$classes" test-data/src/special-resolution/RuntimeTargets.java
// cp "$classes"/'SpecialResolutionErrors$ConstructorChild.class' test-data/
// cp "$classes"/'SpecialResolutionErrors$StaticBase.class' test-data/
// cp "$classes"/'SpecialResolutionErrors$AbstractBase.class' test-data/
// rm -r "$classes"
public class SpecialResolutionErrors {
    public static void main(String[] args) {
        try {
            new ConstructorChild(1);
        } catch (NoSuchMethodError e) {
            System.out.println(e.getClass().getName());
        }
        try {
            new StaticChild().call();
        } catch (IncompatibleClassChangeError e) {
            System.out.println(e.getClass().getName());
        }
        try {
            new AbstractChild().call();
        } catch (AbstractMethodError e) {
            System.out.println(e.getClass().getName());
        }
    }
    static class ConstructorBase {
        ConstructorBase() {}
        ConstructorBase(int value) {}
    }

    static class ConstructorChild extends ConstructorBase {
        ConstructorChild(int value) { super(value); }
    }

    static class StaticBase {
        int method() { return 1; }
    }

    static class StaticParent extends StaticBase {}

    static class StaticChild extends StaticParent {
        int call() { return super.method(); }
    }

    static class AbstractBase {
        int method() { return 1; }
    }

    static class AbstractParent extends AbstractBase {}

    static class AbstractChild extends AbstractParent {
        int method() { return 2; }
        int call() { return super.method(); }
    }
}
