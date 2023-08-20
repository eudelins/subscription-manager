import { Button, Image, Modal, Space, message } from 'antd';

import { TableMode } from 'components/UpdateTable/UpdateTable';
import UploadButton from 'components/UpdateTable/UploadButton';
import { deleteFile } from 'services/uploads';

interface Props {
  isModalOpen: boolean;
  elemId: number;
  setIsModalOpen: React.Dispatch<React.SetStateAction<boolean>>;
  mode: TableMode;
}

const ImageModal = ({ isModalOpen, elemId, setIsModalOpen, mode }: Props) => {
  const uploadDir = mode === TableMode.Brand ? 'brands/' : 'categories/';
  const uploadPath = import.meta.env.VITE_BASEURL + 'uploads/' + uploadDir;

  const deleteElemFile = async () => {
    let success = await deleteFile(uploadDir + elemId);
    if (success) {
      message.success('file deleted successfully');
    } else {
      message.error('file delete failed.');
    }
    setIsModalOpen(false);
  };

  return (
    <Modal
      open={isModalOpen}
      onOk={() => setIsModalOpen(false)}
      onCancel={() => setIsModalOpen(false)}>
      <Space style={{ marginBottom: 20 }}>
        <Image src={uploadPath + elemId} preview={false} />
        <Space direction="vertical">
          <UploadButton uploadPath={uploadPath + elemId} disabled={false} />
          <Button onClick={deleteElemFile}>Delete</Button>
        </Space>
      </Space>
    </Modal>
  );
};

export default ImageModal;
