import { useState } from 'react';

import { Form, Popconfirm, Space, Table, Typography } from 'antd';
import EditableCell from 'components/UpdateTable/EditableCell';

import Brand from 'interfaces/brands/brand.interface';
import Category from 'interfaces/categories/category.interface';
import { createBrand, deleteBrandById, updateBrand } from 'services/brands';
import { createCategory, deleteCategoryById, updateCategory } from 'services/categories';
import ImageModal from 'components/UpdateTable/ImageModal';
import AddElementButton from 'components/UpdateTable/AddElementButton';

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

export const NEW_ELEM_ID = -2;

function UpdateTable({ elements, mode, setBrands, setCategories }: Props) {
  const [form] = Form.useForm();
  const [editingId, setEditingId] = useState(-1);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [openModalElemId, setOpenModalElemId] = useState(-1);
  const isEditing = (record: Category | Brand) => record.id === editingId;

  const openModal = (id: number) => {
    setIsModalOpen(true);
    setOpenModalElemId(id);
  };

  const setElements = (elements: (Category | Brand)[]) => {
    mode === TableMode.Category
      ? setCategories(elements as Category[])
      : setBrands(elements as Brand[]);
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
      let row = (await form.validateFields()) as Category | Brand;

      const newElements = [...elements];
      const index = newElements.findIndex((item) => id === item.id);
      if (index > -1) {
        row =
          id === NEW_ELEM_ID
            ? mode === TableMode.Brand
              ? await createBrand(row.name)
              : await createCategory(row.name)
            : mode === TableMode.Brand
            ? await updateBrand(id, row.name)
            : await updateCategory(id, row.name);

        const item = newElements[index];
        newElements.splice(index, 1, {
          ...item,
          ...row
        });
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
      mode === TableMode.Brand
        ? deleteBrandById(elements[index].id.toString())
        : deleteCategoryById(elements[index].id.toString());
      const newElements = [...elements].splice(index, 1);
      setElements(newElements);
    }
  };

  const columns = [
    {
      title: 'Nom ' + (mode === TableMode.Brand ? 'du fournisseur' : 'de la catégorie'),
      dataIndex: 'name',
      width: '60%',
      editable: true
    },
    {
      title: mode === TableMode.Brand ? 'Logo' : 'Icône',
      dataIndex: 'file',
      width: '25%',
      editable: true
    },
    {
      title: 'Operation',
      dataIndex: 'operation',
      render: (_: any, record: Category | Brand) => {
        return isEditing(record) ? (
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
    if (!col.editable) return col;
    return {
      ...col,
      onCell: (record: Category | Brand) => ({
        record,
        inputType: col.dataIndex === 'file' ? 'file' : 'text',
        dataIndex: col.dataIndex,
        title: col.title,
        editing: isEditing(record),
        openModal: openModal,
        mode
      })
    };
  });

  return (
    <>
      <ImageModal
        isModalOpen={isModalOpen}
        elemId={openModalElemId}
        setIsModalOpen={setIsModalOpen}
        mode={mode}
      />
      <AddElementButton
        mode={mode}
        editingId={editingId}
        setEditingId={setEditingId}
        elements={elements}
        setElements={setElements}
      />
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
