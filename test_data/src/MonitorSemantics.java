public class MonitorSemantics {
    private static final Object LOCK = new Object();
    private static int counter;

    private static class CounterWorker implements Runnable {
        public void run() {
            for (int i = 0; i < 50; i++) {
                synchronized (LOCK) {
                    int current = counter;
                    Thread.yield();
                    counter = current + 1;
                }
            }
        }
    }

    private static class FailingWorker implements Runnable {
        public void run() {
            try {
                failWhileSynchronized();
            } catch (RuntimeException expected) {
            }
        }
    }

    private static synchronized void failWhileSynchronized() {
        throw new RuntimeException("expected");
    }

    private static synchronized void addTen() {
        counter += 10;
    }

    public static void main(String[] args) throws Exception {
        Thread first = new Thread(new CounterWorker());
        Thread second = new Thread(new CounterWorker());
        first.start();
        second.start();
        first.join();
        second.join();

        Thread failing = new Thread(new FailingWorker());
        failing.start();
        failing.join();
        addTen();

        System.out.println(counter);
    }
}
