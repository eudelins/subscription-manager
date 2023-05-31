import { Button } from 'antd';

function AddSubscriptionButton() {
  return (
    <Button shape="round" size="large" style={buttonStyle}>
      Ajouter un nouvel abonnement
    </Button>
  );
}

const buttonStyle: React.CSSProperties = {
  backgroundColor: '#1DB954',
  color: 'white',
  fontSize: 20,
  fontWeight: 'bold',
  height: 65
};

export default AddSubscriptionButton;
