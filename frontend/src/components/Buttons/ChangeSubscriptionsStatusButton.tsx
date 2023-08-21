import { useState, CSSProperties } from 'react';

import { Button, Space, Modal } from 'antd';
import { FolderOpenOutlined, ToTopOutlined } from '@ant-design/icons';
import Subscription from 'interfaces/subscriptions/subscription.interface';
import { activateSubscriptions, archiveSubscriptions } from 'services/subscriptions';

export enum StatusMode {
  Activate,
  Archive
}

interface Props {
  statusMode: StatusMode;
  changeStatusMode: boolean;
  setChangeStatusMode: (changeStatusMode: boolean) => void;
  subsToChangeStatus: Subscription[];
  refreshSubscriptions: () => void;
}

function ChangeSubscriptionsStatusButton({
  statusMode,
  changeStatusMode,
  setChangeStatusMode,
  subsToChangeStatus,
  refreshSubscriptions
}: Props) {
  const [openModal, setOpenModal] = useState(false);

  const handleButtonClick = () => {
    if (changeStatusMode && subsToChangeStatus.length > 0) {
      setOpenModal(true);
    } else {
      setChangeStatusMode(!changeStatusMode);
    }
  };

  const handleModalConfirm = async () => {
    if (statusMode === StatusMode.Archive) {
      await archiveSubscriptions(subsToChangeStatus);
    } else {
      await activateSubscriptions(subsToChangeStatus);
    }
    setOpenModal(false);
    setChangeStatusMode(false);
    refreshSubscriptions();
  };

  const buttonStyle: CSSProperties = {
    width: changeStatusMode ? 400 : 75,
    transition: 'width 0.3s ease-in-out',
    fontSize: 17,
    fontWeight: 'bold',
    height: 65,
    color: 'white',
    overflow: 'hidden'
  };

  return (
    <>
      <Button
        onClick={handleButtonClick}
        style={buttonStyle}
        type="primary"
        shape="round"
        danger={statusMode === StatusMode.Archive}>
        <Space>
          {statusMode === StatusMode.Archive ? <FolderOpenOutlined /> : <ToTopOutlined />}
          {changeStatusMode
            ? (statusMode === StatusMode.Archive ? 'Archiver' : 'Activer') +
              ' les abonnements sélectionnés'
            : ''}
        </Space>
      </Button>
      <Modal
        title={(statusMode === StatusMode.Archive ? 'Archiver' : 'Activer') + ' les abonnements ?'}
        open={openModal}
        onOk={handleModalConfirm}
        onCancel={() => setOpenModal(false)}
        okText="Ok"
        cancelText="Annuler">
        <p>Voulez-vous vraiment changer le statut des abonnements sélectionnés ?</p>
        <br />
      </Modal>
    </>
  );
}

export default ChangeSubscriptionsStatusButton;
