interface Subscription {
  id: number;
  brandId?: number;
  name: string;
  price: number;
  status: boolean;
  categoriesId?: number[];
}

export default Subscription;
