#include <math.h>
#include <stdio.h>

#include "./sfc/sfc.h"

int main() {
    double x = 1.0;
    double ex = exp(x);
    double ax = sf_exp(x);
    printf("%.18g %.18g\n", ex, ax);
    return 0;
}
