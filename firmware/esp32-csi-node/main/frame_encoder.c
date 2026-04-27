#include "frame_encoder.h"

#include <inttypes.h>
#include <stdio.h>
#include <string.h>

#include "esp_log.h"
#include "mbedtls/base64.h"
#include "sdkconfig.h"

static const char *TAG = "frame_encoder";

static int append(char *out, size_t cap, size_t *pos, const char *fmt, ...) {
    if (*pos >= cap) return -1;
    va_list ap;
    va_start(ap, fmt);
    int n = vsnprintf(out + *pos, cap - *pos, fmt, ap);
    va_end(ap);
    if (n < 0 || (size_t)n >= cap - *pos) return -1;
    *pos += (size_t)n;
    return 0;
}

size_t frame_encoder_encode_batch(const ws_csi_sample_t *samples,
                                  size_t                 count,
                                  char                  *out,
                                  size_t                 out_capacity) {
    if (samples == NULL || out == NULL || out_capacity < 64) {
        return 0;
    }

    size_t pos = 0;
    if (append(out, out_capacity, &pos,
               "{\"node\":\"%s\",\"batch\":[", CONFIG_WAVESIGHT_NODE_NAME) != 0) {
        ESP_LOGW(TAG, "header truncated");
        return 0;
    }

    for (size_t i = 0; i < count; i++) {
        const ws_csi_sample_t *s = &samples[i];

        // Base64-encode the IQ payload in-place.
        unsigned char b64[((WS_MAX_SUBCARRIERS * 2) * 4 / 3) + 4];
        size_t b64_len = 0;
        if (mbedtls_base64_encode(b64, sizeof(b64), &b64_len,
                                  (const unsigned char *)s->samples,
                                  s->sample_len) != 0) {
            ESP_LOGW(TAG, "base64 failed (len=%u)", s->sample_len);
            return 0;
        }
        b64[b64_len] = 0;

        const char *sep = (i == 0) ? "" : ",";
        if (append(out, out_capacity, &pos,
                   "%s{\"seq\":%" PRIu32 ",\"ts_us\":%" PRIu64
                   ",\"rssi\":%d,\"ch\":%u,\"bw\":%u,\"iq\":\"%s\"}",
                   sep,
                   s->sequence, s->timestamp_us,
                   (int)s->rssi_dbm,
                   (unsigned)s->channel,
                   (unsigned)s->bandwidth,
                   (const char *)b64) != 0) {
            ESP_LOGW(TAG, "batch item %u truncated", (unsigned)i);
            return 0;
        }
    }

    if (append(out, out_capacity, &pos, "]}") != 0) {
        return 0;
    }
    return pos;
}
