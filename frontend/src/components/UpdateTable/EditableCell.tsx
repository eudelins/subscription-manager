import { useState } from 'react';
import { Form, Image, Input } from 'antd';

import Brand from 'interfaces/brands/brand.interface';
import Category from 'interfaces/categories/category.interface';
import { NEW_ELEM_ID, TableMode } from 'components/UpdateTable/UpdateTable';
import UploadButton from './UploadButton';

interface EditableCellProps extends React.HTMLAttributes<HTMLElement> {
  editing: boolean;
  dataIndex: string;
  title: any;
  inputType: 'text' | 'file';
  record: Category | Brand;
  index: number;
  children: React.ReactNode;
  openModal: (id: number) => void;
  mode: TableMode;
}

const EditableCell: React.FC<EditableCellProps> = ({
  editing,
  dataIndex,
  title,
  inputType,
  record,
  index,
  children,
  openModal,
  mode,
  ...restProps
}) => {
  const uploadDir = mode === TableMode.Brand ? 'brands/' : 'categories/';
  const uploadPath = import.meta.env.VITE_BASEURL + 'uploads/' + uploadDir;
  const [error, setError] = useState(false);

  const onError = () => {
    setError(true);
  };

  return (
    <td {...restProps}>
      {inputType === 'file' ? (
        error ? (
          <UploadButton uploadPath={uploadPath + record.id} disabled={record.id === NEW_ELEM_ID} />
        ) : (
          <Image
            src={uploadPath + record.id}
            preview={false}
            style={{ width: '48px', height: '48px', cursor: 'pointer' }}
            onClick={() => openModal(record.id)}
            onError={onError}
          />
        )
      ) : editing ? (
        <Form.Item
          name={dataIndex}
          style={{ margin: 0 }}
          rules={[
            {
              required: true,
              message: `Please Input ${title}!`
            }
          ]}>
          <Input />
        </Form.Item>
      ) : (
        children
      )}
    </td>
  );
};

export default EditableCell;
