package com.media.image;

/**
 * WebP 格式转换.
 *
 * @author JNI.
 */
public final class WebP extends Media {
    /**
     * 编码.
     *
     * @param from    文件源.
     * @param to      输出源.
     * @param quality 质量 (0-100).
     */
    public static native void encoded(String from, String to, float quality);

    /**
     * 编码.
     *
     * @param from    文件源.
     * @param quality 质量(0-100).
     * @return WebP 图片.
     */
    public static native byte[] encodedByBytes(byte[] from, float quality);

    /**
     * 解码.
     *
     * @param from 源文件.
     * @param to   解码后图片.
     */
    public static native void decode(String from, String to);

    /**
     * 解码.
     *
     * @param from 源文件.
     * @return 解码后图片.
     */
    public static native byte[] decodeByBytes(byte[] from);
}
