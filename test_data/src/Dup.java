class Dup {
    int temp;
    Integer[] int_array;

    public static void main(String[] args) {
        new Dup();
    }

    public Dup() {
        this.temp = 0;
        this.int_array = new Integer[10];

        int_array[0] = 112344;
        System.out.println(int_array[0]);

        System.out.println(test(int_array, 0, 123));
        System.out.println(this.temp ++);
    }

    public static int test(Integer[] array, int i, int j) {
        return array[i] = j;
    }
}
