package com.media.image;

import lombok.Data;
import org.apache.commons.lang3.StringUtils;

import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.Objects;

/**
 * 二维码参数.
 *
 * @author JNI.
 */
@Data
public class QrCodeParams {
    /**
     * 内容.
     */
    private String text;
    /**
     * 大小.
     */
    private Integer size;
    /**
     * 间距.
     */
    private Integer margin;
    /**
     * 质量.
     */
    private Integer quality;
    /**
     * Logo (二选一).
     */
    private byte[] logo;
    /**
     * Logo 路径 (二选一).
     */
    private String logoPath;
    /**
     * Logo 的大小.
     */
    private Integer logoSize;
    /**
     * Logo 的间距.
     */
    private Integer logoMargin;
    /**
     * 海报 (二选一).
     */
    private byte[] poster;
    /**
     * 海报路径 (二选一).
     */
    private String posterPath;
    /**
     * 海报二维码: X.
     */
    private Integer posterQrCodeX;
    /**
     * 海报二维码: Y.
     */
    private Integer posterQrCodeY;

    public QrCodeParams(String text) {
        this.text = text;
        this.size = 400;
        this.margin = 2;
        this.quality = 1;
    }

    @Override
    public String toString() {
        final StringBuilder sb = new StringBuilder();
        if (StringUtils.isNotBlank(text)) {
            sb.append("text=").append(URLEncoder.encode(text, StandardCharsets.UTF_8)).append("&");
        }
        if (Objects.nonNull(size)) {
            sb.append("size=").append(size).append("&");
        }
        if (Objects.nonNull(margin)) {
            sb.append("margin=").append(margin).append("&");
        }
        if (Objects.nonNull(quality)) {
            sb.append("quality=").append(quality).append("&");
        }
        if (StringUtils.isNotBlank(logoPath)) {
            sb.append("logo_path=").append(URLEncoder.encode(logoPath, StandardCharsets.UTF_8)).append("&");
        }
        if (Objects.nonNull(logoSize)) {
            sb.append("logo_size=").append(logoSize).append("&");
        }
        if (Objects.nonNull(logoMargin)) {
            sb.append("logo_margin=").append(logoMargin).append("&");
        }
        if (StringUtils.isNotBlank(posterPath)) {
            sb.append("poster_path=").append(URLEncoder.encode(posterPath, StandardCharsets.UTF_8)).append("&");
        }
        if (Objects.nonNull(posterQrCodeX)) {
            sb.append("poster_qr_code_x=").append(posterQrCodeX).append("&");
        }
        if (Objects.nonNull(posterQrCodeY)) {
            sb.append("poster_qr_code_y=").append(posterQrCodeY).append("&");
        }
        return sb.toString();
    }
}
