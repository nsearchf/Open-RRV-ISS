#include <stdint.h>
#include <string.h>

#include "md5.h"

#include "../common/uart_printf.h"

#if 1
#define usr_printf(fmt, ...) uart_printf(fmt, ##__VA_ARGS__)
#else
#define usr_printf(fmt, ...) \
    do {                     \
    } while (0)
#endif

static unsigned char expected[] = { 0x6d, 0x8d, 0x0b, 0x2c, 0x6a, 0x06, 0xf8, 0x8e,
                                    0x13, 0x16, 0x7f, 0xff, 0xac, 0xb1, 0xb7, 0x4e };

static int compare_result(const uint8_t* result) {
    for (int i = 0; i < 16; ++i) {
        if (result[i] != expected[i]) {
            return i + 1;
        }
    }

    return 0;
}

/*
 * Functions that run the algorithm on the provided input and put the digest into result.
 * result should be able to store 16 bytes.
 */
static void md5String(const char* input, uint8_t* result) {
    MD5Context ctx;

    memset(&ctx, 0, sizeof(ctx));

    md5Init(&ctx);
    dump_MD5Context("After md5Init()", &ctx);

    md5Update(&ctx, (const uint8_t*)input, strlen(input));
    dump_MD5Context("After md5Update()", &ctx);

    md5Finalize(&ctx);
    dump_MD5Context("After md5Finalize()", &ctx);

    memcpy(result, ctx.digest, 16);
}

#if 0
void md5File(FILE *file, uint8_t *result){
    char *input_buffer = malloc(1024);
    size_t input_size = 0;

    MD5Context ctx;
    md5Init(&ctx);

    while((input_size = fread(input_buffer, 1, 1024, file)) > 0){
        md5Update(&ctx, (const uint8_t *)input_buffer, input_size);
    }

    md5Finalize(&ctx);

    free(input_buffer);

    memcpy(result, ctx.digest, 16);
}
#endif

int main(void) {
    static uint8_t calc_result[16];
    const char* input_str = "1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    md5String(input_str, calc_result);
    int ret = compare_result(calc_result);
    if (ret == 0) {
        uart_printf("Check MD5 sum SUCCESS\n");
    } else {
        uart_printf("Check MD5 sum FAILURE: %d\n", ret);
    }
    return ret;
}
