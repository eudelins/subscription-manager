import { ContainerOutlined, DesktopOutlined, PieChartOutlined } from '@ant-design/icons';
import { Menu } from 'antd';
import type { MenuProps } from 'antd';
import { Link } from 'react-router-dom';

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
  getItem(<Link to={`subscriptions/`}>Gestion des abonnements</Link>, '1', <PieChartOutlined />),
  getItem(<Link to={`data/`}>Visualisation</Link>, '2', <DesktopOutlined />),
  getItem(<Link to={`customization/`}>Personnalisation</Link>, '3', <ContainerOutlined />)
];

function Sidebar() {
  return <Menu defaultSelectedKeys={['1']} mode="inline" theme="dark" items={items} />;
}

export default Sidebar;
