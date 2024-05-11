import { Table , Button , Modal, Form, Row, Col , Accordion} from 'react-bootstrap';
import { useEffect, useState } from 'react'; 
import { open } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
import { clipboard } from '@tauri-apps/api';
import 'bootstrap/dist/css/bootstrap.min.css';

export {Mappings};

function Mappings ({data , t}) {
    const [mappings, setMappings] = useState([]);
    const [deleteURL , setDeleteURL] = useState("")
    const [deleteShow, setDeleteShow] = useState(false)

    const [changeFolderShow , setChangeFolderShow] = useState(false)

    const handleDeleteClose = () => setDeleteShow(false);
    const handleDeleteShow = () => setDeleteShow(true);

    const handleChangeFolderClose = () => setChangeFolderShow(false);
    const handleChangeFolderShow = () => setChangeFolderShow(true);

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
    const handleDeleteMapping = (url) => {
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

            {/* folders manage & file select */}
            <div className="button-container">
                {/*<FoldersManager t={t}/>*/}
                <FileSelector t={t}/>
                <Button
                    variant="none" 
                    className='box_no_shadow'
                    onClick={() => {
                        location.reload();
                    }}
                >
                    <svg t="1714045278174" className='button_icon' viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="8428" id="mx_n_1714045278175" width="200" height="200">
                        <path d="M861.866667 349.866667l-102.4 102.4C733.866667 341.333333 631.466667 256 512 256c-140.8 0-256 115.2-256 256s115.2 256 256 256c136.533333 0 251.733333-106.666667 256-243.2l76.8-76.8c4.266667 21.333333 8.533333 42.666667 8.533333 68.266667 0 187.733333-153.6 341.333333-341.333333 341.333333s-341.333333-153.6-341.333333-341.333333 153.6-341.333333 341.333333-341.333334c110.933333 0 213.333333 55.466667 273.066667 136.533334l8.533333-12.8h119.466667l-51.2 51.2z" p-id="8429">
                        </path>
                        <title>{t('index.refresh')}</title>
                    </svg>
                </Button>
            </div>

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
                        <Button 
                            variant="none" 
                            className='box_no_shadow' 
                            onClick={() => copyToClipboard(mapping.url)}
                        >
                            <svg className='button_icon' viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg" p-id="11338" width="30" height="30">
                                <path d="M682.666667 341.333333h128v469.333334H341.333333v-128H213.333333V213.333333h469.333334v128z m0 85.333334v256h-256v42.666666h298.666666v-298.666666h-42.666666zM298.666667 298.666667v298.666666h298.666666V298.666667H298.666667z" p-id="11339"></path>
                                <title>{t('common.copy')}</title>
                            </svg>
                        </Button>
                    </td>
                    <td>
                        {/* folders manage button */}
                        {/*<Button 
                            variant="none" 
                            className='box_no_shadow' 
                            onClick={() => handleChangeFolderShow()}
                        >
                            <svg className='button_icon' viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
                                <path d="M853.333333 298.666667v512H170.666667V213.333333h221.866666l42.666667 42.666667H853.333333v42.666667zM256 341.333333v384h512V341.333333H256z m298.666667 256h170.666666v85.333334h-170.666666v-85.333334z"></path>
                                <title>{t('index.change_folder')}</title>
                            </svg>
                        </Button>*/}

                    </td>
                    <td>
                        {/* delete button */}
                        <Button
                            variant="none"
                            className='box_no_shadow'
                            onClick={() => handleDeleteMapping(mapping.url)}
                        >
                            <svg  className='button_icon_delete button_icon' viewBox="0 0 1024 1024">
                                <path d="M256 298.666667h512v554.666666H256V298.666667z m85.333333 85.333333v384h341.333334V384H341.333333z m42.666667 85.333333h85.333333v213.333334H384v-213.333334z m170.666667 0h85.333333v213.333334h-85.333333v-213.333334zM213.333333 298.666667h597.333334v85.333333H213.333333V298.666667z m170.666667-128h256v85.333333H384V170.666667z" p-id="9168"></path>
                                <title>{t('index.delete')}</title>
                            </svg>
                        </Button>
                    </td>
                    </tr>
                ))}
                </tbody>
            </Table>

            {/* confirm delete mapping */}
            <Modal 
                show={deleteShow} 
                onHide={handleDeleteClose}
                backdrop="static"
                keyboard={true}
                centered
                >
                <Modal.Header closeButton>
                <Modal.Title>{t('index.confirm_delete')}</Modal.Title>
                </Modal.Header>
                <Modal.Footer>
                <Button variant="secondary" onClick={handleDeleteClose}>
                {t('common.cancel')}
                </Button>
                <Button variant="danger" onClick={() => deleteMapping(deleteURL)}>
                {t('common.confirm')}
                </Button>
                </Modal.Footer>
            </Modal>

        {/* change mapping folder */}
            <Modal
                show={changeFolderShow}
                onHide={handleChangeFolderClose}
                keyboard={true}
                centered
            >
                <Modal.Header closeButton>
                    <Modal.Title>{t('index.change_folder')}</Modal.Title>
                </Modal.Header>
                <Modal.Body>
                    在这里更改文件夹
                </Modal.Body>
                <Modal.Footer>
                    <Form.Check
                    type='switch'
                    name="copyMappingToFolders"
                    label={t('common.copy')}
                    id="copyMappingToFolders"
                    />
                    <Button variant="secondary" onClick={handleChangeFolderClose}>{t('common.cancel')}</Button>
                    <Button variant="primary">{t('common.confirm')}</Button>
                </Modal.Footer>
            </Modal>
        </div>
    )
}

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
      <span>
          <Button onClick={openFileDialog}>{t('index.select_files')}</Button>
          {selectedFile && (
          <p style={{display:"none"}}>Selected file: {Array.isArray(selectedFile) ? selectedFile.join(', ') : selectedFile}</p>
          )}
      </span>
    );
  }

