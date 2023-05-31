import { useState, CSSProperties } from 'react';

import { Button, Space, Modal } from 'antd';
import { FolderOpenOutlined } from '@ant-design/icons';
import Subscription from '../interfaces/subscriptions/subscription.interface';

interface Props {
  archiveMode: boolean;
  setArchiveMode: (archiveMode: boolean) => void;
  subsToArchive: Subscription[];
}

function ArchiveSubscriptionsButton({ archiveMode, setArchiveMode, subsToArchive }: Props) {
  const [openModal, setOpenModal] = useState(false);

  const handleButtonClick = () => {
    if (archiveMode && subsToArchive.length > 0) {
      setOpenModal(true);
    } else {
      setArchiveMode(!archiveMode);
    }
  };

  const handleModalConfirm = () => {
    // TO DO : API CALL
    console.log('Archiving subscriptions : ');
    console.log(subsToArchive);
    setOpenModal(false);
    setArchiveMode(false);
  };

  const archiveButtonStyle: CSSProperties = {
    width: archiveMode ? 400 : 75,
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
        style={archiveButtonStyle}
        type="primary"
        shape="round"
        danger>
        <Space>
          <FolderOpenOutlined />
          {archiveMode ? 'Archiver les abonnements sélectionnés' : ''}
        </Space>
      </Button>
      <Modal
        title="Archiver les abonnements ?"
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

export default ArchiveSubscriptionsButton;