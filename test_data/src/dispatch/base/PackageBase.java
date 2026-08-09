package dispatch.base;

public class PackageBase {
    void hook() {
        System.out.println("base");
    }

    public void callHook() {
        hook();
    }
}
