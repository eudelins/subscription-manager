import { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';

import { Row, Space } from 'antd';

import DeleteSubscriptionsButton from 'components/buttons/DeleteSubscriptionButton';
import EditSubscriptionButton from 'components/buttons/EditSubscriptionButton';
import SubscriptionForm from 'components/SubscriptionForm';

import Brand from 'interfaces/brands/brand.interface';
import Category from 'interfaces/categories/category.interface';
import Subscription from 'interfaces/subscriptions/subscription.interface';
import { getBrandById } from 'services/brands';
import { getAllCategories } from 'services/categories';
import { getSubscriptionById, updateSubscription } from 'services/subscriptions';

function SubscriptionPage() {
  const { id = '' } = useParams();
  const [subscription, setSubscription] = useState<Subscription>();
  const [brand, setBrand] = useState<Brand>();
  const [categories, setCategories] = useState<Category[]>([]);
  const [editMode, setEditMode] = useState<boolean>(false);

  const loadSubscription = () => {
    getSubscriptionById(id).then((res) => {
      setSubscription(res);
      if (res.brandId) {
        getBrandById(res.brandId.toString()).then((brand) => setBrand(brand));
      }
      getAllCategories().then((categories) => {
        setCategories(
          categories.filter((cat) => {
            return res.categoriesId?.find((catId) => catId === cat.id) !== undefined;
          })
        );
      });
    });
  };

  useEffect(() => {
    loadSubscription();
  }, []);

  const onEditFinish = async (formValues: any) => {
    const success = await updateSubscription(
      subscription ? subscription.id : -1,
      formValues.name,
      formValues.price,
      formValues.status,
      formValues.brandId,
      formValues.categoriesId
    );
    if (success) {
      loadSubscription();
      setEditMode(false);
    }
  };

  return (
    <>
      <Row justify="end">
        <Space size="middle" style={{ marginBottom: 32 }}>
          {subscription && editMode && <DeleteSubscriptionsButton subscription={subscription} />}
          <EditSubscriptionButton editMode={editMode} setEditMode={setEditMode} />
        </Space>
      </Row>

      {subscription && (
        <SubscriptionForm
          subscription={subscription}
          onFinish={onEditFinish}
          isDisabled={!editMode}
        />
      )}
    </>
  );
}

export default SubscriptionPage;
