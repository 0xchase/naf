#include <inttypes.h>
#include <binaryninjaapi.h>
#include <binaryninjacore.h>

void trigger_plugin(BinaryNinja::BinaryView *view) {
	println("Triggered plugin");
}

extern "C"
{
  BINARYNINJAPLUGIN bool CorePluginInit()
  {
	PluginCommand::RegisterForRange("CHASE PLUGIN", "SOME DESCRIPTION HERE", &trigger_plugin);
	return true;
  }
}
