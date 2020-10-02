#include <inttypes.h>
#include "binaryninjaapi.h"
#include "lowlevelilinstruction.h"
#include "mediumlevelilinstruction.h"
#include "binaryninjacore.h"

#include <stdio.h>

//void cool_function(int i, char c, CoolStruct* cs) {
//	printf("Calling C code!!!");
//}

void call_rust(BinaryNinja::BinaryView *view, uint64_t start, uint64_t length) {
	printf("\nCALLING C++\n");
}

extern "C"
{
	#include "interface.h"


	extern void trigger1();

	int square(int value) {
		trigger1();
		return value * value * value;
	}

	BINARYNINJAPLUGIN bool CorePluginInit()
	{
			// Register the plugin with Binary Ninja        
			
			//PluginCommand::RegisterForRange("CHASE PLUGIN", "Fill region with breakpoint instructions.", &write_breakpoint);

			BinaryNinja::PluginCommand::RegisterForRange("CHASE PLUGIN", "SOME DESCRIPTION HERE", &call_rust);
											
		return true;
	}
}
