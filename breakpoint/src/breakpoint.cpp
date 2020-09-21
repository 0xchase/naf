#include <inttypes.h>
#include "binaryninjaapi.h"

using namespace BinaryNinja;
using namespace std;

void write_breakpoint(BinaryNinja::BinaryView *view, uint64_t start, uint64_t length)
{
  // Sample function to show registering a plugin menu item for a range of bytes.
  // Also possible:
  //   register
  //   register_for_address
  //   register_for_function

  for (auto& func : view->GetAnalysisFunctionList()) {
    Ref<Symbol> sym = func->GetSymbol();
    if (sym)
      printf("Function %s:\n", sym->GetFullName().c_str());
    else
      printf("Function at 0x%x:\n", func->GetStart());
  }
}

bool return_true() {
  return true;
}

extern "C"
{
  BINARYNINJAPLUGIN bool CorePluginInit()
  {
        // Register the plugin with Binary Ninja        
        
        //PluginCommand::RegisterForRange("CHASE PLUGIN", "Fill region with breakpoint instructions.", &write_breakpoint);

        PluginCommand::RegisterForRange("CHASE PLUGIN", "SOME DESCRIPTION HERE", &write_breakpoint);
                                        
    return true;
  }
}