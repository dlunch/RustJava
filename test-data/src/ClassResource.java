import loader.Base;

class ClassResource {
    public static void main(String[] args) {
        System.out.println(Base.class.getResourceAsStream("Payload.class") != null);
        System.out.println(Base.class.getResourceAsStream("/loader/Payload.class") != null);
        System.out.println(Base.class.getResourceAsStream("loader/Payload.class") != null);
        System.out.println(Base.class.getResourceAsStream("/Payload.class") != null);
        System.out.println(ClassResource.class.getResourceAsStream("loader/Payload.class") != null);
        System.out.println(Base.class.getResource("Payload.class") != null);
        System.out.println(Base.class.getResource("/loader/Payload.class") != null);
        System.out.println(Base.class.getResource("loader/Payload.class") != null);

        ClassLoader loader = ClassResource.class.getClassLoader();
        System.out.println(loader.getResourceAsStream("loader/Payload.class") != null);
        System.out.println(loader.getResourceAsStream("/loader/Payload.class") != null);
    }
}
