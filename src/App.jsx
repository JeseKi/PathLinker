import { useEffect, useState } from 'react'; 
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import "./App.css";
import 'bootstrap/dist/css/bootstrap.min.css';
import { useTranslation } from 'react-i18next';

import {Mappings} from "./compoents/table"
import Settings from './compoents/settings';

function App() {
  const [mappings, setMappings] = useState([]);
  const { t, i18n } = useTranslation();

  // 测试
  async function bo_test() {
    try {
      const data = await invoke('folders_struct');
      console.log(data);
    } catch (error) {
      console.error('Failed to fetch folders:', error);
    }
  }

// 获取全部映射数据
  async function fetchMappingsData() {
    try {
      const data = await invoke('get_mappings');
      setMappings(data);
    } catch (error) {
      console.error('Failed to fetch mappings:', error);
    }
  }  

// 挂载时获取数据
  useEffect(() => {

    fetchMappingsData();
    bo_test();

    // 设置监听后端发来的消息
    const unlisten = listen('error-message', (event) => {
      alert(`Received error message: ${event.payload}`);
    });

    return () => {
      unlisten.then((fn) => fn()); // 清理监听器
    };
    
  }, [])

  return (
    <div className='content'>
      <Settings t={t} i18n={i18n}/>
      {/* title */}
      <h1 style={{fontSize: "8vw" , userSelect:"none"}}>PathLinker</h1>
      <div className='mappings_container'>
        <Mappings data={mappings} t={t}/>
      </div>
    </div>
  );
}

export default App;
