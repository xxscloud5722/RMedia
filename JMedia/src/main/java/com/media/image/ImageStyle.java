package com.media.image;

import java.io.File;
import java.util.HashMap;
import java.util.Map;

/**
 * 图片根据样式处理.
 * <br/>CSS 参数： 每个参数都用字母A分割.
 * <ul>
 * <li><p>样式参数例子</p></li>
 * <li><b>ZW800</b> 按照宽度800缩放图片</li>
 * <li><b>ZH800</b> 按照高度800缩放图片</li>
 * <li><b>ZW800ACLT450x300</b> 先按照宽度800缩放图片然后通过左上角截取图片450x300大小</li>
 * <li><b>CCC300</b> 居中截取图片300x300</li>
 * </ul>
 *
 * <ul>
 * <li>ZW[800] 按照宽度800压缩</li>
 * <li>ZH[800] 按照高度800压缩</li>
 * <li>ZS[3] 按照百分之三十缩放</li>
 * </ul>
 *
 * <ul>
 * <li>CLT300[X400不填默认左值] 左上角截取图片</li>
 * <li>CRT300[X400不填默认左值] 右上角截取图片</li>
 * <li>CLB300[X400不填默认左值] 左下角截取图片</li>
 * <li>CRB300[X400不填默认左值] 右下角截取图片</li>
 * <li>CLC300[X400不填默认左值] 左居中截取图片</li>
 * <li>CRC300[X400不填默认左值] 右居中截取图片</li>
 * <li>CCC300[X400不填默认左值] 完全居中截取图片</li>
 * </ul>
 *
 * @author JNI.
 */
public class ImageStyle extends Media {

    /**
     * 转换.
     *
     * @param from 源文件.
     * @param to   目标文件.
     * @param args 样式参数.
     */
    public static void convert(File from, File to, String args) {
        handle(null, String.format("quality=75&type=webp&from=%s&to=%s&args=%s", from, to, args));
    }

    /**
     * 转换.
     *
     * @param from 源文件.
     * @param to   目标文件.
     * @param args 样式参数.
     */
    public static void convert(String from, String to, String args) {
        handle(null, String.format("quality=75&type=webp&from=%s&to=%s&args=%s", from, to, args));
    }

    /**
     * 转换.
     *
     * @param image 文件二进制.
     * @param file  输出文件.
     * @param args  样式参数.
     */
    public static void convert(byte[] image, File file, String args) {
        handle(image, String.format("quality=75&type=webp&to=%s&args=%s", file, args));
    }

    /**
     * 转换.
     *
     * @param image 文件二进制.
     * @param to    目标文件.
     * @param args  样式参数.
     */
    public static void convert(byte[] image, String to, String args) {
        handle(image, String.format("quality=75&type=webp&to=%s&args=%s", to, args));
    }


    /**
     * 转换.
     *
     * @param file 文件二进制.
     * @param args 样式参数.
     */
    public static byte[] convert(byte[] file, String args) {
        return handle(file, String.format("quality=75&type=webp&args=%s", args));
    }

    /**
     * 处理图片.
     *
     * @param image       图片.
     * @param queryParams 参数.
     * @return 二进制 (如果传入To 参数则不会返回).
     */
    public static native byte[] handle(byte[] image, String queryParams);


    /**
     * 元信息.
     *
     * @param file 文件.
     * @return 信息.
     */
    public static Map<String, String> metadata(File file) {
        final String result = imageMetadata(file.getAbsolutePath());
        final HashMap<String, String> map = new HashMap<>();
        for (String item : result.split("&")) {
            final String[] values = item.split("=");
            if (values.length > 1) {
                map.put(values[0], values[1]);
            }
        }
        return map;
    }


    public static native String imageMetadata(String file);
}
