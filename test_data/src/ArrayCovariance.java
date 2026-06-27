public class ArrayCovariance {
    public static void main(String[] args) {
        Object strArr = new String[1];
        System.out.println(strArr instanceof Object[]);
        System.out.println(strArr instanceof Object);
        System.out.println(strArr instanceof Cloneable);
        System.out.println(strArr instanceof java.io.Serializable);

        Object intArr = new int[1];
        System.out.println(intArr instanceof Object);
        System.out.println(intArr instanceof Object[]);

        Object nested = new String[1][1];
        System.out.println(nested instanceof Object[]);
        System.out.println(nested instanceof Object[][]);
        System.out.println(nested instanceof String[][]);

        Object[] casted = (Object[]) strArr;
        System.out.println(casted != null);
        try {
            String[] bad = (String[]) (Object) new Object[1];
            System.out.println("no-cce");
        } catch (ClassCastException e) {
            System.out.println("cce");
        }
    }
}
