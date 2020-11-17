#pragma once

#include <qt5/QtWidgets/QWidget>
#include <qt5/QtWidgets/QScrollArea>
#include <qt5/QtWidgets/QPushButton>

#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/filecontext.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/viewframe.h"

class TestTab: public QScrollArea, public View
{
	BinaryViewRef m_data;
	uint64_t m_currentOffset = 0;
	//ByteView* m_byteView = nullptr;
	QPushButton* m_fullAnalysisButton = nullptr;

public:
	TestTab(QWidget* parent, BinaryViewRef data);

	virtual BinaryViewRef getData() override;
	virtual uint64_t getCurrentOffset() override;
	virtual void getSelectionOffsets(uint64_t& begin, uint64_t& end) override;
	virtual QFont getFont() override;
	virtual bool navigate(uint64_t addr) override;

	void setCurrentOffset(uint64_t offset);
	void navigateToFileOffset(uint64_t offset);

protected:
	virtual void focusInEvent(QFocusEvent* event) override;

private Q_SLOTS:
	void startFullAnalysis();
};


class TestTabType: public ViewType
{
public:
	TestTabType();
	virtual int getPriority(BinaryViewRef data, const QString& filename) override;
	virtual QWidget* create(BinaryViewRef data, ViewFrame* frame) override;
};
