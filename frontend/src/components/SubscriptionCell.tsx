import { Card, Avatar } from 'antd';
import Subscription from '../interfaces/subscriptions/subscription.interface';

const LOGO_SIZE = 80;

function SubscriptionCell(subscription: Subscription) {
  const url = '../../src-tauri/icons/icon.png'; // TO CHANGE
  return (
    <Card style={{ textAlign: 'center' }}>
      <Avatar size={LOGO_SIZE} src={url} alt="logo" />
      <div style={cardTitleStyle}>{subscription.name.toUpperCase()}</div>
    </Card>
  );
}

const cardTitleStyle: React.CSSProperties = {
  marginTop: 16,
  fontSize: 20,
  fontWeight: 'bold'
};

export default SubscriptionCell;
