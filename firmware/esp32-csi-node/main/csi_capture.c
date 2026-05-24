#include "csi_capture.h"

#include <string.h>
#include <sys/time.h>

#include "esp_log.h"
#include "esp_timer.h"
#include "esp_wifi.h"
#include "esp_wifi_types.h"
#include "freertos/FreeRTOS.h"
#include "freertos/queue.h"
#include "sdkconfig.h"

static const char *TAG = "csi_capture";

static QueueHandle_t s_queue;
static uint32_t s_sequence;
static uint32_t s_drop_count;

static void IRAM_ATTR on_csi(void *ctx, wifi_csi_info_t *info) {
    if (info == NULL || info->buf == NULL || info->len == 0) {
        return;
    }

    ws_csi_sample_t sample;
    sample.timestamp_us = esp_timer_get_time();
    sample.sequence = ++s_sequence;
    sample.rssi_dbm = info->rx_ctrl.rssi;
    sample.channel = info->rx_ctrl.channel;
    sample.bandwidth = (info->rx_ctrl.cwb == WIFI_BW_HT40) ? 1 : 0;

    uint16_t copy_len = info->len;
    if (copy_len > sizeof(sample.samples)) {
        copy_len = sizeof(sample.samples);
    }
    sample.sample_len = copy_len;
    memcpy(sample.samples, info->buf, copy_len);

    BaseType_t hp_task_woken = pdFALSE;
    if (xQueueSendFromISR(s_queue, &sample, &hp_task_woken) != pdTRUE) {
        s_drop_count++;
        // Pop oldest to make room; tolerant of bursty traffic.
        ws_csi_sample_t discard;
        xQueueReceiveFromISR(s_queue, &discard, &hp_task_woken);
        xQueueSendFromISR(s_queue, &sample, &hp_task_woken);
    }
    portYIELD_FROM_ISR(hp_task_woken);
}

esp_err_t csi_capture_start(void) {
    s_queue = xQueueCreate(CONFIG_WAVESIGHT_CSI_QUEUE_LEN, sizeof(ws_csi_sample_t));
    if (s_queue == NULL) {
        ESP_LOGE(TAG, "queue alloc failed");
        return ESP_ERR_NO_MEM;
    }

    wifi_csi_config_t cfg = {
        .lltf_en = true,
        .htltf_en = true,
        .stbc_htltf2_en = true,
        .ltf_merge_en = true,
        .channel_filter_en = true,
        .manu_scale = false,
        .shift = 0,
    };
    ESP_ERROR_CHECK(esp_wifi_set_csi_config(&cfg));
    ESP_ERROR_CHECK(esp_wifi_set_csi_rx_cb(&on_csi, NULL));
    ESP_ERROR_CHECK(esp_wifi_set_csi(true));

    ESP_LOGI(TAG, "csi capture armed, queue depth %d", CONFIG_WAVESIGHT_CSI_QUEUE_LEN);
    return ESP_OK;
}

bool csi_capture_next(ws_csi_sample_t *out, uint32_t timeout_ms) {
    if (out == NULL || s_queue == NULL) {
        return false;
    }
    return xQueueReceive(s_queue, out, pdMS_TO_TICKS(timeout_ms)) == pdTRUE;
}
