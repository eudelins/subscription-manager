import { ReactNode, Key } from 'react';
import { Link } from 'react-router-dom';
import {
  CUSTOMIZATION_ROUTE,
  DATA_VISUALIZATION_ROUTE,
  SUBSCRIPTION_MANAGER_ROUTE
} from '../routes/routes';

import { ContainerOutlined, DesktopOutlined, PieChartOutlined } from '@ant-design/icons';
import { Menu } from 'antd';
import type { MenuProps } from 'antd';

type MenuItem = Required<MenuProps>['items'][number];

function getItem(
  label: ReactNode,
  key: Key,
  icon?: ReactNode,
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
  getItem(
    <Link to={SUBSCRIPTION_MANAGER_ROUTE}>Gestion des abonnements</Link>,
    '1',
    <PieChartOutlined />
  ),
  getItem(<Link to={DATA_VISUALIZATION_ROUTE}>Visualisation</Link>, '2', <DesktopOutlined />),
  getItem(<Link to={CUSTOMIZATION_ROUTE}>Personnalisation</Link>, '3', <ContainerOutlined />)
];

function Sidebar() {
  return <Menu defaultSelectedKeys={['1']} mode="inline" theme="dark" items={items} />;
}

export default Sidebar;
