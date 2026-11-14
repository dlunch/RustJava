import java.lang.ref.Reference;
import java.lang.ref.WeakReference;

public class WeakReferenceSemantics {
    private static class Node {
        Object next;
    }

    // referents are created in a separate frame so that nothing but the references can keep them alive
    private static WeakReference<Object> unreachable() {
        return new WeakReference<Object>(new Object());
    }

    private static WeakReference<Object>[] unreachableChain() {
        Node head = new Node();
        head.next = new Object();
        @SuppressWarnings("unchecked")
        WeakReference<Object>[] result = new WeakReference[] { new WeakReference<Object>(head), new WeakReference<Object>(head.next) };
        return result;
    }

    private static void report(String label, boolean value) {
        System.out.print(label);
        System.out.print(": ");
        System.out.println(value);
    }

    public static void main(String[] args) {
        Object strong = new Object();
        Reference<Object> held = new WeakReference<Object>(strong);
        Node holder = new Node();
        holder.next = new Object();
        Reference<Object> heldByField = new WeakReference<Object>(holder.next);
        Reference<Object> collected = unreachable();
        Reference<Object>[] chain = unreachableChain();
        Reference<Object> empty = new WeakReference<Object>(null);

        System.gc();

        report("held", held.get() == strong);
        report("held by field", heldByField.get() == holder.next);
        report("collected", collected.get() == null);
        report("chain head", chain[0].get() == null);
        report("chain tail", chain[1].get() == null);
        report("empty", empty.get() == null);

        held.clear();
        report("cleared", held.get() == null);
        System.gc();
        report("held after clear", heldByField.get() == holder.next);
    }
}
