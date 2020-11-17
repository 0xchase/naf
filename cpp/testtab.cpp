#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QGroupBox>

#include "testtab.h"
#include "testui.h"

#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/fontsettings.h"


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

	if (m_fullAnalysisButton && (BinaryNinja::Settings::Instance()->Get<std::string>("analysis.mode", data) == "full"))
		m_fullAnalysisButton->hide();
}


BinaryViewRef TestTab::getData()
{
	return m_data;
}


uint64_t TestTab::getCurrentOffset()
{
	return m_currentOffset;
}


void TestTab::getSelectionOffsets(uint64_t& begin, uint64_t& end)
{
	begin = m_currentOffset;
	end = m_currentOffset;
}


void TestTab::setCurrentOffset(uint64_t offset)
{
	m_currentOffset = offset;
	UIContext::updateStatus(true);
}


QFont TestTab::getFont()
{ 
	return getMonospaceFont(this);
}


bool TestTab::navigate(uint64_t addr)
{
	return false;
}


void TestTab::startFullAnalysis()
{
	BinaryNinja::Settings::Instance()->Set("analysis.mode", "full", m_data);
	for (auto& f: m_data->GetAnalysisFunctionList())
	{
		if (f->IsAnalysisSkipped())
			f->Reanalyze();
	}
	m_data->UpdateAnalysis();
	m_fullAnalysisButton->hide();
}


void TestTab::navigateToFileOffset(uint64_t offset)
{
    ViewFrame* frame = ViewFrame::viewFrameForWidget(this);
    if (frame)
        frame->navigate("Hex:Raw", offset);
}


void TestTab::focusInEvent(QFocusEvent*)
{
	//m_byteView->setFocus(Qt::OtherFocusReason);
}


TestTabType::TestTabType(): ViewType("Triage", "Triage Summary")
{
}


int TestTabType::getPriority(BinaryViewRef data, const QString&)
{
	BinaryNinja::Ref<BinaryNinja::Settings> settings = BinaryNinja::Settings::Instance();
	auto analysisMode = settings->Get<std::string>("analysis.mode", data);
	bool full = analysisMode == "full";
	bool intermediate = analysisMode == "intermediate";
	bool alwaysPrefer = settings->Get<bool>("triage.preferSummaryView", data);
	bool preferForRaw = settings->Get<bool>("triage.preferSummaryViewForRaw", data);
	if (data->IsExecutable() && (alwaysPrefer || (!full && !intermediate)))
		return 100;
	if (data->GetLength() > 0)
	{
		if (alwaysPrefer || data->IsExecutable() || preferForRaw)
			return 25;
		return 1;
	}
	return 0;
}


QWidget* TestTabType::create(BinaryViewRef data, ViewFrame* frame)
{
	return new TestTab(frame, data);
}

