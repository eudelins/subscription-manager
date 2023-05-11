import { useState, useEffect } from 'react';

import { Col, Row } from 'antd';
import SubscriptionCell from '../components/SubscriptionCell';
import AddSubscriptionButton from '../components/AddSubcriptionButton';

import { getAllSubscriptions } from '../services/subscriptions';
import Subscription from '../interfaces/subscriptions/subscription.interface';

const NUMBER_OF_CELLS_IN_GRID = 24;
const NUMBER_OF_SUBS_IN_ROW = 4;
const SPACE_BETWEEN_CELLS = 16;

function SubscriptionManager() {
  const [subscriptions, setSubscriptions] = useState<Subscription[]>([]);

  useEffect(() => {
    getAllSubscriptions().then((res) => setSubscriptions(res));
  }, []);

  return (
    <>
      <Row style={{ marginBottom: SPACE_BETWEEN_CELLS }}>
        <Col offset={16} span={8} style={{ textAlign: 'right' }}>
          <AddSubscriptionButton />
        </Col>
      </Row>
      {createRange(getNumberOfRows(subscriptions)).map((rowIndex) => {
        return (
          <Row
            gutter={SPACE_BETWEEN_CELLS}
            key={rowIndex}
            style={{ marginBottom: SPACE_BETWEEN_CELLS }}>
            {getOneRow(subscriptions, rowIndex).map((sub) => {
              return (
                <Col span={NUMBER_OF_CELLS_IN_GRID / NUMBER_OF_SUBS_IN_ROW} key={sub.id}>
                  <SubscriptionCell {...sub} />
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
  console.log(length);
  console.log([...Array(length).keys()]);
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
