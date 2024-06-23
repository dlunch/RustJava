class OddEven {
    public static void main(String[] args) {
        OddEven oe = new OddEven();

        System.out.println(oe.run("1234"));
        System.out.println(oe.run("1233"));
    }

    String run(String arg) {
        int i = Integer.parseInt(arg);

        if (i % 2 == 1) {
            return "i is odd";
        } else {
            return "i is even";
        }
    }
}
