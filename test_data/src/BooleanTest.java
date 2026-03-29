class BooleanTest {
    static boolean boolField;

    public static void main(String[] args) {
        // Test boolean array store with low-order bit semantics
        // bastore to boolean[] should use value & 1
        boolean[] arr = new boolean[5];
        arr[0] = true;   // 1 -> true
        arr[1] = false;  // 0 -> false

        System.out.println("boolean array:");
        System.out.println(arr[0]);
        System.out.println(arr[1]);

        // Test boolean field store/load
        boolField = true;
        System.out.println("boolean field:");
        System.out.println(boolField);

        boolField = false;
        System.out.println(boolField);

        // Test boolean method parameter and return
        System.out.println("boolean method:");
        System.out.println(identity(true));
        System.out.println(identity(false));
        System.out.println(negate(true));
        System.out.println(negate(false));
    }

    static boolean identity(boolean v) {
        return v;
    }

    static boolean negate(boolean v) {
        return !v;
    }
}
