#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
  , m_cxxQtThreadObj(
      ::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(
        this))
{
}

MyObject::~MyObject()
{
  const auto guard = ::std::unique_lock(m_cxxQtThreadObj->mutex);
  m_cxxQtThreadObj->ptr = nullptr;
}

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

::std::unique_ptr<MyObjectCxxQtThread>
MyObject::qtThread() const
{
  return ::std::make_unique<MyObjectCxxQtThread>(m_cxxQtThreadObj,
                                                 m_rustObjMutex);
}

void
MyObject::invokable()
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableWrapper(*this);
}

void
MyObject::emitReady()
{
  Q_EMIT ready();
}

::QMetaObject::Connection
MyObject::readyConnect(::rust::Fn<void(MyObject&)> func,
                       ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::ready,
    this,
    [&, func = ::std::move(func)]() {
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
      func(*this);
    },
    type);
}

void
MyObject::emitDataChanged(::std::int32_t first,
                          ::std::unique_ptr<Opaque> second,
                          QPoint third,
                          QPoint const& fourth)
{
  Q_EMIT dataChanged(
    ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t, ::std::int32_t>{}(
      ::std::move(first)),
    ::rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<Opaque>,
                                      ::std::unique_ptr<Opaque>>{}(
      ::std::move(second)),
    ::rust::cxxqtlib1::cxx_qt_convert<QPoint, QPoint>{}(::std::move(third)),
    ::rust::cxxqtlib1::cxx_qt_convert<QPoint const&, QPoint const&>{}(
      ::std::move(fourth)));
}

::QMetaObject::Connection
MyObject::dataChangedConnect(::rust::Fn<void(MyObject&,
                                             ::std::int32_t first,
                                             ::std::unique_ptr<Opaque> second,
                                             QPoint third,
                                             QPoint const& fourth)> func,
                             ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::dataChanged,
    this,
    [&, func = ::std::move(func)](::std::int32_t first,
                                  ::std::unique_ptr<Opaque> second,
                                  QPoint third,
                                  QPoint const& fourth) {
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
      func(
        *this,
        ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t, ::std::int32_t>{}(
          ::std::move(first)),
        ::rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<Opaque>,
                                          ::std::unique_ptr<Opaque>>{}(
          ::std::move(second)),
        ::rust::cxxqtlib1::cxx_qt_convert<QPoint, QPoint>{}(::std::move(third)),
        ::rust::cxxqtlib1::cxx_qt_convert<QPoint const&, QPoint const&>{}(
          ::std::move(fourth)));
    },
    type);
}

void
MyObject::emitNewData(::std::int32_t first,
                      ::std::unique_ptr<Opaque> second,
                      QPoint third,
                      QPoint const& fourth)
{
  Q_EMIT newData(
    ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t, ::std::int32_t>{}(
      ::std::move(first)),
    ::rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<Opaque>,
                                      ::std::unique_ptr<Opaque>>{}(
      ::std::move(second)),
    ::rust::cxxqtlib1::cxx_qt_convert<QPoint, QPoint>{}(::std::move(third)),
    ::rust::cxxqtlib1::cxx_qt_convert<QPoint const&, QPoint const&>{}(
      ::std::move(fourth)));
}

::QMetaObject::Connection
MyObject::newDataConnect(::rust::Fn<void(MyObject&,
                                         ::std::int32_t first,
                                         ::std::unique_ptr<Opaque> second,
                                         QPoint third,
                                         QPoint const& fourth)> func,
                         ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::newData,
    this,
    [&, func = ::std::move(func)](::std::int32_t first,
                                  ::std::unique_ptr<Opaque> second,
                                  QPoint third,
                                  QPoint const& fourth) {
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
      func(
        *this,
        ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t, ::std::int32_t>{}(
          ::std::move(first)),
        ::rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<Opaque>,
                                          ::std::unique_ptr<Opaque>>{}(
          ::std::move(second)),
        ::rust::cxxqtlib1::cxx_qt_convert<QPoint, QPoint>{}(::std::move(third)),
        ::rust::cxxqtlib1::cxx_qt_convert<QPoint const&, QPoint const&>{}(
          ::std::move(fourth)));
    },
    type);
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
::std::unique_ptr<MyObject>
newCppObject()
{
  return ::std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object
