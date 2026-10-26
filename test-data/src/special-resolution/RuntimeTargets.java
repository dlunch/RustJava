// Replace only these targets after compiling SpecialResolutionErrors.java.
// The stale callers must report linkage errors rather than use inherited
// constructors, invoke static methods, or execute abstract methods.
class SpecialResolutionErrors {
    static class ConstructorBase {
        ConstructorBase() {}
        ConstructorBase(int value) {}
    }

    static class ConstructorChild extends ConstructorBase {
        ConstructorChild() {}
    }

    static class StaticBase {
        static int method() { return 1; }
    }

    static abstract class AbstractBase {
        abstract int method();
    }
}
