import { Col, Row } from 'antd';

import SubscriptionCell from '../components/SubscriptionCell';

import Subscription from '../interfaces/subscriptions/subscription.interface';

const NUMBER_OF_CELLS_IN_GRID = 24;
const NUMBER_OF_SUBS_IN_ROW = 4;

interface Props {
  subs: Subscription[];
  archiveMode: boolean;
  cellsSpacing: number;
  changeSubscriptionStatus: (index: number) => void;
}

function SubscriptionsGrid({ subs, archiveMode, cellsSpacing, changeSubscriptionStatus }: Props) {
  return (
    <>
      {createRange(getNumberOfRows(subs)).map((rowIndex) => {
        return (
          <Row gutter={cellsSpacing} key={rowIndex} style={{ marginBottom: cellsSpacing }}>
            {getOneRow(subs, rowIndex).map((sub, index) => {
              return (
                <Col span={NUMBER_OF_CELLS_IN_GRID / NUMBER_OF_SUBS_IN_ROW} key={sub.id}>
                  <SubscriptionCell
                    subscription={sub}
                    archiveMode={archiveMode}
                    onStatusUpdate={() => changeSubscriptionStatus(index)}
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

function getNumberOfRows(subs: Subscription[]): number {
  return ((subs.length / NUMBER_OF_SUBS_IN_ROW) | 0) + 1;
}

function createRange(length: number): number[] {
  return [...Array(length).keys()];
}

function getOneRow(subs: Subscription[], rowIndex: number): Subscription[] {
  return subs.filter((_, index) => {
    return (
      index >= rowIndex * NUMBER_OF_SUBS_IN_ROW && index < (rowIndex + 1) * NUMBER_OF_SUBS_IN_ROW
    );
  });
}

export default SubscriptionsGrid;
