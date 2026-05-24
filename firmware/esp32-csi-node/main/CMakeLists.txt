idf_component_register(
    SRCS
        "csi_node_main.c"
        "wifi_join.c"
        "csi_capture.c"
        "ws_uploader.c"
        "frame_encoder.c"
    INCLUDE_DIRS "."
    REQUIRES
        esp_wifi
        esp_netif
        esp_event
        esp_websocket_client
        nvs_flash
        json
        freertos
        log
)
