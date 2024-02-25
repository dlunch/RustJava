class Exception {
    public static void main(String[] args) {
        try {
            throw new Throwable();
        } catch (Throwable e) {
            System.out.println("Caught exception1");
        } finally {
            System.out.println("Finally1");
        }

        try {
            throw new IllegalArgumentException();
        } catch (Throwable e) {
            System.out.println("Caught exception2");
        } finally {
            System.out.println("Finally2");
        }

        try {
            throw new NullPointerException();
        } catch (IllegalArgumentException e) {
            System.out.println("Should not be executed");
        } catch (RuntimeException e) {
            System.out.println("Caught exception3");
        } finally {
            System.out.println("Finally3");
        }

        try {
            throwsException();
            System.out.println("Should not be executed");
        } catch (NullPointerException e) {
            System.out.println("Should not be executed");
        } catch (UnsupportedOperationException e) {
            System.out.println("Caught exception4");
        } finally {
            System.out.println("Finally4");
        }
    }

    public static void throwsException() throws RuntimeException {
        throw new UnsupportedOperationException();
    }
}
