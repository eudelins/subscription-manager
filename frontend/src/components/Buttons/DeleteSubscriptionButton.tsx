import { useState, CSSProperties } from 'react';

import { Button, Space, Modal } from 'antd';
import { DeleteOutlined } from '@ant-design/icons';
import Subscription from 'interfaces/subscriptions/subscription.interface';
import { SUBSCRIPTION_MANAGER_ROUTE } from 'routes/routes';
import { useNavigate } from 'react-router-dom';
import { deleteSubscription } from 'services/subscriptions';

interface Props {
  subscription: Subscription;
}

function DeleteSubscriptionsButton({ subscription }: Props) {
  const [openModal, setOpenModal] = useState(false);
  const navigate = useNavigate();

  const handleButtonClick = () => {
    setOpenModal(true);
  };

  const handleModalConfirm = async () => {
    await deleteSubscription(subscription);
    setOpenModal(false);
    navigate(SUBSCRIPTION_MANAGER_ROUTE);
  };

  const deleteButtonStyle: CSSProperties = {
    width: 400,
    fontSize: 17,
    fontWeight: 'bold',
    height: 65,
    color: 'white'
  };

  return (
    <>
      <Button
        onClick={handleButtonClick}
        style={deleteButtonStyle}
        type="primary"
        shape="round"
        danger>
        <Space>
          <DeleteOutlined />
          Supprimer l&apos;abonnement
        </Space>
      </Button>
      <Modal
        title="Supprimer l'abonnement ?"
        open={openModal}
        onOk={handleModalConfirm}
        onCancel={() => setOpenModal(false)}
        okText="Ok"
        cancelText="Annuler">
        <p>Voulez-vous vraiment changer supprimer cet abonnement de la base de donées ?</p>
        <br />
      </Modal>
    </>
  );
}

export default DeleteSubscriptionsButton;
