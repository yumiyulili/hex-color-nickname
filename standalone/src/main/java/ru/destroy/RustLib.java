package ru.destroy;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Pointer;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;

public interface RustLib extends Library {
    RustLib INSTANCE = load();

    static RustLib load() {
        try {
            String libExtension;
            String osName = System.getProperty("os.name").toLowerCase();

            if (osName.contains("win")) {
                libExtension = ".dll";
            } else if (osName.contains("linux")) {
                libExtension = ".so";
            } else if (osName.contains("mac")) {
                libExtension = ".dylib";
            } else {
                throw new UnsupportedOperationException("Unsupported platform: " + osName);
            }

            File tempLib = File.createTempFile("hexlib", libExtension);
            tempLib.deleteOnExit();

            try (InputStream in = RustLib.class.getResourceAsStream("/native/hexlib" + libExtension)) {
                if (in == null) {
                    throw new UnsatisfiedLinkError("Native library not found in resources: /native/hexlib" + libExtension);
                }
                Files.copy(in, tempLib.toPath(), StandardCopyOption.REPLACE_EXISTING);
            }

            return Native.load(tempLib.getAbsolutePath(), RustLib.class);
        } catch (IOException e) {
            throw new RuntimeException("Failed to load native library", e);
        }
    }

    Pointer encode(String input);

    Pointer decode(String input);

    Pointer hex_color_of(String input);

    Pointer alt_hex_color_of(String input);

    void free_string(Pointer ptr);
}
