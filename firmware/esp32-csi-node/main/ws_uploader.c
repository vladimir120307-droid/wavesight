#include "ws_uploader.h"

#include <string.h>

#include "csi_capture.h"
#include "esp_event.h"
#include "esp_log.h"
#include "esp_websocket_client.h"
#include "frame_encoder.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "sdkconfig.h"

static const char *TAG = "ws_uploader";

#define JSON_BUFFER_BYTES (16 * 1024)

static esp_websocket_client_handle_t s_client;
static volatile bool s_connected;

static void on_ws_event(void *arg, esp_event_base_t base,
                        int32_t event_id, void *event_data) {
    switch (event_id) {
        case WEBSOCKET_EVENT_CONNECTED:
            s_connected = true;
            ESP_LOGI(TAG, "connected to %s", CONFIG_WAVESIGHT_SERVER_URI);
            break;
        case WEBSOCKET_EVENT_DISCONNECTED:
            s_connected = false;
            ESP_LOGW(TAG, "disconnected, client will retry");
            break;
        case WEBSOCKET_EVENT_ERROR:
            ESP_LOGE(TAG, "websocket error");
            break;
        default:
            break;
    }
}

static void uploader_task(void *arg) {
    const size_t batch_size = CONFIG_WAVESIGHT_BATCH_SIZE;
    ws_csi_sample_t *batch = pvPortMalloc(sizeof(ws_csi_sample_t) * batch_size);
    char            *json  = pvPortMalloc(JSON_BUFFER_BYTES);
    if (batch == NULL || json == NULL) {
        ESP_LOGE(TAG, "alloc failed");
        vTaskDelete(NULL);
    }

    while (true) {
        size_t filled = 0;
        while (filled < batch_size) {
            if (!csi_capture_next(&batch[filled], 200)) {
                break;
            }
            filled++;
        }
        if (filled == 0) {
            continue;
        }

        if (!s_connected) {
            // Drop batches while disconnected — we are observing live RF;
            // there is no point in caching seconds-old phases for an
            // edge server that wants near-real-time data.
            continue;
        }

        size_t json_len = frame_encoder_encode_batch(batch, filled, json,
                                                     JSON_BUFFER_BYTES);
        if (json_len == 0) {
            ESP_LOGW(TAG, "encode produced 0 bytes, dropping batch");
            continue;
        }
        int sent = esp_websocket_client_send_text(s_client, json, json_len,
                                                  pdMS_TO_TICKS(2000));
        if (sent < 0) {
            ESP_LOGW(TAG, "send failed");
        }
    }
}

esp_err_t ws_uploader_start(void) {
    esp_websocket_client_config_t cfg = {
        .uri               = CONFIG_WAVESIGHT_SERVER_URI,
        .reconnect_timeout_ms = 5000,
        .buffer_size       = 16384,
        .task_stack        = 8192,
    };

    s_client = esp_websocket_client_init(&cfg);
    if (s_client == NULL) {
        return ESP_FAIL;
    }
    ESP_ERROR_CHECK(esp_websocket_register_events(s_client, WEBSOCKET_EVENT_ANY,
                                                  on_ws_event, NULL));
    ESP_ERROR_CHECK(esp_websocket_client_start(s_client));

    BaseType_t r = xTaskCreatePinnedToCore(uploader_task, "ws_up",
                                           6144, NULL, 5, NULL, 1);
    return (r == pdPASS) ? ESP_OK : ESP_FAIL;
}
