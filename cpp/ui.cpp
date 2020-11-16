#include "../binja-rs/binaryninjacore-sys/binaryninja-api/binaryninjaapi.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/binaryninjacore.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/filecontext.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/viewframe.h"

#include <qt5/QtWidgets/QMessageBox>

#include "ui.h"

void ui_init() {
	//BinaryNinja::LogInfo("Initializing C++ plugin");
	puts("Initializing C++ plugin");
}

extern "C"
{
	extern void call_rust();
	
	void call_cpp() {
		puts("Called cpp from rust");
		ui_init();
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
