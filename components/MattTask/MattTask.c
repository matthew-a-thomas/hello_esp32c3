#include <stdio.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "MattTask.h"

void print_task_name(void)
{
	TaskStatus_t xTaskDetails;
	vTaskGetInfo(NULL, &xTaskDetails, pdFALSE, eInvalid);
	printf("Hello from ");
	printf(xTaskDetails.pcTaskName);
	printf("!\n");
}

void run_matt_task(void * parameters)
{
	printf("Matt Task is active!\n");
	const TickType_t frequency = 1000 / portTICK_PERIOD_MS;
	TickType_t last_wake_time = xTaskGetTickCount();
	for (;;)
	{
		vTaskDelayUntil(&last_wake_time, frequency);
		print_task_name();
	}
	vTaskDelete(NULL);
}
