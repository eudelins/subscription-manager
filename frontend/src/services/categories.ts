import axios from 'axios';

import Category from 'interfaces/categories/category.interface';
import { CATEGORIES_API_PATH } from './utils/path';

export async function getAllCategories(): Promise<Category[]> {
  const categories = await axios.get<Category[]>(CATEGORIES_API_PATH);
  return categories.data;
}

export async function getCategoryById(id: string): Promise<Category> {
  const category = await axios.get<Category>(CATEGORIES_API_PATH + id);
  return category.data;
}

export async function deleteCategoryById(id: string): Promise<boolean> {
  const reponse = await axios.delete(CATEGORIES_API_PATH + id);
  return reponse.status === 200;
}

export async function createCategory(name: string): Promise<boolean> {
  const reponse = await axios.post(CATEGORIES_API_PATH, {
    id: -1,
    name
  });
  return reponse.status === 200;
}

export async function updateCategory(id: number, name: string): Promise<boolean> {
  const reponse = await axios.put(CATEGORIES_API_PATH, {
    id,
    name
  });
  return reponse.status === 200;
}
