import axios from 'axios';

import Brand from '../interfaces/brands/brand.interface';
import { BRANDS_API_PATH } from './utils/path';

export async function getAllBrands(): Promise<Brand[]> {
  const subscriptions = await axios.get<Brand[]>(BRANDS_API_PATH);
  return subscriptions.data;
}
