#include "testui.h"
#include <QtWidgets/QVBoxLayout>

TestUI::TestUI(UIContext* context) {
    m_context = context;
    m_actionHandler.setupActionHandler(this);
    
    QVBoxLayout* layout = new QVBoxLayout();
    layout->setContentsMargins(0, 0, 0, 0);
    
    m_tree = new QTreeView(this);
    m_tree->setColumnWidth(0, 500);
    layout->addWidget(m_tree, 1);

    setLayout(layout);
}