import axios from 'axios';

import Category from 'interfaces/categories/category.interface';
import { CATEGORIES_API_PATH } from './utils/path';

export async function getAllCategories(): Promise<Category[]> {
  const categories = await axios.get<Category[]>(CATEGORIES_API_PATH);
  return categories.data;
}
