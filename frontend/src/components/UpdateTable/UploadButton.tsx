import { Button, Upload, UploadProps, message } from 'antd';
import { UploadOutlined } from '@ant-design/icons';

const ACCEPTED_MIME_TYPE = ['image/png', 'image/x-icon', 'image/jpeg', 'image/svg+xml'];

interface Props {
  uploadPath: string;
  disabled: boolean;
}

const UploadButton = ({ uploadPath, disabled }: Props) => {
  const uploadProps: UploadProps = {
    beforeUpload: (file) => {
      const isImage = ACCEPTED_MIME_TYPE.includes(file.type);
      if (!isImage) {
        message.error(`${file.name} is not an image file`);
      }
      return isImage || Upload.LIST_IGNORE;
    },
    name: 'file',
    action: uploadPath,
    onChange(info) {
      if (info.file.status !== 'uploading') {
        console.log(info.file, info.fileList);
      }
      if (info.file.status === 'done') {
        message.success(`${info.file.name} file uploaded successfully`);
      } else if (info.file.status === 'error') {
        message.error(`${info.file.name} file upload failed.`);
      }
    },
    showUploadList: false
  };

  return (
    <Upload {...uploadProps} disabled={disabled}>
      <Button icon={<UploadOutlined />} disabled={disabled}>
        Upload
      </Button>
    </Upload>
  );
};

export default UploadButton;
