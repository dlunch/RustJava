package dispatch.foreign;

import dispatch.base.PackageMiddle;

public class ForeignChild extends PackageMiddle {
    public void hook() {
        System.out.println("foreign");
    }
}
