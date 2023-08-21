import { Button } from 'antd';
import { PlusCircleOutlined } from '@ant-design/icons';

import { TableMode, NEW_ELEM_ID } from 'components/UpdateTable/UpdateTable';
import Brand from 'interfaces/brands/brand.interface';
import Category from 'interfaces/categories/category.interface';

interface Props {
  mode: TableMode;
  elements: Category[] | Brand[];
  setElements: (elements: Category[] | Brand[]) => void;
  editingId: number;
  setEditingId: React.Dispatch<React.SetStateAction<number>>;
}

const AddElementButton = ({ mode, elements, setElements, editingId, setEditingId }: Props) => {
  const handleAdd = () => {
    if (mode === TableMode.Brand) {
      setElements([
        ...(elements as Brand[]),
        {
          id: NEW_ELEM_ID,
          name: '',
          logo: null
        } as Brand
      ]);
    } else {
      setElements([
        ...(elements as Category[]),
        {
          id: NEW_ELEM_ID,
          name: '',
          icon: null
        } as Category
      ]);
    }
    setEditingId(NEW_ELEM_ID);
  };

  return (
    <Button
      disabled={editingId !== -1}
      onClick={handleAdd}
      type="primary"
      style={{ marginBottom: 16 }}>
      <PlusCircleOutlined />
      {mode === TableMode.Brand ? 'Ajouter un fournisseur' : 'Ajouter une catégorie'}
    </Button>
  );
};

export default AddElementButton;
