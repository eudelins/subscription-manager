import { useState, useEffect, CSSProperties } from 'react';

import { Button, Radio, Form, Input, Select, InputNumber } from 'antd';

import Brand from 'interfaces/brands/brand.interface';
import Category from 'interfaces/categories/category.interface';
import Subscription from 'interfaces/subscriptions/subscription.interface';
import { getAllBrands } from 'services/brands';
import { getAllCategories } from 'services/categories';

interface Props {
  subscription?: Subscription;
  onFinish: (formValues: any) => void;
  isDisabled: boolean;
  style?: CSSProperties;
}

function SubscriptionForm({ subscription, onFinish, isDisabled, style }: Props) {
  const [brands, setBrands] = useState<Brand[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);

  useEffect(() => {
    getAllBrands().then((res) => setBrands(res));
    getAllCategories().then((res) => setCategories(res));
  }, []);

  return (
    <Form
      name="basic"
      disabled={isDisabled}
      labelCol={{ span: 8 }}
      wrapperCol={{ span: 16 }}
      style={{ maxWidth: 1200, ...style }}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
      initialValues={{
        name: subscription ? subscription.name : '',
        price: subscription ? subscription.price : 9.99,
        status: subscription ? subscription.status : false,
        brandId: subscription?.brandId,
        categoriesId: subscription ? subscription.categoriesId : []
      }}
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
      {!subscription && (
        <Form.Item wrapperCol={{ offset: 13, span: 16 }}>
          <Button shape='round' style={buttonStyle} htmlType="submit">Valider</Button>
        </Form.Item>
      )}
    </Form>
  );
}

const onFinishFailed = (errorInfo: any) => {
  console.log('Failed:', errorInfo);
};

const buttonStyle: CSSProperties = {
  backgroundColor: '#1DB954',
  color: 'white',
  fontSize: 17,
  fontWeight: 'bold',
  height: 50,
  width: 120
};

export default SubscriptionForm;
