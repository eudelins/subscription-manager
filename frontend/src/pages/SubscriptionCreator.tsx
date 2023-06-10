import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { SUBSCRIPTION_MANAGER_ROUTE } from 'routes/routes';

import { Button, Radio, Form, Input, Select, InputNumber } from 'antd';

import Brand from 'interfaces/brands/brand.interface';
import Category from 'interfaces/categories/category.interface';
import { getAllBrands } from 'services/brands';
import { getAllCategories } from 'services/categories';
import { createSubscription } from 'services/subscriptions';

function SubscriptionCreator() {
  const [brands, setBrands] = useState<Brand[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);

  useEffect(() => {
    getAllBrands().then((res) => setBrands(res));
    getAllCategories().then((res) => setCategories(res));
  }, []);

  const navigate = useNavigate();
  const onFinish = async (formValues: any) => {
    const success = await createSubscription(
      formValues.name,
      formValues.price,
      formValues.status,
      formValues.brandId,
      formValues.categoriesId
    );
    if (success) {
      navigate(SUBSCRIPTION_MANAGER_ROUTE);
    }
  };

  return (
    <Form
      name="basic"
      labelCol={{ span: 8 }}
      wrapperCol={{ span: 16 }}
      style={{ maxWidth: 1200 }}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
      initialValues={{ price: 9.99, status: false }}
      autoComplete="off">
      <Form.Item
        label="Nom de l'abonnement"
        name="name"
        rules={[{ required: true, message: 'Veuillez saisir un nom' }]}>
        <Input />
      </Form.Item>
      <Form.Item
        label="Prix de l'abonnement"
        name="price"
        rules={[{ required: true, message: 'Veuillez saisir un prix' }]}>
        <InputNumber addonAfter="€" min={0} />
      </Form.Item>
      <Form.Item label="Abonnement actif" name="status">
        <Radio.Group>
          <Radio value={true}>Actif</Radio>
          <Radio value={false}>Inactif</Radio>
        </Radio.Group>
      </Form.Item>
      <Form.Item label="Fournisseur" name="brandId">
        <Select
          showSearch
          placeholder="Sélectionnez la marque associée à l'abonnement"
          optionFilterProp="children"
          filterOption={(input, option) =>
            (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
          }
          options={brands.map((brand) => {
            return {
              value: brand.id,
              label: brand.name
            };
          })}
        />
      </Form.Item>
      <Form.Item label="Catégorie(s)" name="categoriesId">
        <Select
          mode="multiple"
          allowClear
          style={{ width: '100%' }}
          placeholder="Please select"
          options={categories.map((category) => {
            return {
              value: category.id,
              label: category.name
            };
          })}
        />
      </Form.Item>
      <Form.Item>
        <Button htmlType="submit">Submit</Button>
      </Form.Item>
    </Form>
  );
}

const onFinishFailed = (errorInfo: any) => {
  console.log('Failed:', errorInfo);
};

export default SubscriptionCreator;
