// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QAbstractListModel>

class QAbstractListModelCXX : public QAbstractListModel
{
public:
  explicit QAbstractListModelCXX(QObject* parent = nullptr)
    : QAbstractListModel(parent)
  {
  }

  // Proxy Qt API to more CXX friendly API
  QVariant data(const QModelIndex& index, int role) const override
  {
    return data(index.row(), role);
  }
  QHash<int, QByteArray> roleNames() const override
  {
    QHash<int, QByteArray> names;
    for (int i = 0; i < roleNameCount(); i++) {
      names.insert(i, roleName(i).toLocal8Bit());
    }
    return names;
  }
  int rowCount(const QModelIndex& parent) const override
  {
    Q_UNUSED(parent);
    return rowCount();
  }

  // Define a CXX friendly API
  virtual QVariant data(int index, int role) const
  {
    Q_UNUSED(index);
    Q_UNUSED(role);
    return QVariant();
  }
  // TODO: could be a list of strings
  // virtual rust::Vec<std::unique_ptr<QString>> roleNames() const = 0;
  virtual QString roleName(int index) const
  {
    Q_UNUSED(index);
    return QString();
  }
  virtual int roleNameCount() const { return 0; }
  virtual int rowCount() const { return 0; }

  // Can't define beginResetModel/endResetModel in CXX as they are protected?
  void resetModel() {
    beginResetModel();
    endResetModel();
  }
};
