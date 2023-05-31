import { CSSProperties } from 'react';
import { Card, Avatar, Checkbox } from 'antd';
import Subscription from '../interfaces/subscriptions/subscription.interface';

const LOGO_SIZE = 80;

interface Props {
  subscription: Subscription;
  archiveMode: boolean;
  onStatusUpdate: () => void;
}

function SubscriptionCell({ subscription, archiveMode, onStatusUpdate }: Props) {
  const url = '../../src-tauri/icons/icon.png'; // TO CHANGE

  const handleStatusChange = () => {
    onStatusUpdate();
  };

  return (
    <div style={{ display: 'grid', gridTemplateRows: 'auto 1fr' }}>
      <Card style={{ textAlign: 'center', border: '2px solid #f0f0f0' }} hoverable>
        <Avatar size={LOGO_SIZE} src={url} alt="logo" />
        <div style={cardTitleStyle}>{subscription.name.toUpperCase()}</div>
        <Checkbox
          onChange={handleStatusChange}
          style={{ visibility: archiveMode ? 'visible' : 'hidden' }}
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
