import Selector from "./compoents/file_selector";

import { Container, Table , Button , Modal, Form} from 'react-bootstrap';
import { useEffect, useState } from 'react'; 
import "./App.css";
import 'bootstrap/dist/css/bootstrap.min.css';

function App() {
  const [mappings, setMappings] = useState([]);
  const [path , setPath] = useState("")
  const [deleteURL , setDeleteURL] = useState("")
  const [addShow, setAddShow] = useState(false);
  const [deleteShow, setDeleteShow] = useState(false)

  const handleAddClose = () => setAddShow(false);
  const handleAddShow = () => setAddShow(true);

  const handeDeleteClose = () => setDeleteShow(false);
  const handleDeleteShow = () => setDeleteShow(true);

  // 异步获取数据的函数
  async function fetchData() {
    const response = await fetch('http://localhost:8000/mappings');
    if (response.ok) {
      const data = await response.json();
      setMappings(data); // 设置状态
    } else {
      // 错误处理
      console.error('Failed to fetch mappings:', response.statusText);
    }
  }

  // 复制链接
  const copyToClipboard = (url) => {
    navigator.clipboard.writeText(url).then(() => {
      alert('URL copied to clipboard');
    }).catch(err => {
      console.error('Could not copy text: ', err);
    });
  };

  // 从数据库中删除条目的函数，并刷新页面
  const deleteMapping = async (originalUrl) => {
    // 对URL进行二重编码
    const encodedUrl_1 = encodeURIComponent(originalUrl);
    const encodedUrl_2 = encodeURIComponent(encodedUrl_1)

    const response = await fetch(`http://localhost:8000/mappings/${encodedUrl_2}`, {
      method: 'DELETE',
    });

    if (response.ok) {
      window.location.reload();
    } else {
      // 错误处理
      console.error('Failed to delete mapping:', response.statusText);
    }
  };

  //添加映射
  const createNewMapping = async (path) => {
    if (!path) {
      console.error('Path is required');
      return;
    }
  
    // 使用encodeURIComponent确保路径中的特殊字符被正确编码
    const encodedPath = encodeURIComponent(path);
  
    const response = await fetch(`http://localhost:8000/mappings?path=${encodedPath}`, {
      method: 'POST',
      headers: {
        'accept': 'application/json',
      },
    });
  
    if (response.ok) {
      const data = await response.json();
      alert("Add success")
      window.location.reload();
      return data;
    } else {
      console.error('Failed to create new mapping:', response.statusText);
      return null;
    }
  };  

// 删除映射
const handeDeleteMapping = (url) => {
    // 设置要删除的URL
    setDeleteURL(url);
    // 确定删除页面
    handleDeleteShow();
}

// 组件挂载时获取数据
  useEffect(() => {
    fetchData();
  }, [])

  return (
    <div className='content'>
      {/* title */}
      <h1 style={{fontSize: "8vw"}}>PathLinker</h1>
      <div className='mappings_container'>
        {/* add button */}
        <Button style={{marginRight: "-96%"}} onClick={handleAddShow}>Add</Button>
        {/* mappings */}
        <Table className='mappings_table'>
          <thead>
            <tr>
              <th>#</th>
              <th>Name</th>
              <th>Path</th>
              <th>URL</th>
            </tr>
          </thead>
          <tbody>
            {mappings.map((mapping, index) => (
              <tr key={index}>
                <td>{index + 1}</td>
                <td>{mapping.file_name}</td>
                <td>{mapping.path}</td>
                <td><a href={mapping.url}>{mapping.url}</a></td>
                <td>
                  {/* copy button */}
                  <Button variant="primary" onClick={() => copyToClipboard(mapping.url)}>
                    copy
                  </Button>
                </td>
                <td>
                  {/* delete button */}
                  <Button variant="danger" onClick={() => handeDeleteMapping(mapping.url)}>
                    delete
                  </Button>
              </td>
              </tr>
            ))}
          </tbody>
        </Table>
      </div>
      {/* add mapping modal */}
      <Modal 
        show={addShow} 
        onHide={handleAddClose}
        backdrop="static"
        keyboard={false}
        centered
        >
        <Modal.Header closeButton>
          <Modal.Title>Add a new mapping</Modal.Title>
        </Modal.Header>
        <Modal.Body>
        <Form.Control 
          id='path'
          type='text'
          placeholder='Type your file path here.'
          value={path}
          onChange={(e) => setPath(e.target.value)}
        />
          <p>eg:</p>
          Windows: C:\Users\YourName\Desktop\FileName.txt <br></br>
          Linux: /home/YourName/Desktop/FileName.txt <br></br>
          Mac: /Users/YourName/Desktop/FileName.txt<br></br>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={handleAddClose}>
            Cancel
          </Button>
          <Button variant="primary" onClick={() => createNewMapping(path)}>
            Commit
          </Button>
        </Modal.Footer>
      </Modal>
      {/* confirm delete mapping */}
      <Modal 
        show={deleteShow} 
        onHide={handeDeleteClose}
        backdrop="static"
        keyboard={false}
        centered
        >
        <Modal.Header closeButton>
          <Modal.Title>Confirm deletion of mapping?</Modal.Title>
        </Modal.Header>
        <Modal.Footer>
          <Button variant="secondary" onClick={handeDeleteClose}>
            Cancel
          </Button>
          <Button variant="danger" onClick={() => deleteMapping(deleteURL)}>
            Confirm
          </Button>
        </Modal.Footer>
      </Modal>
    </div>
  );
}

export default App;
