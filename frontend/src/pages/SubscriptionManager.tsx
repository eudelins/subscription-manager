import { useState, useEffect } from 'react';

import { Row, Space } from 'antd';
import AddSubscriptionButton from 'components/buttons/AddSubcriptionButton';
import DividerWithTitle from 'components/DividerWithTitle';
import SubscriptionsGrid from 'components/SubscriptionsGrid';

import { getAllSubscriptions } from 'services/subscriptions';
import Subscription from 'interfaces/subscriptions/subscription.interface';
import ChangeSubscriptionsStatusButton, {
  StatusMode
} from 'components/buttons/ChangeSubscriptionsStatusButton';

const SPACE_BETWEEN_CELLS = 16;

function SubscriptionManager() {
  const [subscriptions, setSubscriptions] = useState<Subscription[]>([]);
  const [subscriptionsToArchive, setSubscriptionsToArchive] = useState<Subscription[]>([]);
  const [archiveMode, setArchiveMode] = useState(false);
  const [subscriptionsToActivate, setSubscriptionsToActivate] = useState<Subscription[]>([]);
  const [activateMode, setActivateMode] = useState(false);

  useEffect(() => {
    getAllSubscriptions().then((res) => setSubscriptions(res));
  }, []);

  const changeSubscriptionStatus = (subscription: Subscription, statusMode: StatusMode) => {
    const setSubscriptionsToChangeStatus =
      statusMode === StatusMode.Activate ? setSubscriptionsToActivate : setSubscriptionsToArchive;

    setSubscriptionsToChangeStatus((prevSubscriptionsChangeStatus: Subscription[]) => {
      const updatedSubscriptionsChangeStatus = [...prevSubscriptionsChangeStatus];
      const updatedSubIndex = updatedSubscriptionsChangeStatus.findIndex((sub) => {
        return sub.id === subscription.id;
      });
      if (updatedSubIndex === -1) {
        updatedSubscriptionsChangeStatus.push(subscription);
      } else {
        updatedSubscriptionsChangeStatus.splice(updatedSubIndex, 1);
      }
      return updatedSubscriptionsChangeStatus;
    });
  };

  const refreshSubscriptions = () => {
    getAllSubscriptions().then((res) => {
      setSubscriptions(res);
      setSubscriptionsToActivate([]);
      setSubscriptionsToArchive([]);
    });
  };

  const activeSubscriptions = subscriptions.filter((s) => s.status);
  const archivedSubscriptions = subscriptions.filter((s) => !s.status);

  return (
    <>
      <Row justify="end">
        <Space style={{ marginBottom: SPACE_BETWEEN_CELLS * 2 }} size="middle">
          <ChangeSubscriptionsStatusButton
            statusMode={StatusMode.Archive}
            changeStatusMode={archiveMode}
            setChangeStatusMode={setArchiveMode}
            subsToChangeStatus={subscriptionsToArchive}
            refreshSubscriptions={refreshSubscriptions}
          />
          <AddSubscriptionButton />
        </Space>
      </Row>
      {activeSubscriptions.length > 0 && <DividerWithTitle title="Vos abonnements en cours" />}
      <SubscriptionsGrid
        subs={activeSubscriptions}
        changeStatusMode={archiveMode}
        cellsSpacing={SPACE_BETWEEN_CELLS}
        changeSubscriptionStatus={(sub) => changeSubscriptionStatus(sub, StatusMode.Archive)}
      />
      {archivedSubscriptions.length > 0 && (
        <>
          <Row justify="end">
            <Space
              style={{ marginBottom: SPACE_BETWEEN_CELLS * 2, marginTop: SPACE_BETWEEN_CELLS * 4 }}
              size="middle">
              <ChangeSubscriptionsStatusButton
                statusMode={StatusMode.Activate}
                changeStatusMode={activateMode}
                setChangeStatusMode={setActivateMode}
                subsToChangeStatus={subscriptionsToActivate}
                refreshSubscriptions={refreshSubscriptions}
              />
            </Space>
          </Row>
          <DividerWithTitle title="Vos abonnements archivés" />
        </>
      )}
      <SubscriptionsGrid
        subs={archivedSubscriptions}
        changeStatusMode={activateMode}
        cellsSpacing={SPACE_BETWEEN_CELLS}
        changeSubscriptionStatus={(index) => changeSubscriptionStatus(index, StatusMode.Activate)}
      />
    </>
  );
}

export default SubscriptionManager;
