package com.media.image;

import com.billbear.tools.common.LoadLibs;
import lombok.extern.log4j.Log4j2;

import java.util.List;

@Log4j2
public class Media {
    static {
        log.info("JNI Media Loaded ...");
        final List<String> libs = LoadLibs.isWindows() ? List.of("media_jni.dll") : List.of("libmedia_jni.so");
        final String rootPath = LoadLibs.getRootPath();
        LoadLibs.load(rootPath, libs, libPath -> log.info("System Load .so(dll) <== {}", libPath));
    }
}
