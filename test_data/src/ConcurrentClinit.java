public class ConcurrentClinit {
    private static volatile boolean successfulOwnerStarted;
    private static volatile boolean successfulWaiterStarted;
    private static volatile boolean failingOwnerStarted;
    private static volatile boolean failingWaiterStarted;

    private static class Successful {
        static int initializationCount;
        static int value;

        static {
            initializationCount++;
            successfulOwnerStarted = true;
            while (!successfulWaiterStarted) {
                Thread.yield();
            }
            try {
                Thread.sleep(10);
            } catch (InterruptedException exception) {
                throw new RuntimeException("interrupted");
            }
            value = 42;
        }
    }

    private static class Failing {
        static int value;

        static {
            failingOwnerStarted = true;
            while (!failingWaiterStarted) {
                Thread.yield();
            }
            try {
                Thread.sleep(10);
            } catch (InterruptedException exception) {
                throw new RuntimeException("interrupted");
            }
            if (failingOwnerStarted) {
                throw new RuntimeException("initialization failed");
            }
        }
    }

    private static class SuccessfulReader implements Runnable {
        private final boolean waiter;
        int value;

        SuccessfulReader(boolean waiter) {
            this.waiter = waiter;
        }

        public void run() {
            if (waiter) {
                successfulWaiterStarted = true;
            }
            value = Successful.value;
        }
    }

    private static class FailingReader implements Runnable {
        private final boolean waiter;
        String errorClass;

        FailingReader(boolean waiter) {
            this.waiter = waiter;
        }

        public void run() {
            if (waiter) {
                failingWaiterStarted = true;
            }
            try {
                int ignored = Failing.value;
            } catch (Throwable throwable) {
                errorClass = throwable.getClass().getName();
            }
        }
    }

    public static void main(String[] args) throws Exception {
        SuccessfulReader successfulOwner = new SuccessfulReader(false);
        Thread successfulOwnerThread = new Thread(successfulOwner);
        successfulOwnerThread.start();
        while (!successfulOwnerStarted) {
            Thread.yield();
        }

        SuccessfulReader successfulWaiter = new SuccessfulReader(true);
        Thread successfulWaiterThread = new Thread(successfulWaiter);
        successfulWaiterThread.start();
        successfulOwnerThread.join();
        successfulWaiterThread.join();

        System.out.println(successfulOwner.value);
        System.out.println(successfulWaiter.value);
        System.out.println(Successful.initializationCount);

        FailingReader failingOwner = new FailingReader(false);
        Thread failingOwnerThread = new Thread(failingOwner);
        failingOwnerThread.start();
        while (!failingOwnerStarted) {
            Thread.yield();
        }

        FailingReader failingWaiter = new FailingReader(true);
        Thread failingWaiterThread = new Thread(failingWaiter);
        failingWaiterThread.start();
        failingOwnerThread.join();
        failingWaiterThread.join();

        System.out.println(failingOwner.errorClass);
        System.out.println(failingWaiter.errorClass);
        try {
            int ignored = Failing.value;
        } catch (Throwable throwable) {
            System.out.println(throwable.getClass().getName());
        }
    }
}
