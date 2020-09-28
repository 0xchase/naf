#include "binaryninjaapi.h"

BINARYNINJAPLUGIN bool CorePluginInit() {
    // Register the plugin with Binary Ninja        
    
    //PluginCommand::RegisterForRange("CHASE PLUGIN", "Fill region with breakpoint instructions.", &write_breakpoint);

    PluginCommand::RegisterForRange("CHASE PLUGIN", "SOME DESCRIPTION HERE", &write_breakpoint);
                                    
return true;
}