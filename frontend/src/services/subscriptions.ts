import axios from 'axios';

import Subscription from '../interfaces/subscriptions/subscription.interface';
import { SUBSCRIPTIONS_API_PATH } from './utils/path';

export async function getAllSubscriptions(): Promise<Subscription[]> {
  const subscriptions = await axios.get<Subscription[]>(SUBSCRIPTIONS_API_PATH);
  return subscriptions.data;
}

export async function createSubscription(
  name: string,
  price: number,
  status: boolean,
  brandId: number,
  categoriesId: number[]
): Promise<boolean> {
  const reponse = await axios.post(SUBSCRIPTIONS_API_PATH, {
    name,
    price,
    status,
    brand_id: brandId,
    categories_id: categoriesId
  });
  return reponse.status === 200;
}
