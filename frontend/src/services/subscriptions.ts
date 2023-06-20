import axios from 'axios';

import Subscription from 'interfaces/subscriptions/subscription.interface';
import { SUBSCRIPTIONS_API_PATH } from './utils/path';

export async function getAllSubscriptions(): Promise<Subscription[]> {
  const subscriptions = await axios.get<Subscription[]>(SUBSCRIPTIONS_API_PATH);
  return subscriptions.data;
}

export async function getSubscriptionById(id: string): Promise<Subscription> {
  const subscription = await axios.get(SUBSCRIPTIONS_API_PATH + id);
  return {
    id: subscription.data.id,
    brandId: subscription.data.brand_id,
    name: subscription.data.name,
    price: subscription.data.price,
    status: subscription.data.status,
    categoriesId: subscription.data.categories_id
  };
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

export async function updateSubscription(
  id: number,
  name: string,
  price: number,
  status: boolean,
  brandId: number,
  categoriesId: number[]
): Promise<boolean> {
  const reponse = await axios.put(SUBSCRIPTIONS_API_PATH, {
    id,
    name,
    price,
    status,
    brand_id: brandId,
    categories_id: categoriesId
  });
  return reponse.status === 200;
}

export async function archiveSubscriptions(subIds: Subscription[]): Promise<boolean> {
  const reponse = await axios.post(
    SUBSCRIPTIONS_API_PATH + 'archive',
    subIds.map((s) => s.id)
  );
  return reponse.status === 200;
}

export async function activateSubscriptions(subIds: Subscription[]): Promise<boolean> {
  const reponse = await axios.post(
    SUBSCRIPTIONS_API_PATH + 'activate',
    subIds.map((s) => s.id)
  );
  return reponse.status === 200;
}

export async function deleteSubscription(subscription: Subscription): Promise<boolean> {
  const reponse = await axios.delete(SUBSCRIPTIONS_API_PATH + subscription.id.toString());
  return reponse.status === 200;
}
