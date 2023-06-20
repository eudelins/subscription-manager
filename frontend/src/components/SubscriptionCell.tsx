import { CSSProperties, useState } from 'react';
import { Card, Avatar, Checkbox } from 'antd';
import Subscription from 'interfaces/subscriptions/subscription.interface';
import { useNavigate } from 'react-router-dom';
import { SUBSCRIPTION_PAGE_ROUTE } from 'routes/routes';

const LOGO_SIZE = 80;

interface Props {
  subscription: Subscription;
  changeStatusMode: boolean;
  onStatusUpdate: () => void;
}

function SubscriptionCell({ subscription, changeStatusMode, onStatusUpdate }: Props) {
  const url = '../src-tauri/icons/icon.png'; // TO CHANGE

  const [isChecked, setIsChecked] = useState(false);
  const navigate = useNavigate();

  const handleClick = () => {
    if (changeStatusMode) {
      setIsChecked(!isChecked);
      onStatusUpdate();
    } else {
      navigate(SUBSCRIPTION_PAGE_ROUTE.replace(':id', subscription.id.toString()));
    }
  };

  const handleStatusChange = () => {
    onStatusUpdate();
  };

  return (
    <div style={{ display: 'grid', gridTemplateRows: 'auto 1fr' }}>
      <Card
        style={{ textAlign: 'center', border: '2px solid #f0f0f0' }}
        hoverable
        onClick={handleClick}>
        <Avatar size={LOGO_SIZE} src={url} alt="logo" />
        <div style={cardTitleStyle}>{subscription.name.toUpperCase()}</div>
        <Checkbox
          checked={isChecked}
          onChange={handleStatusChange}
          style={{ visibility: changeStatusMode ? 'visible' : 'hidden' }}
        />
      </Card>
    </div>
  );
}

const cardTitleStyle: CSSProperties = {
  marginTop: 16,
  marginBottom: 16,
  fontSize: 20,
  fontWeight: 'bold'
};

export default SubscriptionCell;
