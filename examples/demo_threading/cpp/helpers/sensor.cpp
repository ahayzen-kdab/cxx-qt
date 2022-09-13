// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "sensor.h"

enum EnergyUsageRoles
{
  Uuid,
  Power,
};

Sensor::Sensor(QObject* parent)
  : QObject(parent)
{
  connect(this, &Sensor::uuidChanged, this, &Sensor::findUuid);
  connect(this, &Sensor::modelChanged, this, &Sensor::findUuid);
}

void
Sensor::findUuid()
{
  if (m_model) {
    for (int i = 0; i < m_model->rowCount(); i++) {
      if (m_model->data(m_model->index(i), EnergyUsageRoles::Uuid).toString() ==
          m_uuid) {
        m_index = i;
        break;
      }
    }
  }

  Q_EMIT onlineChanged();
  Q_EMIT powerChanged();
}

QAbstractListModel*
Sensor::model() const
{
  return m_model;
}

bool
Sensor::online() const
{
  return m_index.has_value();
}

double
Sensor::power() const
{
  if (m_model && m_index) {
    return m_model
      ->data(m_model->index(m_index.value()), EnergyUsageRoles::Power)
      .toDouble();
  } else {
    return 0.0;
  }
}

void
Sensor::onModelDataChanged(const QModelIndex& topLeft,
                           const QModelIndex& bottomRight,
                           const QVector<int>& roles)
{
  if (m_index >= topLeft.row() && m_index <= bottomRight.row() &&
      roles.contains(EnergyUsageRoles::Power)) {
    Q_EMIT powerChanged();
  }
}

void
Sensor::setModel(QAbstractListModel* model)
{
  if (m_model != model) {
    if (m_model) {
      m_model->disconnect(this);
    }

    m_model = model;

    if (m_model) {
      connect(m_model,
              &QAbstractListModel::dataChanged,
              this,
              &Sensor::onModelDataChanged);
      connect(
        m_model, &QAbstractListModel::rowsInserted, this, &Sensor::findUuid);
      connect(
        m_model, &QAbstractListModel::rowsRemoved, this, &Sensor::findUuid);
      connect(
        m_model, &QAbstractListModel::modelReset, this, &Sensor::findUuid);
    }

    Q_EMIT modelChanged();
  }
}

void
Sensor::setUuid(const QString& uuid)
{
  if (m_uuid != uuid) {
    m_uuid = uuid;

    Q_EMIT uuidChanged();
  }
}

QString
Sensor::uuid() const
{
  return m_uuid;
}
