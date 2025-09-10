#pragma once

#include "rust/cxx.h"

#include <QtCore/QString>

namespace rust {

template<>
struct IsRelocatable<QString> : ::std::true_type
{};

}
