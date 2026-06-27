import java.util.ArrayList;
import java.util.Collection;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Hashtable;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.jar.Attributes;

class CollectionsInvokeInterface {
    public static void main(String[] args) {
        List list = new ArrayList();
        list.add("array");
        if ("array".equals(list.get(0)) && list.iterator().hasNext()) {
            System.out.println("arraylist-ok");
        }

        Collection set = new HashSet();
        set.add("hash");
        if (set.contains("hash") && set.iterator().hasNext()) {
            System.out.println("hashset-ok");
        }

        Map hashMap = new HashMap();
        hashMap.put("key", "value");
        if ("value".equals(hashMap.get("key"))
                && hashMap.keySet().contains("key")
                && hashMap.values().contains("value")
                && hashMap.entrySet().iterator().hasNext()) {
            System.out.println("hashmap-ok");
        }

        Iterator hashMapIterator = hashMap.entrySet().iterator();
        Map.Entry hashMapEntry = (Map.Entry) hashMapIterator.next();
        hashMapEntry.setValue("updated");
        if ("updated".equals(hashMap.get("key"))) {
            System.out.println("hashmap-entry-ok");
        }

        Map hashtable = new Hashtable();
        hashtable.put("table-key", "table-value");
        if ("table-value".equals(hashtable.get("table-key"))
                && hashtable.keySet().contains("table-key")
                && hashtable.values().contains("table-value")
                && hashtable.entrySet().iterator().hasNext()) {
            System.out.println("hashtable-ok");
        }

        Iterator hashtableIterator = hashtable.entrySet().iterator();
        Map.Entry hashtableEntry = (Map.Entry) hashtableIterator.next();
        hashtableEntry.setValue("table-updated");
        if ("table-updated".equals(hashtable.get(hashtableEntry.getKey()))) {
            System.out.println("hashtable-entry-ok");
        }

        Attributes attributes = new Attributes();
        attributes.putValue("Name", "Value");
        if ("Value".equals(attributes.getValue("Name"))) {
            System.out.println("attributes-ok");
        }
    }
}
