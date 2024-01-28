class Array {
    public static void main(String[] args) {
        int[] int_array = new int[10];
        byte[] byte_array = new byte[10];
        short[] short_array = new short[10];
        long[] long_array = new long[10];
        char[] char_array = new char[10];
        boolean[] boolean_array = new boolean[10];
        String[] string_array = new String[10];

        int_array[0] = 112344;
        byte_array[0] = 123;
        short_array[0] = 12345;
        long_array[0] = 1123412341234L;
        char_array[0] = '가';
        boolean_array[0] = true;
        string_array[0] = "test한글";

        System.out.println(int_array[0]);
        System.out.println(byte_array[0]);
        System.out.println(short_array[0]);
        System.out.println(long_array[0]);
        System.out.println(char_array[0]);
        System.out.println(boolean_array[0]);
        System.out.println(string_array[0]);

        System.out.println(int_array.length);
        System.out.println(byte_array.length);
        System.out.println(short_array.length);
        System.out.println(long_array.length);
        System.out.println(char_array.length);
        System.out.println(boolean_array.length);
        System.out.println(string_array.length);
    }
}
