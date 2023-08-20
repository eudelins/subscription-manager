import axios from 'axios';

import { UPLOADS_API_PATH } from 'services/utils/path';

export async function deleteFile(path: string): Promise<boolean> {
  const reponse = await axios.delete(UPLOADS_API_PATH + path);
  return reponse.status === 200;
}
