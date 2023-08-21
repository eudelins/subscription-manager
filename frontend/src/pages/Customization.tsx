import { useEffect, useState } from 'react';

import { getAllBrands } from 'services/brands';
import { getAllCategories } from 'services/categories';
import Brand from 'interfaces/brands/brand.interface';
import Category from 'interfaces/categories/category.interface';
import UpdateTable, { TableMode } from 'components/UpdateTable/UpdateTable';
import { Space } from 'antd';

function Customization() {
  const [brands, setBrands] = useState<Brand[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);

  useEffect(() => {
    getAllBrands().then((res) => setBrands(res));
    getAllCategories().then((res) => setCategories(res));
  }, []);

  return (
    <Space direction="vertical" size="large" style={{ display: 'flex' }}>
      <UpdateTable
        elements={brands}
        mode={TableMode.Brand}
        setBrands={setBrands}
        setCategories={setCategories}
      />
      <UpdateTable
        elements={categories}
        mode={TableMode.Category}
        setBrands={setBrands}
        setCategories={setCategories}
      />
    </Space>
  );
}

export default Customization;
