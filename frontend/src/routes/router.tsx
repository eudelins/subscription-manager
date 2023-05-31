import App from '../App';

import { createBrowserRouter } from 'react-router-dom';
import SubscriptionManager from '../pages/SubscriptionManager';
import DataVisualization from '../pages/DataVisualization';
import Customization from '../pages/Customization';

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    children: [
      {
        path: '/',
        element: <SubscriptionManager />
      },
      {
        path: 'subscriptions/',
        element: <SubscriptionManager />
      },
      {
        path: 'data/',
        element: <DataVisualization />
      },
      {
        path: 'customization/',
        element: <Customization />
      }
    ]
  }
]);

export default router;
