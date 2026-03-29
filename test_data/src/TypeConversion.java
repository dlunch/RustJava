class TypeConversion {
    public static void main(String[] args) {
        // i2b: should sign-extend (128 -> -128, -1 stays -1, 255 -> -1)
        System.out.println("i2b:");
        System.out.println((byte) 128);
        System.out.println((byte) -1);
        System.out.println((byte) 255);
        System.out.println((byte) 0);
        System.out.println((byte) 127);

        // i2s: should sign-extend (32768 -> -32768, -1 stays -1, 65535 -> -1)
        System.out.println("i2s:");
        System.out.println((short) 32768);
        System.out.println((short) -1);
        System.out.println((short) 65535);
        System.out.println((short) 0);
        System.out.println((short) 32767);

        // i2c: should zero-extend
        System.out.println("i2c:");
        System.out.println((int)(char) -1);
        System.out.println((int)(char) 65535);
        System.out.println((int)(char) 0);
    }
}
