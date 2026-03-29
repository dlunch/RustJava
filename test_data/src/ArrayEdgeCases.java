class ArrayEdgeCases {
    public static void main(String[] args) {
        // Negative array size should throw NegativeArraySizeException
        System.out.println("negative size:");
        try {
            int[] a = new int[-1];
            System.out.println("should not reach");
        } catch (NegativeArraySizeException e) {
            System.out.println("caught");
        }

        try {
            Object[] a = new Object[-1];
            System.out.println("should not reach");
        } catch (NegativeArraySizeException e) {
            System.out.println("caught");
        }

        // Negative index should throw ArrayIndexOutOfBoundsException
        System.out.println("negative index:");
        try {
            int[] a = new int[5];
            a[-1] = 42;
            System.out.println("should not reach");
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("caught");
        }

        try {
            int[] a = new int[5];
            int v = a[-1];
            System.out.println("should not reach");
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("caught");
        }

        // 2D array (anewarray of array type)
        System.out.println("2d array:");
        String[][] arr2d = new String[2][3];
        arr2d[0][0] = "hello";
        arr2d[1][2] = "world";
        System.out.println(arr2d[0][0]);
        System.out.println(arr2d[1][2]);
        System.out.println(arr2d.length);
        System.out.println(arr2d[0].length);

        // multianewarray with negative dimension
        System.out.println("multi negative:");
        try {
            int[][] m = new int[-1][2];
            System.out.println("should not reach");
        } catch (NegativeArraySizeException e) {
            System.out.println("caught");
        }
    }
}
