#include <stdio.h>

unsigned int custom_abs(int a) {
    if (a > 0) {
        printf("positive: %d\n", a);
        return a;
    } else {
        printf("negative: %d\n", a);
        return -a;
    }
}
