#include "esp_http_server.h"

httpd_handle_t start_http_server(void);
void stop_webserver(httpd_handle_t server);