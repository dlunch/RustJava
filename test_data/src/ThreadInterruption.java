public class ThreadInterruption {
    private static final Object WAIT_LOCK = new Object();
    private static final Object WAIT_ACK_LOCK = new Object();
    private static final Object TARGET_LOCK = new Object();
    private static final Object JOIN_LOCK = new Object();
    private static boolean waitReady;
    private static boolean secondWaitReady;
    private static boolean secondWaitWoke;
    private static boolean targetReady;
    private static boolean targetReleased;
    private static boolean joinReady;

    private static class WaitWorker implements Runnable {
        public void run() {
            synchronized (WAIT_LOCK) {
                waitReady = true;
                WAIT_LOCK.notifyAll();
                try {
                    WAIT_LOCK.wait();
                    System.out.println("wait returned");
                } catch (InterruptedException expected) {
                    System.out.println("wait interrupted");
                    System.out.println(Thread.interrupted());
                }

                secondWaitReady = true;
                WAIT_LOCK.notifyAll();
                try {
                    WAIT_LOCK.wait(500);
                } catch (InterruptedException unexpected) {
                    throw new RuntimeException(unexpected.toString());
                }
            }
            synchronized (WAIT_ACK_LOCK) {
                secondWaitWoke = true;
                WAIT_ACK_LOCK.notifyAll();
            }
        }
    }

    private static class PreInterruptedSleepWorker implements Runnable {
        public void run() {
            Thread.currentThread().interrupt();
            try {
                Thread.sleep(25);
                System.out.println("pre sleep returned");
            } catch (InterruptedException expected) {
                System.out.println("pre sleep interrupted");
                System.out.println(Thread.currentThread().isInterrupted());
            }
        }
    }

    private static class TargetWorker implements Runnable {
        public void run() {
            synchronized (TARGET_LOCK) {
                targetReady = true;
                TARGET_LOCK.notifyAll();
                while (!targetReleased) {
                    try {
                        TARGET_LOCK.wait();
                    } catch (InterruptedException unexpected) {
                        throw new RuntimeException(unexpected.toString());
                    }
                }
            }
        }
    }

    private static class JoinWorker implements Runnable {
        private final Thread target;

        JoinWorker(Thread target) {
            this.target = target;
        }

        public void run() {
            synchronized (target) {
                synchronized (JOIN_LOCK) {
                    joinReady = true;
                    JOIN_LOCK.notifyAll();
                }
                try {
                    target.join();
                    System.out.println("join returned");
                } catch (InterruptedException expected) {
                    System.out.println("join interrupted");
                    System.out.println(Thread.interrupted());
                }
            }
        }
    }

    private static class DaemonWorker implements Runnable {
        public void run() {
            System.out.println(new Thread().isDaemon());
        }
    }

    public static void main(String[] args) throws Exception {
        Thread current = Thread.currentThread();
        current.interrupt();
        System.out.println(current.isInterrupted());
        System.out.println(Thread.interrupted());
        System.out.println(current.isInterrupted());

        Thread waiter = new Thread(new WaitWorker());
        waiter.start();
        synchronized (WAIT_LOCK) {
            while (!waitReady) {
                WAIT_LOCK.wait();
            }
        }
        waiter.interrupt();
        synchronized (WAIT_LOCK) {
            while (!secondWaitReady) {
                WAIT_LOCK.wait();
            }
            WAIT_LOCK.notify();
        }
        synchronized (WAIT_ACK_LOCK) {
            long deadline = System.currentTimeMillis() + 200;
            while (!secondWaitWoke) {
                long remaining = deadline - System.currentTimeMillis();
                if (remaining <= 0) {
                    break;
                }
                WAIT_ACK_LOCK.wait(remaining);
            }
            System.out.println(secondWaitWoke);
        }
        waiter.join();

        Thread sleeper = new Thread(new PreInterruptedSleepWorker());
        sleeper.start();
        sleeper.join();

        Thread target = new Thread(new TargetWorker());
        target.start();
        synchronized (TARGET_LOCK) {
            while (!targetReady) {
                TARGET_LOCK.wait();
            }
        }

        Thread joiner = new Thread(new JoinWorker(target));
        joiner.start();
        synchronized (JOIN_LOCK) {
            while (!joinReady) {
                JOIN_LOCK.wait();
            }
        }
        synchronized (target) {
            target.notifyAll();
        }
        joiner.interrupt();
        joiner.join();

        long before = System.currentTimeMillis();
        target.join(10);
        long elapsed = System.currentTimeMillis() - before;
        System.out.println(target.isAlive());
        System.out.println(elapsed >= 0);

        synchronized (TARGET_LOCK) {
            targetReleased = true;
            TARGET_LOCK.notifyAll();
        }
        target.join(0);
        System.out.println(target.isAlive());

        Thread daemon = new Thread(new DaemonWorker());
        daemon.setDaemon(true);
        daemon.start();
        daemon.join();
    }
}
