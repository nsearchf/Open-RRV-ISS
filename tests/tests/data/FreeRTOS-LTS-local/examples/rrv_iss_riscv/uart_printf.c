#include "uart_printf.h"

#include <stdio.h>
#include <stdarg.h>

// #define X86_PLATFORM

// SiFive FE310-G002
// 0x1001_3000 0x1001_3FFF   RWA    UART 0
// 0x1002_3000 0x1002_3FFF   RWA    UART 1
// 0x00 txdata  Transmit data register
// 0x04 rxdata  Receive data register

#define UART0_TXDATA_PTR (volatile char*)(0x10013000 + 0)
void usr_putchar(char c) {
#ifndef X86_PLATFORM
    *UART0_TXDATA_PTR = c;
#else
    putchar(c);
#endif
}

#define USR_PRINTF_BUFFER_SIZE 1024

int uart_printf(const char* fmt, ...) {
    char buffer[USR_PRINTF_BUFFER_SIZE];
    va_list args;

    va_start(args, fmt);
    int ret = vsnprintf(buffer, USR_PRINTF_BUFFER_SIZE, fmt, args);
    va_end(args);

#ifdef X86_PLATFORM
    // printf("[>>>>>>] %s", buffer);
#endif

    for (int i = 0; i < ret; i++)
        usr_putchar(buffer[i]);

    return ret;
}
