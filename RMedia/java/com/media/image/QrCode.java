package com.media.image;

/**
 * 二维码.
 *
 * @author JNI.
 */
public class QrCode extends Media {
    /**
     * 生成二维码图片.
     *
     * @param codeParams 参数.
     * @return 二维码(海报二维码).
     */
    public static byte[] image(QrCodeParams codeParams) {
        return generate(codeParams.toString(), codeParams.getLogo(), codeParams.getPoster());
    }

    /**
     * 生成.
     *
     * @param queryParams 参数.
     * @param logo        logo 文件.
     * @param poster      海报文件.
     * @return 二维码.
     */
    public static native byte[] generate(String queryParams, byte[] logo, byte[] poster);
}
