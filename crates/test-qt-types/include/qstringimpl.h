#pragma once

#include <QtCore/QString>

#include "rust/cxx.h"

::rust::Slice<const ::std::uint16_t>
qstringAsSlice(const QString& string);

// TODO: common namespace and header
template<typename T, typename... Args>
T
construct(Args... args)
{
  return T(args...);
}

template<typename T>
void
drop(T& value)
{
  value.~T();
}

template<typename T>
bool
operatorEq(const T& a, const T& b)
{
  return a == b;
}
