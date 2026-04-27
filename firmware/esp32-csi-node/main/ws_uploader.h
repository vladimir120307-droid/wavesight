#pragma once

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Spawn the uploader task. The task pops CSI samples from the capture
 * queue, batches them according to CONFIG_WAVESIGHT_BATCH_SIZE, encodes
 * them as JSON, and pushes them over a WebSocket connection to the
 * configured server URI.
 *
 * Reconnects automatically with exponential backoff on disconnect.
 */
esp_err_t ws_uploader_start(void);

#ifdef __cplusplus
}
#endif
