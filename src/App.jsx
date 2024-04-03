import { useEffect, useState } from 'react'; 
import { invoke } from '@tauri-apps/api/tauri';
import "./App.css";
import 'bootstrap/dist/css/bootstrap.min.css';
import { useTranslation, Trans } from 'react-i18next';

import FileSelector from "./compoents/file_selector"
import Mappings from "./compoents/table"
import Settings from './compoents/settings';

function App() {
  const [mappings, setMappings] = useState([]);
  const { t, i18n } = useTranslation();

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
      <Settings t={t} i18n={i18n}/>
      {/* title */}
      <h1 style={{fontSize: "8vw"}}>PathLinker</h1>
      <div className='mappings_container'>
        {/* add mapping button */}
        <FileSelector t={t} />
        <Mappings data={mappings} t={t}/>
      </div>
    </div>
  );
}

export default App;
