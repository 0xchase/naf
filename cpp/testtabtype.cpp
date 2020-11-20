#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QGroupBox>

#include "testtabtype.h"
#include "testtab.h"

#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/fontsettings.h"

// --------------------- Type stuff ---------------------

TestTabType::TestTabType(): ViewType("Triage2", "Triage Summary2")
{
    const QString name = "lsjdf";
    const QString longname = "lsjdf";

    //TestTabType::ViewType(name, longname);
}


QWidget* TestTabType::create(BinaryViewRef data, ViewFrame* frame)
{
	return new TestTab(frame, data);
}

int TestTabType::getPriority(BinaryViewRef data, const QString&)
{
	return 1;
}
