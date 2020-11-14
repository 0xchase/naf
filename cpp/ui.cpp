#include "../binja-rs/binaryninjacore-sys/binaryninja-api/binaryninjaapi.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/binaryninjacore.h"
#include "ui.h"

void ui_init(BinaryNinja::BinaryView *view, uint64_t start, uint64_t length) {
	printf("CALLING RUST!!!");
}

extern "C"
{
	extern void call_rust();
	
	void call_cpp() {
		puts("Called cpp from rust");
		call_rust();
	}

	/*
	BINARYNINJAPLUGIN bool UIPluginInit()
	{
			//PluginCommand::RegisterForRange("CHASE PLUGIN", "Fill region with breakpoint instructions.", &write_breakpoint);

			BinaryNinja::PluginCommand::RegisterForRange("CHASE PLUGIN", "SOME DESCRIPTION HERE", &call_rust);
											
		return true;
	}
	*/
}