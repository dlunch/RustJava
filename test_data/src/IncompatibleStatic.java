// IncompatibleStatic is compiled against a version of StaleHolder that declares `shared`
// as a static field (so javac emits getstatic), while the committed StaleHolder.class
// declares it as an instance field. A stale client hitting getstatic on a now-instance
// field must fail with IncompatibleClassChangeError (JVMS 5.4.3.2 / getstatic).
public class IncompatibleStatic {
    public static void main(String[] args) {
        try {
            System.out.println(StaleHolder.shared);
        } catch (Throwable t) {
            System.out.println(t.getClass().getName());
        }
    }
}
