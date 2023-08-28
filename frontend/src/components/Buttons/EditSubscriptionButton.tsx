import { useState, CSSProperties } from 'react';

import { Button, Space, Modal } from 'antd';
import { EditOutlined } from '@ant-design/icons';

interface Props {
  editMode: boolean;
  setEditMode: (editMode: boolean) => void;
  submitRef?: any;
}

function EditSubscriptionButton({ editMode, setEditMode, submitRef }: Props) {
  const [openModal, setOpenModal] = useState(false);

  const handleButtonClick = () => {
    if (editMode) {
      setOpenModal(true);
    } else {
      setEditMode(!editMode);
    }
  };

  const handleModalConfirm = async () => {
    submitRef.current.click();
    setOpenModal(false);
    setEditMode(false);
  };

  const archiveButtonStyle: CSSProperties = {
    width: editMode ? 400 : 75,
    transition: 'width 0.3s ease-in-out',
    fontSize: 17,
    fontWeight: 'bold',
    height: 65,
    backgroundColor: 'blue',
    color: 'white',
    overflow: 'hidden'
  };

  return (
    <>
      <Button onClick={handleButtonClick} style={archiveButtonStyle} type="primary" shape="round">
        <Space>
          <EditOutlined />
          {editMode ? "Modifier l'abonnement" : ''}
        </Space>
      </Button>
      <Modal
        title="Modifier l'abonnement ?"
        open={openModal}
        onOk={handleModalConfirm}
        onCancel={() => setOpenModal(false)}
        okText="Ok"
        cancelText="Annuler">
        <p>Voulez-vous vraiment modifier l&apos;abonnement ?</p>
        <br />
      </Modal>
    </>
  );
}

export default EditSubscriptionButton;
