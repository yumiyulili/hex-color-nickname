package ru.destroy;

import com.sun.jna.Pointer;

public class RustWrapper {
    private static final RustLib lib = RustLib.INSTANCE;

    public static String encode(String input) {
        Pointer ptr = lib.encode(input);
        String result = ptr.getString(0);
        lib.free_string(ptr);
        return result;
    }

    public static String decode(String input) {
        Pointer ptr = lib.decode(input);
        String result = ptr.getString(0);
        lib.free_string(ptr);
        return result;
    }

    public static String hexColorOf(String input) {
        Pointer ptr = lib.hex_color_of(input);
        String result = ptr.getString(0);
        lib.free_string(ptr);
        return result;
    }
    public static String altHexColorOf(String input) {
        Pointer ptr = lib.alt_hex_color_of(input);
        String result = ptr.getString(0);
        lib.free_string(ptr);
        return result;
    }
}
