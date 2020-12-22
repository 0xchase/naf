#include "../binja-rs/binaryninjacore-sys/binaryninja-api/binaryninjaapi.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/binaryninjacore.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/uitypes.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/uicontext.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/filecontext.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/viewframe.h"

#include <qt5/QtWidgets/QMessageBox>
#include <qt5/QtWidgets/QGroupBox>

#include <iostream>

#include "ui.h"
#include "testui.h"
#include "testtab.h"
#include "testtabtype.h"
#include "mywidget.h"

void ui_init() {
	//BinaryNinja::LogInfo("Initializing C++ plugin");

	puts("Initializing C++ UI plugin");

	//ViewType::registerViewType(new TestTabType());
	
	//UIAction::registerAction("Open selected files");
	Menu::mainMenu("Tools")->addAction("TEST ANALYSIS PLUGIN WINDOW", "Other");
	
	//DockHandler *dock_handler = DockHandler::getActiveDockHandler();

	//QWidget *mywidget = &MyWidget();
	//dock_handler->addDockWidget(mywidget);

	
	UIActionHandler::globalActions()->bindAction("TEST ANALYSIS PLUGIN WINDOW", UIAction([](const UIActionContext& context) {
		UIContext* currentContext = context.context;
		if (!currentContext)
			return;

		// Do not try to set the parent window when creating tabs, as this will create a parent relationship in
		// the bindings and will cause the widget to be destructed early. The correct parent will be assigned
		// when createTabForWidget is called.

		//TriageFilePicker* fp = new TriageFilePicker(currentContext);
		//currentContext->createTabForWidget("Open for Triage", fp);

		TestUI* fp = new TestUI(currentContext);
		currentContext->createTabForWidget("Demo stuff", fp);

		puts("Creating UI element");
	}));
}

void messageBox() {
	QMessageBox msgBox;
	msgBox.setText("Here is some example text");
	msgBox.exec();
}

void MessageBoxWarning(std::string s) {
	QMessageBox msgBox;
	msgBox.setText(QString::fromStdString(s));
	msgBox.exec();
}

extern "C"
{
	extern void call_rust();
	
	void call_cpp() {
		puts("Called cpp from rust");
		ui_init();
		call_rust();
	}

	void call_ui() {
		MessageBoxWarning("You have been warned");
	}

	// BinaryNinja::PluginCommand::RegisterForRange("CHASE PLUGIN", "SOME DESCRIPTION HERE", &call_rust);

}
