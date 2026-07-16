public class ForName {
    public static void main(String[] args) throws Exception {
        System.out.println(Class.forName("java.util.Vector").getName());
        try {
            Class.forName("no.such.Clazz");
            System.out.println("no-ex");
        } catch (ClassNotFoundException e) {
            System.out.println("cnfe");
        }
    }
}
