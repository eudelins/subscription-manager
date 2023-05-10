import { useState, useEffect } from 'react';

import { Layout, theme } from 'antd';

import { getAllSubscriptions } from './services/subscriptions';
import Subscriptions from './interfaces/subscriptions/subscription.interface';
import Sidebar from './components/Sidebar';

const { Header, Content, Footer, Sider } = Layout;

function App() {
  const [subscriptions, setSubscriptions] = useState<Subscriptions[]>([]);
  const [isMenuCollapsed, setMenuCollapsed] = useState(false);

  const {
    token: { colorBgContainer }
  } = theme.useToken();

  useEffect(() => {
    getAllSubscriptions().then((res) => setSubscriptions(res));
  }, []);

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider
        collapsible
        collapsed={isMenuCollapsed}
        onCollapse={(value) => setMenuCollapsed(value)}
        width={300}>
        <div style={{ height: 32, margin: 16, background: 'rgba(255, 255, 255, 0.2)' }} />
        <Sidebar />
      </Sider>
      <Layout className="site-layout">
        <Header style={{ fontSize: 24, textAlign: 'center', background: colorBgContainer }}>
          Welcome to your subscription manager !
        </Header>
        <Content style={{ margin: '24px 16px' }}>
          <div style={{ padding: 24, minHeight: 360, background: colorBgContainer }}>
            {subscriptions.map((s) => {
              return <p key={s.name}>{s.name}</p>;
            })}
          </div>
        </Content>
        <Footer />
      </Layout>
    </Layout>
  );
}

export default App;
