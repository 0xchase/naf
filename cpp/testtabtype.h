#pragma once

#include <qt5/QtWidgets/QWidget>
#include <qt5/QtWidgets/QScrollArea>
#include <qt5/QtWidgets/QPushButton>

#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/filecontext.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/viewframe.h"

class TestTabType: public ViewType
{
public:
	TestTabType();
	virtual int getPriority(BinaryViewRef data, const QString& filename) override;
	virtual QWidget* create(BinaryViewRef data, ViewFrame* frame) override;
};
