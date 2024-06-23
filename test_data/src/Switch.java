class Switch {
    public static void main(String[] args) {
        run("1");
        run("3");
        run("10");
        run("100");
    }

    static void run(String arg) {
        int a = Integer.parseInt(arg);

        switch(a) {
            case 1:
                System.out.println("1");
                break;
            case 2:
                System.out.println("2");
                break;
            case 3:
                System.out.println("3");
            case 4:
                System.out.println("4");
                break;
        }

        switch(a) {
            case 1:
                System.out.println("1");
                break;
            case 10:
                System.out.println("10");
                break;
            case 100:
                System.out.println("100");
                break;
            case 1000:
                System.out.println("1000");
                break;
        }
    }
}
