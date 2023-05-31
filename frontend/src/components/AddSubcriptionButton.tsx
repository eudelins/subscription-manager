import { CSSProperties } from 'react';
import { useNavigate } from 'react-router-dom';
import { SUBSCRIPTION_CREATOR_ROUTE } from '../routes/routes';

import { Button } from 'antd';
import { PlusCircleOutlined } from '@ant-design/icons';

function AddSubscriptionButton() {
  const navigate = useNavigate();
  const handleClick = () => navigate(SUBSCRIPTION_CREATOR_ROUTE);

  return (
    <Button shape="round" size="large" style={buttonStyle} onClick={handleClick}>
      <PlusCircleOutlined />
      Ajouter un nouvel abonnement
    </Button>
  );
}

const buttonStyle: CSSProperties = {
  backgroundColor: '#1DB954',
  color: 'white',
  fontSize: 17,
  fontWeight: 'bold',
  height: 65
};

export default AddSubscriptionButton;
