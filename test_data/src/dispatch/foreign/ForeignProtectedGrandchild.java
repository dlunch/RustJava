package dispatch.foreign;

import dispatch.base.ProtectedBridge;

public class ForeignProtectedGrandchild extends ProtectedBridge {
    @Override
    public String value() {
        return "protected-transitive";
    }
}
