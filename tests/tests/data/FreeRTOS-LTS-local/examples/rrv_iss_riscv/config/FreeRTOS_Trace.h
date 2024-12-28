#ifndef FREERTOS_TRACE_H
#define FREERTOS_TRACE_H

#include "../uart_printf.h"

#define traceENTER_vTaskSwitchContext()                                        \
  do {                                                                         \
    uart_printf("vTaskSwitchContext, current task name: %s\n", pcTaskGetName(xTaskGetCurrentTaskHandle()));                                       \
  } while (0)

#define traceMOVED_TASK_TO_READY_STATE(pxTCB)                                  \
  do {                                                                         \
    uart_printf("Moved task to ready state: %s\n", (pxTCB)->pcTaskName);       \
  } while (0)

#define tracePOST_MOVED_TASK_TO_READY_STATE(pxTCB)                             \
  do {                                                                         \
    uart_printf("Post moved task to ready state: %s\n", (pxTCB)->pcTaskName);  \
  } while (0)

#define traceTASK_SUSPEND(pxTaskToSuspend)                                     \
  do {                                                                         \
    uart_printf("Task suspended: %s\n", (pxTaskToSuspend)->pcTaskName);        \
  } while (0)

#define traceTASK_RESUME(pxTaskToResume)                                       \
  do {                                                                         \
    uart_printf("Task resumed: %s\n", (pxTaskToResume)->pcTaskName);           \
  } while (0)

#define traceENTER_vTaskResume(xTaskToResume)                                  \
  do {                                                                         \
    uart_printf("Task resuming from ISR: %s\n", (xTaskToResume)->pcTaskName);  \
  } while (0)

#define traceTASK_RESUME_FROM_ISR(pxTaskToResume)                              \
  do {                                                                         \
    uart_printf("Task resumed from ISR: %s\n", (pxTaskToResume)->pcTaskName);  \
  } while (0)

#define traceENTER_vTaskDelay(xTicksToDelay)                                   \
  do {                                                                         \
    uart_printf("Task delay: %d\n", (xTicksToDelay));                          \
  } while (0)

#if 0
#define traceTASK_INCREMENT_TICK(xTickCount)                                   \
  do {                                                                         \
    uart_printf("Task tick increment: %d\n", (xTickCount));                    \
  } while (0)

#define traceENTER_xTaskIncrementTick()                                        \
  do {                                                                         \
    uart_printf("Task increment tick\n");                                      \
  } while (0)
#endif 

/* Enable or disable specific trace macros */
#define ENABLE_TRACE_TASKS 1
/* #define ENABLE_TRACE_QUEUES 1 */
#define ENABLE_TRACE_TIMERS 1

/* Define the debug output function */
#ifndef TRACE_PRINT
#define TRACE_PRINT uart_printf
#endif

/* Task-related trace macros */
#if ENABLE_TRACE_TASKS
#define traceTASK_CREATE(pxTaskHandle)                                         \
  TRACE_PRINT("Task created: %s\n", pcTaskGetName(pxTaskHandle))

#define traceTASK_DELETE(pxTaskHandle)                                         \
  TRACE_PRINT("Task deleted: %s\n", pcTaskGetName(pxTaskHandle))

#define traceTASK_SWITCHED_IN()                                                \
  TRACE_PRINT("Task switched in: %s\n",                                        \
              pcTaskGetName(xTaskGetCurrentTaskHandle()))

#define traceTASK_SWITCHED_OUT()                                               \
  TRACE_PRINT("Task switched out: %s\n",                                       \
              pcTaskGetName(xTaskGetCurrentTaskHandle()))
#if 0
#define traceTASK_SUSPEND(pxTaskHandle)                                        \
  TRACE_PRINT("Task suspended: %s\n", pcTaskGetName(pxTaskHandle))

#define traceTASK_RESUME(pxTaskHandle)                                         \
  TRACE_PRINT("Task resumed: %s\n", pcTaskGetName(pxTaskHandle))
#endif
#else
#define traceTASK_CREATE(pxTaskHandle)
#define traceTASK_DELETE(pxTaskHandle)
#define traceTASK_SWITCHED_IN()
#define traceTASK_SWITCHED_OUT()
#define traceTASK_SUSPEND(pxTaskHandle)
#define traceTASK_RESUME(pxTaskHandle)
#endif

