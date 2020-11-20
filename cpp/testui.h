#pragma once

#include <qt5/QtWidgets/QWidget>
#include <qt5/QtWidgets/QTreeView>
#include <qt5/QtWidgets/QFileSystemModel>

#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/uicontext.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/filecontext.h"


class TestUI: public QWidget { 
    UIContext* m_context;
    UIActionHandler m_actionHandler;
    Menu m_contextMenu;

    QTreeView* m_tree;

public:
        TestUI(UIContext* context);
};
