// Replace only these targets after compiling SpecialResolutionErrors.java.
// The stale callers must report linkage errors rather than use inherited
// constructors, invoke static methods, or execute abstract methods.
class ConstructorChild extends ConstructorBase {
    ConstructorChild() {}
}

class StaticBase {
    static int method() { return 1; }
}

abstract class AbstractBase {
    abstract int method();
}
