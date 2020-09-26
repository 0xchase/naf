#include "interface.h"
#include <stdio.h>

void cool_function(int i, char c, CoolStruct* cs) {
	printf("Calling C code!!!");
}

int square(int value) {
	return value * value;
}
