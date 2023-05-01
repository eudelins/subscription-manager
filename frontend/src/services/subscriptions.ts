import axios from 'axios';
import Subscriptions from '../interfaces/subscriptions/subscription.interface';

export async function getAllSubscriptions(): Promise<Subscriptions[]> {
  const subscriptions = await axios.get(import.meta.env.VITE_BASEURL + 'subscriptions/');
  return subscriptions.data;
}
