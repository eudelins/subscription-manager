import { useState } from 'react';

import { Layout, theme } from 'antd';
import Sidebar from './components/Sidebar';

import { Outlet } from 'react-router-dom';

const { Header, Content, Footer, Sider } = Layout;

function App() {
  const [isMenuCollapsed, setMenuCollapsed] = useState(false);

  const {
    token: { colorBgContainer }
  } = theme.useToken();

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
          Bienvenue sur votre gestionnaire d&apos;abonnements !
        </Header>
        <Content style={{ margin: '24px 16px' }}>
          <div style={{ padding: 24, minHeight: 360, background: colorBgContainer }}>
            <Outlet />
          </div>
        </Content>
        <Footer />
      </Layout>
    </Layout>
  );
}

export default App;
