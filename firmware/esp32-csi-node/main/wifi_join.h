#pragma once

#include "esp_err.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Initialise the WiFi station, connect to the configured SSID, and block
 * until association completes (or until five retry attempts fail).
 *
 * Subsequent disconnects are handled asynchronously by an event handler
 * that triggers a reconnect; callers do not need to do anything special.
 */
esp_err_t wifi_join_start(void);

#ifdef __cplusplus
}
#endif
