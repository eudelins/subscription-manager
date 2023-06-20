import { useState } from 'react';

import { Button, Form, Input, Popconfirm, Space, Table, Typography } from 'antd';

import Brand from 'interfaces/brands/brand.interface';
import Category from 'interfaces/categories/category.interface';
import { createBrand, deleteBrandById, updateBrand } from 'services/brands';
import { createCategory, deleteCategoryById, updateCategory } from 'services/categories';

interface EditableCellProps extends React.HTMLAttributes<HTMLElement> {
  editing: boolean;
  dataIndex: string;
  title: any;
  inputType: 'text';
  record: Category | Brand;
  index: number;
  children: React.ReactNode;
}

const EditableCell: React.FC<EditableCellProps> = ({
  editing,
  dataIndex,
  title,
  inputType,
  record,
  index,
  children,
  ...restProps
}) => {
  return (
    <td {...restProps}>
      {editing ? (
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

export enum TableMode {
  Brand,
  Category
}

interface Props {
  elements: Category[] | Brand[];
  mode: TableMode;
  setBrands: React.Dispatch<React.SetStateAction<Brand[]>>;
  setCategories: React.Dispatch<React.SetStateAction<Category[]>>;
}

const NEW_ELEM_ID = -2;

function UpdateTable({ elements, mode, setBrands, setCategories }: Props) {
  const [form] = Form.useForm();
  const [editingId, setEditingId] = useState(-1);
  const isEditing = (record: Category | Brand) => record.id === editingId;

  const setElements = (elements: Category[] | Brand[]) => {
    if (mode === TableMode.Category) {
      setCategories(elements as Category[]);
    } else {
      setBrands(elements as Brand[]);
    }
  };

  const edit = (record: Partial<Category | Brand> & { id: number }) => {
    form.setFieldsValue({ name: '', ...record });
    setEditingId(record.id ? record.id : -1);
  };

  const cancel = () => {
    if (editingId === NEW_ELEM_ID) {
      const newElements = [...elements];
      newElements.pop();
      setElements(newElements);
    }
    setEditingId(-1);
  };

  const save = async (id: number) => {
    try {
      const row = (await form.validateFields()) as Category | Brand;

      const newElements = [...elements];
      const index = newElements.findIndex((item) => id === item.id);
      if (index > -1) {
        const item = newElements[index];
        newElements.splice(index, 1, {
          ...item,
          ...row
        });
        if (id === NEW_ELEM_ID) {
          if (mode === TableMode.Brand) {
            createBrand(row.name);
          } else {
            createCategory(row.name);
          }
        } else {
          if (mode === TableMode.Brand) {
            updateBrand(id, row.name);
          } else {
            updateCategory(id, row.name);
          }
        }
        setElements(newElements);
        setEditingId(-1);
      } else {
        console.error('Element id not found:', id);
      }
    } catch (errInfo) {
      console.error('Validate Failed:', errInfo);
    }
  };

  const deleteElement = async (id: number) => {
    const index = elements.findIndex((item) => id === item.id);
    if (index > -1) {
      if (mode === TableMode.Brand) {
        deleteBrandById(elements[index].id.toString());
      } else {
        deleteCategoryById(elements[index].id.toString());
      }
      const newElements = [...elements];
      newElements.splice(index, 1);
      setElements(newElements);
    }
  };

  const columns = [
    {
      title: 'Nom ' + (mode === TableMode.Brand ? 'du fournisseur' : 'de la catégorie'),
      dataIndex: 'name',
      width: '85%',
      editable: true
    },
    {
      title: 'Operation',
      dataIndex: 'operation',
      render: (_: any, record: Category | Brand) => {
        const editable = isEditing(record);
        return editable ? (
          <span>
            <Typography.Link onClick={() => save(record.id)} style={{ marginRight: 8 }}>
              Enregistrer
            </Typography.Link>
            <Popconfirm title="Confirmer l'annulation ?" onConfirm={cancel}>
              <a>Annuler</a>
            </Popconfirm>
          </span>
        ) : (
          <Space>
            <Typography.Link disabled={editingId !== -1} onClick={() => edit(record)}>
              Modifier
            </Typography.Link>
            <Popconfirm
              title="Confirmer la suppression ?"
              onConfirm={() => deleteElement(record.id)}>
              <Typography.Link disabled={editingId !== -1}>Supprimer</Typography.Link>
            </Popconfirm>
          </Space>
        );
      }
    }
  ];

  const mergedColumns = columns.map((col) => {
    if (!col.editable) {
      return col;
    }
    return {
      ...col,
      onCell: (record: Category | Brand) => ({
        record,
        inputType: 'text',
        dataIndex: col.dataIndex,
        title: col.title,
        editing: isEditing(record)
      })
    };
  });

  const handleAdd = () => {
    const newData =
      mode === TableMode.Brand
        ? ({
            id: NEW_ELEM_ID,
            name: '',
            logo: undefined
          } as Brand)
        : ({
            id: NEW_ELEM_ID,
            name: '',
            icon: undefined
          } as Category);
    setElements([...elements, newData]);
    setEditingId(NEW_ELEM_ID);
  };

  return (
    <>
      <Button
        disabled={editingId !== -1}
        onClick={handleAdd}
        type="primary"
        style={{ marginBottom: 16 }}>
        Add a row
      </Button>
      <Form form={form} component={false}>
        <Table
          components={{
            body: {
              cell: EditableCell
            }
          }}
          bordered
          dataSource={elements}
          columns={mergedColumns}
          rowClassName="editable-row"
          pagination={{
            onChange: cancel,
            hideOnSinglePage: true
          }}
          rowKey={(record) => record.id}
        />
      </Form>
    </>
  );
}

export default UpdateTable;
