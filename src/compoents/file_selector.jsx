import React, { useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import { Button }from "react-bootstrap"

function FileSelector({t}) {
  const [selectedFile, setSelectedFile] = useState(null);

  // Choose multi files
  const openFileDialog = async () => {
      console.log("FileSelector clicked!")
      const result = await open({
          multiple: true,
          title: "选择一个或多个文件"
      })

      setSelectedFile(result)

      if (result) {
        invoke('handle_selected_path', { selected: Array.isArray(result) ? result : [result] })
        location.reload();
    }
  }

  return (
    <div style={{marginRight:"-90%"}}>
        <Button onClick={openFileDialog}>{t('index.select_files')}</Button>
        {selectedFile && (
        <p style={{display:"none"}}>Selected file: {Array.isArray(selectedFile) ? selectedFile.join(', ') : selectedFile}</p>
        )}
    </div>
  );
}

export default FileSelector;