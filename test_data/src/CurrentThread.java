public class CurrentThread {
    public static void main(String[] args) throws Exception {
        Thread a = Thread.currentThread();
        Thread b = Thread.currentThread();
        System.out.println(a == b);

        final Thread[] seen = new Thread[1];
        Thread t = new Thread(new Runnable() {
            public void run() {
                seen[0] = Thread.currentThread();
            }
        });
        t.start();
        t.join();
        System.out.println(seen[0] == t);
        System.out.println(seen[0] == a);
    }
}
