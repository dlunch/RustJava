class MultiArray {
    public static void main(String[] args) {
        String[][][][][] array = new String[10][10][10][10][10];
        int[][] intArray = new int[2][3];

        array[0][0][0][0][0] = "test1";
        array[1][5][2][1][2] = "test2";
        array[5][5][5][5][5] = "test3";
        array[9][9][9][9][9] = "test4";

        intArray[1][2] = 123;
        intArray[0][0] = 456;

        System.out.println(array[0][0][0][0][0]);
        System.out.println(array[1][5][2][1][2]);
        System.out.println(array[5][5][5][5][5]);
        System.out.println(array[6][6][6][6][1]);
        System.out.println(array[9][9][9][9][9]);

        System.out.println(intArray[1][2]);
        System.out.println(intArray[0][0]);
    }
}