/* Queue-related trace macros */
#if ENABLE_TRACE_QUEUES
#define traceQUEUE_CREATE(pxQueueHandle)                                       \
  TRACE_PRINT("Queue created: %p", (void *)pxQueueHandle)

#define traceQUEUE_DELETE(pxQueueHandle)                                       \
  TRACE_PRINT("Queue deleted: %p", (void *)pxQueueHandle)

#define traceQUEUE_SEND(pxQueueHandle)                                         \
  TRACE_PRINT("Queue send: %p", (void *)pxQueueHandle)

#define traceQUEUE_RECEIVE(pxQueueHandle)                                      \
  TRACE_PRINT("Queue receive: %p", (void *)pxQueueHandle)

#define traceQUEUE_PEEK(pxQueueHandle)                                         \
  TRACE_PRINT("Queue peek: %p", (void *)pxQueueHandle)
#else
#define traceQUEUE_CREATE(pxQueueHandle)
#define traceQUEUE_DELETE(pxQueueHandle)
#define traceQUEUE_SEND(pxQueueHandle)
#define traceQUEUE_RECEIVE(pxQueueHandle)
#define traceQUEUE_PEEK(pxQueueHandle)
#endif

/* Timer-related trace macros */
#if ENABLE_TRACE_TIMERS
#define traceTIMER_CREATE(pxTimerHandle)                                       \
  TRACE_PRINT("Timer created: %p\n", (void *)pxTimerHandle)

#define traceTIMER_START(pxTimerHandle)                                        \
  TRACE_PRINT("Timer started: %p\n", (void *)pxTimerHandle)

#define traceTIMER_STOP(pxTimerHandle)                                         \
  TRACE_PRINT("Timer stopped: %p\n", (void *)pxTimerHandle)

#define traceTIMER_EXPIRED(pxTimerHandle)                                      \
  TRACE_PRINT("Timer expired: %p\n", (void *)pxTimerHandle)
#else
#define traceTIMER_CREATE(pxTimerHandle)
#define traceTIMER_START(pxTimerHandle)
#define traceTIMER_STOP(pxTimerHandle)
#define traceTIMER_EXPIRED(pxTimerHandle)
#endif

/* ISR-related trace macros */
#define traceISR_ENTER() TRACE_PRINT("ISR entered")

#define traceISR_EXIT() TRACE_PRINT("ISR exited")

#define traceSEMAPHORE_CREATE(xSemaphore, uxMaxCount)                          \
  do {                                                                         \
    uart_printf("Semaphore created: %p, max count: %lu\n", xSemaphore,       \
                uxMaxCount);                                                   \
  } while (0)

#define traceSEMAPHORE_GIVE(xSemaphore)                                        \
  do {                                                                         \
    uart_printf("Semaphore given: %p\n", xSemaphore);                        \
  } while (0)

#define traceSEMAPHORE_GIVE_FAILED(xSemaphore)                                 \
  do {                                                                         \
    uart_printf("Semaphore give failed: %p\n", xSemaphore);                  \
  } while (0)

#define traceSEMAPHORE_TAKE(xSemaphore)                                        \
  do {                                                                         \
    uart_printf("Semaphore taken: %p\n", xSemaphore);                        \
  } while (0)

#define traceSEMAPHORE_TAKE_FAILED(xSemaphore)                                 \
  do {                                                                         \
    uart_printf("Semaphore take failed: %p\n", xSemaphore);                  \
  } while (0)

#define traceSEMAPHORE_DELETE(xSemaphore)                                      \
  do {                                                                         \
    uart_printf("Semaphore deleted: %p\n", xSemaphore);                      \
  } while (0)

#define traceTIMER_DELETE(xTimer)                                              \
  do {                                                                         \
    uart_printf("Timer deleted: %p\n", xTimer);                              \
  } while (0)

#define traceTIMER_COMMAND_RECEIVED(xTimer, xCommandID, xCommandValue)         \
  do {                                                                         \
    uart_printf(                                                               \
        "Timer command received: %p, command ID: %d, command value: %d\n",   \
        xTimer, xCommandID, xCommandValue);                                    \
  } while (0)

#endif