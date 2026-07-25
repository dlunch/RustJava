public class ClassMetadata {
    interface First {
    }

    interface Second {
    }

    static class Sample implements Second, First {
    }

    static class Payload {
    }

    static class ByteLoader extends ClassLoader {
        ByteLoader(ClassLoader parent) {
            super(parent);
        }

        public Class loadClass(String name) throws ClassNotFoundException {
            if (!name.equals("ClassMetadata$Payload")) {
                return super.loadClass(name);
            }

            Class loaded = findLoadedClass(name);
            return loaded == null ? findClass(name) : loaded;
        }

        protected Class findClass(String name) throws ClassNotFoundException {
            try {
                java.io.InputStream input = ClassMetadata.class.getResourceAsStream("ClassMetadata$Payload.class");
                byte[] bytes = new byte[input.available()];
                int offset = 0;
                while (offset < bytes.length) {
                    int read = input.read(bytes, offset, bytes.length - offset);
                    if (read < 0) {
                        break;
                    }
                    offset += read;
                }
                input.close();
                return defineClass(name, bytes, 0, offset);
            } catch (java.io.IOException exception) {
                throw new ClassNotFoundException(name);
            }
        }
    }

    public static void main(String[] args) throws Exception {
        ClassLoader loader = ClassMetadata.class.getClassLoader();

        System.out.println(Object.class.getSuperclass() == null);
        System.out.println(First.class.getSuperclass() == null);
        System.out.println(int.class.getSuperclass() == null);
        System.out.println(String.class.getSuperclass() == Object.class);
        System.out.println(int[].class.getSuperclass() == Object.class);

        System.out.println(int[].class.getClassLoader() == null);
        System.out.println(String[].class.getClassLoader() == null);
        System.out.println(Sample.class.getClassLoader() == loader);
        System.out.println(Sample[].class.getClassLoader() == loader);
        System.out.println(Sample[][].class.getClassLoader() == loader);

        System.out.println(int[].class.getComponentType() == Integer.TYPE);
        System.out.println(Sample[].class.getComponentType() == Sample.class);
        System.out.println(Sample[][].class.getComponentType() == Sample[].class);
        System.out.println(String.class.getComponentType() == null);

        Class[] arrayInterfaces = Sample[].class.getInterfaces();
        System.out.println(arrayInterfaces.length == 2);
        System.out.println(arrayInterfaces[0] == Cloneable.class);
        System.out.println(arrayInterfaces[1] == java.io.Serializable.class);

        Class[] sampleInterfaces = Sample.class.getInterfaces();
        System.out.println(sampleInterfaces.length == 2);
        System.out.println(sampleInterfaces[0] == Second.class);
        System.out.println(sampleInterfaces[1] == First.class);

        ByteLoader customLoader = new ByteLoader(loader);
        Class payload = customLoader.loadClass("ClassMetadata$Payload");
        Class payloadArray = customLoader.loadClass("[LClassMetadata$Payload;");
        Class payloadMatrix = customLoader.loadClass("[[LClassMetadata$Payload;");
        System.out.println(payload.getClassLoader() == customLoader);
        System.out.println(payloadArray.getClassLoader() == customLoader);
        System.out.println(payloadMatrix.getClassLoader() == customLoader);
        System.out.println(payloadArray.getComponentType() == payload);
        System.out.println(payloadMatrix.getComponentType() == payloadArray);
    }
}
