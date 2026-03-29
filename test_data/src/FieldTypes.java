class FieldTypes {
    boolean boolField;
    byte byteField;
    char charField;
    short shortField;
    int intField;

    public static void main(String[] args) {
        FieldTypes f = new FieldTypes();

        // putfield/getfield for Z/B/C/S types
        f.boolField = true;
        f.byteField = 123;
        f.charField = 'A';
        f.shortField = 12345;
        f.intField = 99999;

        System.out.println("fields:");
        System.out.println(f.boolField);
        System.out.println(f.byteField);
        System.out.println((int) f.charField);
        System.out.println(f.shortField);
        System.out.println(f.intField);

        // Overwrite and re-read
        f.boolField = false;
        f.byteField = -1;
        f.charField = '\u0000';
        f.shortField = -1;

        System.out.println(f.boolField);
        System.out.println(f.byteField);
        System.out.println((int) f.charField);
        System.out.println(f.shortField);
    }
}
