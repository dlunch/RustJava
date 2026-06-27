public class ArrayStore {
    public static void main(String[] args) {
        Object[] a = new String[2];
        a[0] = "ok";
        System.out.println("stored-ok");
        try {
            a[1] = Integer.valueOf(1);
            System.out.println("no-ase");
        } catch (ArrayStoreException e) {
            System.out.println("ase");
        }

        Object[] o = new Object[2];
        o[0] = Integer.valueOf(1);
        o[1] = "any";
        System.out.println("object-array-ok");

        // nested: String[][] holding String[] is fine, holding Integer[] is not
        Object[] m = new String[1][];
        m[0] = new String[3];
        System.out.println("nested-ok");
        try {
            m[0] = new Integer[3];
            System.out.println("no-ase2");
        } catch (ArrayStoreException e) {
            System.out.println("ase2");
        }
    }
}
