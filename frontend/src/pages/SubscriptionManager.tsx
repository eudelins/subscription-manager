import { useState, useEffect } from 'react';

import { Col, Row, Space } from 'antd';
import SubscriptionCell from '../components/SubscriptionCell';
import AddSubscriptionButton from '../components/AddSubcriptionButton';

import { getAllSubscriptions } from '../services/subscriptions';
import Subscription from '../interfaces/subscriptions/subscription.interface';

import ArchiveSubscriptionsButton from '../components/ArchiveSubscriptionsButton';

const NUMBER_OF_CELLS_IN_GRID = 24;
const NUMBER_OF_SUBS_IN_ROW = 4;
const SPACE_BETWEEN_CELLS = 16;

function SubscriptionManager() {
  const [subscriptions, setSubscriptions] = useState<Subscription[]>([]);
  const [archiveMode, setArchiveMode] = useState(false);

  useEffect(() => {
    getAllSubscriptions().then((res) => setSubscriptions(res));
  }, []);

  const changeSubscriptionStatus = (index: number, newStatus: boolean) => {
    setSubscriptions((prevSubscriptions: Subscription[]) => {
      const updatedSubscriptions = [...prevSubscriptions];
      updatedSubscriptions[index] = {
        ...updatedSubscriptions[index],
        status: newStatus
      };
      return updatedSubscriptions;
    });
  };

  return (
    <>
      <Row justify="end">
        <Space style={{ marginBottom: SPACE_BETWEEN_CELLS * 2 }} size="middle">
          <ArchiveSubscriptionsButton
            archiveMode={archiveMode}
            setArchiveMode={setArchiveMode}
            subsToArchive={subscriptions.filter((s) => !s.status)}
          />
          <AddSubscriptionButton />
        </Space>
      </Row>
      {createRange(getNumberOfRows(subscriptions)).map((rowIndex) => {
        return (
          <Row
            gutter={SPACE_BETWEEN_CELLS}
            key={rowIndex}
            style={{ marginBottom: SPACE_BETWEEN_CELLS }}>
            {getOneRow(subscriptions, rowIndex).map((sub, index) => {
              return (
                <Col span={NUMBER_OF_CELLS_IN_GRID / NUMBER_OF_SUBS_IN_ROW} key={sub.id}>
                  <SubscriptionCell
                    subscription={sub}
                    archiveMode={archiveMode}
                    onStatusUpdate={(newStatus: boolean) =>
                      changeSubscriptionStatus(index, newStatus)
                    }
                  />
                </Col>
              );
            })}
          </Row>
        );
      })}
    </>
  );
}

function getNumberOfRows(subscriptions: Subscription[]): number {
  return ((subscriptions.length / NUMBER_OF_SUBS_IN_ROW) | 0) + 1;
}

function createRange(length: number): number[] {
  return [...Array(length).keys()];
}

function getOneRow(subscriptions: Subscription[], rowIndex: number): Subscription[] {
  return subscriptions.filter((_, index) => {
    return (
      index >= rowIndex * NUMBER_OF_SUBS_IN_ROW && index < (rowIndex + 1) * NUMBER_OF_SUBS_IN_ROW
    );
  });
}

export default SubscriptionManager;
