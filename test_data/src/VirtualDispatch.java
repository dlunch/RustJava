import dispatch.foreign.ForeignChild;
import java.util.Iterator;
import java.util.LinkedHashMap;

public class VirtualDispatch {
    static class HookCollisionMap extends LinkedHashMap {
        int collisionCalls;

        public void initializeMap() {
            collisionCalls++;
        }
    }

    public static void main(String[] args) {
        new ForeignChild().callHook();

        HookCollisionMap map = new HookCollisionMap();
        map.put("a", "a");
        map.put("b", "b");
        Iterator iterator = map.keySet().iterator();
        System.out.println(map.collisionCalls);
        System.out.println(iterator.next());
        System.out.println(iterator.next());
    }
}
