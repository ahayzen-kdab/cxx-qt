// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <optional>

#include <QAbstractListModel>
#include <QObject>

class Sensor : public QObject
{
  Q_OBJECT

  Q_PROPERTY(
    QAbstractListModel* model READ model WRITE setModel NOTIFY modelChanged)
  Q_PROPERTY(bool online READ online NOTIFY onlineChanged)
  Q_PROPERTY(double power READ power NOTIFY powerChanged)
  Q_PROPERTY(QString uuid READ uuid WRITE setUuid NOTIFY uuidChanged)

public:
  Sensor(QObject* parent = nullptr);

  QAbstractListModel* model() const;
  bool online() const;
  double power() const;
  QString uuid() const;

public Q_SLOTS:
  void setModel(QAbstractListModel* model);
  void setUuid(const QString& uuid);

private Q_SLOTS:
  void findUuid();
  void onModelDataChanged(const QModelIndex& topLeft,
                          const QModelIndex& bottomRight,
                          const QVector<int>& roles);

Q_SIGNALS:
  void modelChanged();
  void onlineChanged();
  void powerChanged();
  void uuidChanged();

private:
  std::optional<int> m_index;
  QAbstractListModel* m_model;
  QString m_uuid;
};
