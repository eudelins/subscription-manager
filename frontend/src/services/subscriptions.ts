import axios from 'axios';

import Subscriptions from '../interfaces/subscriptions/subscription.interface';
import { SUBSCRIPTIONS_PATH } from './utils/path'

export async function getAllSubscriptions(): Promise<Subscriptions[]> {
  const subscriptions = await axios.get(SUBSCRIPTIONS_PATH);
  return subscriptions.data;
}
