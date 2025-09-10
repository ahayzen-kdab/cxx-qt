#include "test-qt-types/include/qstringimpl.h"

::rust::Slice<const ::std::uint16_t>
qstringAsSlice(const QString& string)
{
  return ::rust::Slice<const ::std::uint16_t>(
    reinterpret_cast<const std::uint16_t*>(string.data()),
    static_cast<::std::size_t>(string.size()));
}
