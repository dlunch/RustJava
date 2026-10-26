// Rebuild from the repository root with JDK 22 or newer:
// classes=$(mktemp -d)
// javac --release 22 -d "$classes" test-data/src/special-resolution/SpecialResolutionErrors.java
// javac --release 22 -cp "$classes" -d "$classes" test-data/src/special-resolution/RuntimeTargets.java
// jar --create --file test-data/SpecialResolutionErrors.jar --main-class SpecialResolutionErrors -C "$classes" .
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
}

class ConstructorBase {
    ConstructorBase() {}
    ConstructorBase(int value) {}
}

class ConstructorChild extends ConstructorBase {
    ConstructorChild(int value) { super(value); }
}

class StaticBase {
    int method() { return 1; }
}

class StaticParent extends StaticBase {}

class StaticChild extends StaticParent {
    int call() { return super.method(); }
}

class AbstractBase {
    int method() { return 1; }
}

class AbstractParent extends AbstractBase {}

class AbstractChild extends AbstractParent {
    int method() { return 2; }
    int call() { return super.method(); }
}
