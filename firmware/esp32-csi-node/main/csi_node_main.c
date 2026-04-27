#include "csi_capture.h"
#include "esp_log.h"
#include "sdkconfig.h"
#include "wifi_join.h"
#include "ws_uploader.h"

static const char *TAG = "wavesight";

void app_main(void) {
    ESP_LOGI(TAG, "WaveSight CSI node '%s' booting", CONFIG_WAVESIGHT_NODE_NAME);

    if (wifi_join_start() != ESP_OK) {
        ESP_LOGE(TAG, "wifi join failed, halting");
        return;
    }
    if (csi_capture_start() != ESP_OK) {
        ESP_LOGE(TAG, "csi capture failed, halting");
        return;
    }
    if (ws_uploader_start() != ESP_OK) {
        ESP_LOGE(TAG, "uploader failed, halting");
        return;
    }

    ESP_LOGI(TAG, "running");
}
