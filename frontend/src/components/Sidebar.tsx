import { ContainerOutlined, DesktopOutlined, PieChartOutlined } from '@ant-design/icons';
import { Menu } from 'antd';
import type { MenuProps } from 'antd';

type MenuItem = Required<MenuProps>['items'][number];

function getItem(
  label: React.ReactNode,
  key: React.Key,
  icon?: React.ReactNode,
  children?: MenuItem[],
  type?: 'group'
): MenuItem {
  return {
    key,
    icon,
    children,
    label,
    type
  } as MenuItem;
}

const items: MenuItem[] = [
  getItem('Gestion des abonnements', '1', <PieChartOutlined />),
  getItem('Visualisation', '2', <DesktopOutlined />),
  getItem('Personnalisation', '3', <ContainerOutlined />)
];

function Sidebar() {
  return <Menu defaultSelectedKeys={['1']} mode="inline" theme="dark" items={items} />;
}

export default Sidebar;
