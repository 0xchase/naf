#include <inttypes.h>
#include "binaryninjaapi.h"
#include "lowlevelilinstruction.h"
#include "binaryninjacore.h"
#include <inttypes.h>

using namespace BinaryNinja;
using namespace std;

void print_llil(BinaryNinja::BinaryView *view);

void write_breakpoint(BinaryNinja::BinaryView *view, uint64_t start, uint64_t length)
{
  if (view->GetRecentAnalysisFunctionForAddress(start))
    printf("FOUND\n");

  if (view->GetAnalysisFunctionsForAddress(view->GetCurrentOffset()).size() > 0)
    printf("Found function\n");
  else
    printf("Didn't find functions\n");
    
  for (auto& func : view->GetAnalysisFunctionsForAddress(start)) {
    printf("Running loop\n");

    Ref<Symbol> sym = func->GetSymbol();

    if (sym)
      printf("Function %s:\n", sym->GetFullName().c_str());
    else
      printf("Function at 0x%x:\n", func->GetStart());

    Ref<LowLevelILFunction> il = func->GetLowLevelIL();

    if (!il) {
			printf("    Does not have LLIL\n\n");
			continue;
		}

		for (auto& block : il->GetBasicBlocks()) {
			for (size_t instrIndex = block->GetStart(); instrIndex < block->GetEnd(); instrIndex++) {
				LowLevelILInstruction instr = (*il)[instrIndex];

				vector<InstructionTextToken> tokens;
				il->GetInstructionText(func, func->GetArchitecture(), instrIndex, tokens);
				printf("    %" PRIdPTR " @ 0x%" PRIx64 "  ", instrIndex, instr.address);

				for (auto& token: tokens)
					printf("%s", token.text.c_str());

				printf("\n");

      }
    }
  }
  printf("Done running plugin\n");
}

void print_llil(BinaryNinja::BinaryView *view) {
  for (auto& func : view->GetAnalysisFunctionList()) {
    Ref<Symbol> sym = func->GetSymbol();
    if (sym)
      printf("Function %s:\n", sym->GetFullName().c_str());
    else
      printf("Function at 0x%x:\n", func->GetStart());

    Ref<LowLevelILFunction> il = func->GetLowLevelIL();

    if (!il)
		{
			printf("    Does not have LLIL\n\n");
			continue;
		}

		// Loop through all blocks in the function
		for (auto& block : il->GetBasicBlocks())
		{
			// Loop though each instruction in the block
			for (size_t instrIndex = block->GetStart(); instrIndex < block->GetEnd(); instrIndex++)
			{
				// Fetch IL instruction
				LowLevelILInstruction instr = (*il)[instrIndex];

				// Display core's intrepretation of the IL instruction
				vector<InstructionTextToken> tokens;
				il->GetInstructionText(func, func->GetArchitecture(), instrIndex, tokens);
				printf("    %" PRIdPTR " @ 0x%" PRIx64 "  ", instrIndex, instr.address);
				for (auto& token: tokens)
					printf("%s", token.text.c_str());
				printf("\n");

        
        // print il expressions

      }
    }
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
