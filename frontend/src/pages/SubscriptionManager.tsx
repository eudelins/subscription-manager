import { useState, useEffect } from 'react';

import { Row, Space } from 'antd';
import AddSubscriptionButton from '../components/AddSubcriptionButton';
import ArchiveSubscriptionsButton from '../components/ArchiveSubscriptionsButton';
import DividerWithTitle from '../components/DividerWithTitle';
import SubscriptionsGrid from '../components/SubscriptionsGrid';

import { getAllSubscriptions } from '../services/subscriptions';
import Subscription from '../interfaces/subscriptions/subscription.interface';

const SPACE_BETWEEN_CELLS = 16;

function SubscriptionManager() {
  const [subscriptions, setSubscriptions] = useState<Subscription[]>([]);
  const [subscriptionsToArchive, setSubscriptionsToArchive] = useState<Subscription[]>([]);
  const [archiveMode, setArchiveMode] = useState(false);

  useEffect(() => {
    getAllSubscriptions().then((res) => setSubscriptions(res));
  }, []);

  const changeSubscriptionStatus = (index: number) => {
    setSubscriptionsToArchive((prevSubscriptionsToArchive: Subscription[]) => {
      const updatedSubscriptionsToArchive = [...prevSubscriptionsToArchive];
      const archivedSubIndex = updatedSubscriptionsToArchive.findIndex((sub) => {
        return sub.id === subscriptions[index].id;
      });
      if (archivedSubIndex === -1) {
        updatedSubscriptionsToArchive.push(subscriptions[index]);
      } else {
        updatedSubscriptionsToArchive.splice(archivedSubIndex, 1);
      }
      return updatedSubscriptionsToArchive;
    });
  };

  const activeSubscriptions = subscriptions.filter((s) => s.status);
  const archivedSubscriptions = subscriptions.filter((s) => !s.status);

  return (
    <>
      <Row justify="end">
        <Space style={{ marginBottom: SPACE_BETWEEN_CELLS * 2 }} size="middle">
          <ArchiveSubscriptionsButton
            archiveMode={archiveMode}
            setArchiveMode={setArchiveMode}
            subsToArchive={subscriptionsToArchive}
          />
          <AddSubscriptionButton />
        </Space>
      </Row>
      {activeSubscriptions.length > 0 && <DividerWithTitle title="Vos abonnements en cours" />}
      <SubscriptionsGrid
        subs={activeSubscriptions}
        archiveMode={archiveMode}
        cellsSpacing={SPACE_BETWEEN_CELLS}
        changeSubscriptionStatus={changeSubscriptionStatus}
      />
      {archivedSubscriptions.length > 0 && (
        <DividerWithTitle
          title="Vos abonnements archivés"
          style={{ marginTop: SPACE_BETWEEN_CELLS * 4 }}
        />
      )}
      <SubscriptionsGrid
        subs={archivedSubscriptions}
        archiveMode={false}
        cellsSpacing={SPACE_BETWEEN_CELLS}
        changeSubscriptionStatus={(_) => {}}
      />
    </>
  );
}

export default SubscriptionManager;
