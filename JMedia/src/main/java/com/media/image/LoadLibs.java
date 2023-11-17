package com.media.image;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.Objects;
import java.util.function.Consumer;

/**
 * 加载外部依赖.
 *
 * @author 橘猫.
 */
public final class LoadLibs {
    public static boolean isWindows() {
        return Objects.toString(System.getProperties().getProperty("os.name"), "").toUpperCase().contains("WINDOWS");
    }

    public static boolean isMac() {
        return Objects.toString(System.getProperties().getProperty("os.name"), "").toUpperCase().contains("MAC OS");
    }

    public static String getRootPath() {
        return isWindows() ? System.getProperty("java.io.tmpdir") : System.getProperty("user.dir");
    }

    @SuppressWarnings("ResultOfMethodCallIgnored")
    public static void load(String rootPath, List<String> libs, Consumer<String> consumer) {
        for (String lib : libs) {
            final String[] values = lib.split("/");
            final String libName = values[values.length - 1];
            final Path path = Paths.get(rootPath, libName);
            final File file = path.toFile();
            if (file.exists()) {
                file.delete();
            }
            try (InputStream inputStream = LoadLibs.class.getResourceAsStream("/" + lib)) {
                if (Objects.isNull(inputStream)) {
                    throw new RuntimeException("Load lib fail, File not exist");
                }
                try (FileOutputStream outputStream = new FileOutputStream(file)) {
                    inputStream.transferTo(outputStream);
                } catch (IOException e) {
                    throw new RuntimeException("Load lib fail, File corruption; Delete Retry");
                }
            } catch (IOException e) {
                throw new RuntimeException("Load lib fail, File corruption; Delete Retry");
            }
            final String libPath = rootPath.endsWith(File.separator) ? rootPath + libName : rootPath + File.separator + libName;
            consumer.accept(libPath);
            System.load(libPath);
        }
    }
}
