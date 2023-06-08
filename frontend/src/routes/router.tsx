import App from '../App';
import SubscriptionManager from '../pages/SubscriptionManager';
import DataVisualization from '../pages/DataVisualization';
import Customization from '../pages/Customization';
import SubscriptionCreator from '../pages/SubscriptionCreator';
import SubscriptionPage from '../pages/SubscriptionPage';

import { createBrowserRouter } from 'react-router-dom';
import {
  CUSTOMIZATION_ROUTE,
  DATA_VISUALIZATION_ROUTE,
  ROOT_ROUTE,
  SUBSCRIPTION_CREATOR_ROUTE,
  SUBSCRIPTION_MANAGER_ROUTE,
  SUBSCRIPTION_PAGE_ROUTE
} from './routes';

const router = createBrowserRouter([
  {
    path: ROOT_ROUTE,
    element: <App />,
    children: [
      {
        path: ROOT_ROUTE,
        element: <SubscriptionManager />
      },
      {
        path: SUBSCRIPTION_MANAGER_ROUTE,
        element: <SubscriptionManager />
      },
      {
        path: SUBSCRIPTION_CREATOR_ROUTE,
        element: <SubscriptionCreator />
      },
      {
        path: SUBSCRIPTION_PAGE_ROUTE,
        element: <SubscriptionPage />
      },
      {
        path: DATA_VISUALIZATION_ROUTE,
        element: <DataVisualization />
      },
      {
        path: CUSTOMIZATION_ROUTE,
        element: <Customization />
      }
    ]
  }
]);

export default router;