function FoldersManager({t}) {

    // folders manager modal control
    const [managerShow , setManagerShow] = useState(false);
    const handleManagerShow = () => {
        setManagerShow(true);
    }
    const handleManagerClose = () => {
        setManagerShow(false);
    }

    // fetch folders data
    const [folders , setFolders] = useState([])
    const fetchFolders = async () => {
        try {
            const data = await invoke('get_folders');
            setFolders(data);
          } 
          
          catch (error) {
            console.error('Failed to fetch folders:', error);
          }
    }

    // create new folder
    const [folderName , setFolderName] = useState('')
    const [folderCreateShow , setFolderCreateShow] = useState(false);

    const handleFolderCreateShow = () => {
        setFolderCreateShow(true);
    }
    const handleFolderCreateClose = () => {
        setFolderCreateShow(false);
    }

    const createFolder = async (folder_name) => {
        try {
            await invoke('create_folder', { folderName: folder_name });
            alert('Folder created successfully!');
            handleFolderCreateClose()
            fetchFolders()
        }
        catch (error) {
            console.log('Failed to create folder:', error);
            alert('Failed to create folder:', error)
        }
    }

    useEffect(() => {
        fetchFolders();
        console.log("已获取到文件夹",folders)
    },[])

    return (
        <span>

            <Button onClick={handleManagerShow}>文件夹管理</Button>
            
            {/* main modal */}
            <Modal
                show={managerShow}
                onHide={handleManagerClose}
                keyboard={true}
                backdrop='static'
                size='xl'
                centered
            >
                <Modal.Header closeButton>文件夹管理</Modal.Header>
                <Modal.Body>
                        <Row>
                            <Col xxl={2}>显示</Col>
                            <Col xxl={8}>名称</Col>
                            <Col></Col>
                            <Col></Col>
                        </Row>
                        {folders.map((folder, index) => (
                            <Accordion>
                                <Accordion.Item key={index} eventKey={index} className='box_no_shadow'>
                                    <Accordion.Header as={Row}>
                                        <Col xxl={2}>√</Col>
                                        <Col xxl={8}>{folder.folder_name}</Col>
                                        <Col><Button>编辑</Button></Col>
                                        <Col><Button>删除</Button></Col>
                                    </Accordion.Header>
                                    <Accordion.Body>
                                        <Row>
                                            <Col>文件名A</Col>
                                            <Col>文件A的URL</Col>
                                            <Col><Button>移出该文件夹</Button></Col>
                                        </Row>
                                    </Accordion.Body>
                                </Accordion.Item>
                            </Accordion>
                            )
                        )}
                </Modal.Body>
                <Modal.Footer>
                    <Button onClick={handleFolderCreateShow}>新建文件夹</Button>
                    <Button variant="secondary" onClick={handleManagerClose}>
                        取消
                    </Button>
                    <Button variant="primary" onClick={handleManagerClose}>
                        保存
                    </Button>
                </Modal.Footer>
            </Modal>

            {/* create folder modal */}
            <Modal
                show={folderCreateShow}
                onHide={handleFolderCreateClose}
                keyboard={true}
                backdrop='static'
                centered
            >
                <Modal.Header closeButton>新建文件夹</Modal.Header>
                <Modal.Body>
                    <Form.Label>名称</Form.Label>
                    <Form.Control 
                        type='text'
                        id='folder_name'
                        onChange={e => setFolderName(e.target.value)}
                    />
                </Modal.Body>
                <Modal.Footer>
                    <Button variant='secondary' onClick={handleFolderCreateClose}>取消</Button>
                    <Button variant='primary' onClick={() => createFolder(folderName)}>确定</Button>
                </Modal.Footer>
            </Modal>
        </span>
    )
}