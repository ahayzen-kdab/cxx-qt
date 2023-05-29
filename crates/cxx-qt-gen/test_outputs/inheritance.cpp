#include "cxx-qt-gen/inheritance.cxxqt.h"

MyObject::MyObject(QObject* parent)
  : QAbstractItemModel(parent)
  , m_rustObj(cxx_qt_my_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
{
}

MyObject::~MyObject() {}

MyObjectRust const&
MyObject::unsafeRust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafeRustMut()
{
  return *m_rustObj;
}

QVariant
MyObject::data(QModelIndex const& _index, ::std::int32_t _role) const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return m_rustObj->dataWrapper(*this, _index, _role);
}

bool
MyObject::hasChildren(QModelIndex const& _parent) const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return m_rustObj->hasChildrenWrapper(*this, _parent);
}

namespace cxx_qt_my_object {
::std::unique_ptr<MyObject>
newCppObject()
{
  return ::std::make_unique<MyObject>();
}
} // namespace cxx_qt_my_object
