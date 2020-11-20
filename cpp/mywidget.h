#pragma once

#include <qt5/QtWidgets/QWidget>
#include <qt5/QtWidgets/QScrollArea>
#include <qt5/QtWidgets/QPushButton>

#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/filecontext.h"
#include "../binja-rs/binaryninjacore-sys/binaryninja-api/ui/viewframe.h"

class MyWidget: public QWidget {
public:
    MyWidget();
};
