idf_component_register(SRCS "hello_world_main.c" "cmd_wifi.c" "http.c"
    INCLUDE_DIRS "."
    REQUIRES cmd_nvs cmd_system console esp_http_server esp_netif iperf spi_flash)
