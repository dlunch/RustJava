class ControlFlow {
    public static void main(String[] args) {
        int x = 10;
        while (x > 1) {
            if (x % 2 == 0) {
                x = x / 2;
            } else {
                x = x * 3 + 1;
            }
            System.out.println(x);
        }

        for(int i = 0; i < 10; i ++) {
            System.out.println(i);
        }

        ControlFlow a = new ControlFlow();
        if (a != null) {
            System.out.println("a is not null");
        } else if (a == new ControlFlow()) {
            System.out.println("a is new ControlFlow()");
        } else {
            System.out.println("a is not new ControlFlow()");
        }

        if (x < 10) {
            System.out.println("x < 10");
        } else if (x > 20) {
            System.out.println("x  20");
        } else if (x <= 25) {
            System.out.println("x <= 25");
        } else if (x == 30) {
            System.out.println("x == 30");
        } else if (x >= 40) {
            System.out.println("x >= 40");
        } else if (x != 45) {
            System.out.println("x != 45");
        } else {
            System.out.println(x);
        }

        boolean b = true;

        if(b) {
            System.out.println("x is true");
        } else if (!b) {
            System.out.println("x is false");
        }
    }
}
