class Field {
    public int int_field;
    public String string_field;
    public long long_field;

    public static int static_field;

    public static void main(String[] args) {
        Field.static_field = 1234;
        Field field = new Field();

        field.int_field = 1;
        field.string_field = "test1";

        System.out.println(field.int_field);
        System.out.println(field.string_field);
        System.out.println(field.static_field);
    }
}
