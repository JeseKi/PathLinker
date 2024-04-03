import { Table , Button , Modal} from 'react-bootstrap';
import { useEffect, useState } from 'react'; 
import { invoke } from '@tauri-apps/api/tauri';
import { clipboard } from '@tauri-apps/api';
import 'bootstrap/dist/css/bootstrap.min.css';

function Mappings ({data , t}) {
    const [mappings, setMappings] = useState([]);
    const [deleteURL , setDeleteURL] = useState("")
    const [deleteShow, setDeleteShow] = useState(false)

    const handeDeleteClose = () => setDeleteShow(false);
    const handleDeleteShow = () => setDeleteShow(true);

    // 复制链接
    const copyToClipboard = async (url) => {
      try {
        await clipboard.writeText(url);
        alert(t('index.copy_done'));
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
                <th>{t('index.name')}</th>
                <th>{t('index.origin_path')}</th>
                <th>{t('index.url')}</th>
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
                    {t('index.copy')}
                    </Button>
                </td>
                <td>
                    {/* delete button */}
                    <Button variant="danger" onClick={() => handeDeleteMapping(mapping.url)}>
                    {t('index.delete')}
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
                <Modal.Title>{t('index.confirm_delete')}</Modal.Title>
                </Modal.Header>
                <Modal.Footer>
                <Button variant="secondary" onClick={handeDeleteClose}>
                {t('settings.cancel')}
                </Button>
                <Button variant="danger" onClick={() => deleteMapping(deleteURL)}>
                {t('index.confirm')}
                </Button>
                </Modal.Footer>
            </Modal>
        </div>
    )
}

export default Mappings;