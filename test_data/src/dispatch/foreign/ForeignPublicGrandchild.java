package dispatch.foreign;

import dispatch.base.PublicBridge;

public class ForeignPublicGrandchild extends PublicBridge {
    @Override
    public String value() {
        return "public-transitive";
    }
}
