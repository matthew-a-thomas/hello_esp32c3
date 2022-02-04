/* Hello World Example

   This example code is in the Public Domain (or CC0 licensed, at your option.)

   Unless required by applicable law or agreed to in writing, this
   software is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
   CONDITIONS OF ANY KIND, either express or implied.
*/
#include <stdio.h>
#include "sdkconfig.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "esp_system.h"
#include "esp_spi_flash.h"
#include "MattTask.h"

#define STACK_SIZE 4096

void app_main(void)
{
	// Start tasks
	TaskHandle_t matt_task_handle = NULL;
	xTaskCreate(run_matt_task, "Matt Task", STACK_SIZE, 0, tskIDLE_PRIORITY, &matt_task_handle);
	configASSERT(matt_task_handle);

	printf("Hello world!\n");
	for (;;)
	{

		/* Print chip information */
		esp_chip_info_t chip_info;
		esp_chip_info(&chip_info);
		printf("This is %s chip with %d CPU core(s), WiFi%s%s, ",
				CONFIG_IDF_TARGET,
				chip_info.cores,
				(chip_info.features & CHIP_FEATURE_BT) ? "/BT" : "",
				(chip_info.features & CHIP_FEATURE_BLE) ? "/BLE" : "");

		printf("silicon revision %d, ", chip_info.revision);

		printf("%dMB %s flash\n", spi_flash_get_chip_size() / (1024 * 1024),
				(chip_info.features & CHIP_FEATURE_EMB_FLASH) ? "embedded" : "external");

		printf("Minimum free heap size: %d bytes\n", esp_get_minimum_free_heap_size());

		vTaskDelay(10000 / portTICK_PERIOD_MS);
	}

	// Tear down tasks
	if (matt_task_handle) {
		vTaskDelete(matt_task_handle);
	}
    esp_restart();
}
