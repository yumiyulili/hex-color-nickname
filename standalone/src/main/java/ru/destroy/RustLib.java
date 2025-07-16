package ru.destroy;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Pointer;

public interface RustLib extends Library {
    RustLib INSTANCE = Native.load("hexlib", RustLib.class);

    Pointer encode(String input);

    Pointer decode(String input);

    Pointer hex_color_of(String input);

    Pointer alt_hex_color_of(String input);

    void free_string(Pointer ptr);
}
