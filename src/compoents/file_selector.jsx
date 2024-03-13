import React, { useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';

function Selector() {
  const [selectedPath, setSelectedPath] = useState(null);
  const [selectedFile, setSelectedFile] = useState(null);

  // Choose multi paths
  const openDialog = async () => {
    console.log("Selector clicked!")
    const result = await open({
      directory: true,
      multiple: true,
      title: "选择一个或多个文件夹"
    });

    setSelectedPath(result);

    console.log(result);

    if (result) {
        invoke('handle_selected_path', { selected: Array.isArray(result) ? result : [result] })
    }
  };

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
    }
  }

  return (
    <div>
        <p>Choose a path</p>
        <button onClick={openDialog}>Open Dialog</button>
        {selectedPath && (
        <p>Selected path: {Array.isArray(selectedPath) ? selectedPath.join(', ') : selectedPath}</p>
        )}
        <p>Choose a file</p>
        <button onClick={openFileDialog}>Open File</button>
        {selectedFile && (
        <p>Selected file: {Array.isArray(selectedFile) ? selectedFile.join(', ') : selectedFile}</p>
        )}
    </div>
  );
}

export default Selector;