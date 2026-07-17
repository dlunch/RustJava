public class NativeMethod {
    private native void missing();

    public static void main(String[] args) {
        try {
            new NativeMethod().missing();
        } catch (UnsatisfiedLinkError expected) {
            System.out.println("unsupported");
        }
    }
}
