import { CSSProperties } from 'react';

import { Divider } from 'antd';

interface Props {
  title: string;
  style?: CSSProperties;
}

function DividerWithTitle({ title, style }: Props) {
  return (
    <div style={{ display: 'flex', alignItems: 'center', ...style }}>
      <div style={{ flexGrow: 1 }}>
        <span style={{ marginLeft: '4px', fontSize: 18 }}>{title}</span>
        <Divider
          style={{ width: '40%', minWidth: '40%', border: '2px solid #f0f0f0', marginTop: '8px' }}
        />
      </div>
    </div>
  );
}

export default DividerWithTitle;
