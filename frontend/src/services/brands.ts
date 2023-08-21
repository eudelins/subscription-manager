import axios from 'axios';

import Brand from 'interfaces/brands/brand.interface';
import { BRANDS_API_PATH } from './utils/path';

export async function getAllBrands(): Promise<Brand[]> {
  const brands = await axios.get<Brand[]>(BRANDS_API_PATH);
  return brands.data;
}

export async function getBrandById(id: string): Promise<Brand> {
  const brand = await axios.get<Brand>(BRANDS_API_PATH + id);
  return brand.data;
}

export async function deleteBrandById(id: string): Promise<boolean> {
  const reponse = await axios.delete(BRANDS_API_PATH + id);
  return reponse.status === 200;
}

export async function createBrand(name: string): Promise<Brand> {
  const reponse = await axios.post(BRANDS_API_PATH, {
    id: -1,
    name
  });
  return reponse.data;
}

export async function updateBrand(id: number, name: string): Promise<Brand> {
  const reponse = await axios.put(BRANDS_API_PATH, {
    id,
    name
  });
  return reponse.data;
}
