idf_component_register(SRCS "hello_world_main.c" "cmd_wifi.c"
    INCLUDE_DIRS "."
    REQUIRES cmd_nvs cmd_system console iperf spi_flash)
