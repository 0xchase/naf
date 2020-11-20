#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QGroupBox>

#include "testtab.h"
#include "testui.h"

#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/fontsettings.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/viewframe.h"

TestTab::TestTab(QWidget* parent, BinaryViewRef data): QScrollArea(parent)
{
	setupView(this);
	m_data = data;

	QWidget* container = new QWidget(this);
	QVBoxLayout* layout = new QVBoxLayout();

	QGroupBox* entropyGroup = new QGroupBox("Entropy", container);
	QVBoxLayout* entropyLayout = new QVBoxLayout();

	container->setLayout(layout);
	setWidgetResizable(true);
	setWidget(container);
}

QFont TestTab::getFont()
{ 
	return getMonospaceFont(this);
}

void TestTab::focusInEvent(QFocusEvent*)
{
	//m_byteView->setFocus(Qt::OtherFocusReason);
}

// --------------------- Type stuff ---------------------

BinaryViewRef TestTab::getData() {
    return m_data;
}

uint64_t TestTab::getCurrentOffset() {
    return 0x4000000;
}

void TestTab::setSelectionOffsets(BNAddressRange range) {
    puts("Setting selection offsets");
}

bool TestTab::navigate(uint64_t addr) {
    puts("Navigate");
    return true;
}

void TestTab::setCurrentOffset(uint64_t offset) {
    puts("Setting current offset");
}

void TestTab::navigateToFileOffset(uint64_t offset) {
    puts("Navigating to file offset");
}
