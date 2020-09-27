#include "interface.h"
#include <stdio.h>

//void cool_function(int i, char c, CoolStruct* cs) {
//	printf("Calling C code!!!");
//}

extern void trigger1();

int square(int value) {
	trigger1();
	return value * value * value;
}
