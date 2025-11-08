public class CheckCast {
    public static void main(String[] args) {
        Object obj = "This is a string";

        try {
            String str = (String) obj; // This should succeed
            System.out.println("Successfully cast to String");
        } catch (ClassCastException e) {
            System.out.println("Failed to cast to String");
        }

        try {
            Integer num = (Integer) obj; // This should fail
            System.out.println("Successfully cast to Integer");
        } catch (ClassCastException e) {
            System.out.println("Failed to cast to Integer");
        }
    }
}
