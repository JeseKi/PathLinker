import { useEffect, useState } from 'react'; 
import { invoke } from '@tauri-apps/api/tauri';
import "./App.css";
import 'bootstrap/dist/css/bootstrap.min.css';

import FileSelector from "./compoents/file_selector"
import Mappings from "./compoents/table"

function App() {
  const [mappings, setMappings] = useState([]);

// 获取全部映射数据
  async function fetchData() {
    try {
      const data = await invoke('get_mappings');
      setMappings(data);
    } catch (error) {
      console.error('Failed to fetch mappings:', error);
    }
  }  

// 挂载时获取数据
  useEffect(() => {
    fetchData();
  }, [])

  return (
    <div className='content'>
      {/* title */}
      <h1 style={{fontSize: "8vw"}}>PathLinker</h1>
      <div className='mappings_container'>
        {/* add button */}
        <FileSelector />
        <Mappings data={mappings}/>
      </div>
    </div>
  );
}

export default App;
