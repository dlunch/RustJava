public class InterpreterLoops {
    public static int pure(int count) {
        int sum = 0;
        for (int i = 0; i < count; i++) {
            int value = i * 31 - count;
            value = value < 0 ? -value : value;
            value = value < 10000 ? value : 10000;
            value = value > 7 ? value : 7;
            sum += value;
        }
        return sum;
    }

    public static int javaCalls(int count) {
        int sum = 0;
        for (int i = 0; i < count; i++) {
            sum += max(min(abs(i * 31 - count), 10000), 7);
        }
        return sum;
    }

    public static int runtimeCalls(int count) {
        int sum = 0;
        for (int i = 0; i < count; i++) {
            sum += Math.max(Math.min(Math.abs(i * 31 - count), 10000), 7);
        }
        return sum;
    }

    private static int abs(int value) {
        return value < 0 ? -value : value;
    }

    private static int min(int left, int right) {
        return left < right ? left : right;
    }

    private static int max(int left, int right) {
        return left > right ? left : right;
    }

    public static void main(String[] args) {
        for (int count : new int[] {0, 1, 1000}) {
            System.out.println(pure(count));
            System.out.println(javaCalls(count));
            System.out.println(runtimeCalls(count));
        }
    }
}
