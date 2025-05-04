class ReferenceCompare {
    public static void main(String[] args) {
        ReferenceCompare a = new ReferenceCompare();
        ReferenceCompare b = new ReferenceCompare();
        ReferenceCompare c = null;
        String d = "test";
        ReferenceCompare e = a;

        if (a == b) {
            System.out.println("a == b");
        } else {
            System.out.println("a != b");
        }

        if (a == c) {
            System.out.println("a == c");
        } else {
            System.out.println("a != c");
        }

        if (b == c) {
            System.out.println("b == c");
        } else {
            System.out.println("b != c");
        }

        if ((Object) a == d) {
            System.out.println("a == d");
        } else {
            System.out.println("a != d");
        }

        if (a == e) {
            System.out.println("a == e");
        } else {
            System.out.println("a != e");
        }
    }
}
