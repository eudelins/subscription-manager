import axios from 'axios';

import Subscription from '../interfaces/subscriptions/subscription.interface';
import { SUBSCRIPTIONS_PATH } from './utils/path';

export async function getAllSubscriptions(): Promise<Subscription[]> {
  const subscriptions = await axios.get<Subscription[]>(SUBSCRIPTIONS_PATH);
  return subscriptions.data;
}
