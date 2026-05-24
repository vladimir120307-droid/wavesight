#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "esp_err.h"
#include "freertos/FreeRTOS.h"
#include "freertos/queue.h"

#ifdef __cplusplus
extern "C" {
#endif

/** Maximum subcarrier count we keep on the wire (HT40 wide = 256). */
#define WS_MAX_SUBCARRIERS 256

/**
 * One CSI sample as forwarded from the WiFi callback to the uploader task.
 *
 * Fields are packed to be cheap to enqueue from interrupt context; the
 * uploader thread re-interprets `samples` as interleaved I/Q int8 pairs.
 */
typedef struct {
    uint64_t timestamp_us;   /**< Local microsecond timestamp at capture. */
    uint32_t sequence;       /**< Monotonic per-node sequence number. */
    int8_t   rssi_dbm;       /**< Receiver signal strength. */
    uint8_t  channel;        /**< Primary WiFi channel. */
    uint8_t  bandwidth;      /**< 0 = HT20, 1 = HT40. */
    uint16_t sample_len;     /**< Number of int8 entries in `samples`. */
    int8_t   samples[WS_MAX_SUBCARRIERS * 2];
} ws_csi_sample_t;

/**
 * Initialise the CSI subsystem.
 *
 * Allocates the outbound queue (capacity = CONFIG_WAVESIGHT_CSI_QUEUE_LEN)
 * and registers the WiFi CSI callback. Must be called *after* WiFi is up.
 *
 * Returns ESP_OK on success.
 */
esp_err_t csi_capture_start(void);

/**
 * Pop the next captured sample from the queue.
 *
 * Blocks up to `timeout_ms` waiting for a frame. Returns true if `out` was
 * populated, false on timeout.
 */
bool csi_capture_next(ws_csi_sample_t *out, uint32_t timeout_ms);

#ifdef __cplusplus
}
#endif
