import { Table , Button , Modal} from 'react-bootstrap';
import { useEffect, useState } from 'react'; 
import { invoke } from '@tauri-apps/api/tauri';
import { clipboard } from '@tauri-apps/api';
import 'bootstrap/dist/css/bootstrap.min.css';

function Mappings ({data}) {
    const [mappings, setMappings] = useState([]);
    const [deleteURL , setDeleteURL] = useState("")
    const [deleteShow, setDeleteShow] = useState(false)

    const handeDeleteClose = () => setDeleteShow(false);
    const handleDeleteShow = () => setDeleteShow(true);

    // 复制链接
    const copyToClipboard = async (url) => {
      try {
        await clipboard.writeText(url);
        alert('URL copied to clipboard');
      } catch (err) {
        console.error('Could not copy text: ', err);
      }
    };
    

    // 删除映射
    const handeDeleteMapping = (url) => {
        // 设置要删除的URL
        setDeleteURL(url);
        // 确定删除页面
        handleDeleteShow();
    }

    // 从数据库中删除条目的函数，并刷新页面
    const deleteMapping = async (url) => {
    console.log('Deleting mapping for URL:', url);
        try {
            await invoke('delete_mapping', { url });
            location.reload();
        } catch (error) {
            console.error('Failed to delete mapping:', error);
        }
    };

    useEffect(() => {
       setMappings(data) 
    },[data])

    return (
        <div>
        {/* mappings */}
        <Table className='mappings_table'>
            <thead>
            <tr>
                <th>#</th>
                <th>Name</th>
                <th>Origin Path</th>
                <th>URL</th>
                <th></th>
                <th></th>
            </tr>
            </thead>
            <tbody>
            {mappings.map((mapping, index) => (
                <tr key={index}>
                <td>{index + 1}</td>
                <td>{mapping.file_name}</td>
                <td>{mapping.origin_path}</td>
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
    )
}

export default Mappings;