import { useState } from 'react';
import { Dropdown, Modal, Button } from 'react-bootstrap';

const lngs = {
  'zh-CN': { nativeName: '简体中文' },
  en: { nativeName: 'English' },
  ja: { nativeName: '日本語' },
  es: { nativeName: 'Español' },
  hi: { nativeName: 'हिन्दी' },
  ar: { nativeName: 'العربية' },
  fr: { nativeName: 'Français' },
};


function Settings({t, i18n}) {
  const [show, setShow] = useState(false);

  const handleClose = () => setShow(false);
  const handleShow = () => setShow(true);

  const before_control = () => {
    if (show) {
      return {
        position: "fixed",
        width: "100%",
        height: "100%",
        background: "rgba(0, 0, 0, 0.5)",
        zIndex: 1
      };
    } else {
      return {
        display: 'none',
      };
    }
  };

  return (
    <>
      <Button 
        variant="none" 
        onClick={handleShow}
        style={{position:"absolute", left:"1vw", top:"1vh"}}
        className='box_no_shadow'
        >
        <svg className='button_icon' viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg" >
          <path d="M661.333333 665.6l51.2 12.8 42.666667-72.533333-34.133333-38.4c4.266667-21.333333 4.266667-38.4 4.266666-55.466667s0-34.133333-4.266666-51.2l34.133333-38.4-42.666667-72.533333-51.2 12.8c-25.6-21.333333-55.466667-42.666667-89.6-51.2L554.666667 256h-85.333334l-17.066666 51.2c-34.133333 8.533333-64 25.6-89.6 51.2l-51.2-12.8-42.666667 72.533333 34.133333 38.4c-4.266667 21.333333-4.266667 38.4-4.266666 55.466667s0 34.133333 4.266666 51.2l-34.133333 38.4 42.666667 72.533333 51.2-12.8c25.6 21.333333 55.466667 42.666667 89.6 51.2L469.333333 768h85.333334l17.066666-51.2c34.133333-8.533333 64-25.6 89.6-51.2z m38.4 81.066667c-21.333333 17.066667-51.2 34.133333-76.8 42.666666L597.333333 853.333333h-170.666666l-25.6-64c-29.866667-12.8-55.466667-25.6-76.8-42.666666l-68.266667 12.8-85.333333-149.333334 42.666666-51.2V512c0-17.066667 0-29.866667 4.266667-42.666667l-42.666667-51.2 85.333334-149.333333 68.266666 12.8c21.333333-17.066667 51.2-34.133333 76.8-42.666667L426.666667 170.666667h170.666666l25.6 64c29.866667 12.8 55.466667 25.6 76.8 42.666666l68.266667-12.8 85.333333 149.333334-42.666666 51.2c4.266667 12.8 4.266667 29.866667 4.266666 42.666666s0 29.866667-4.266666 42.666667l42.666666 51.2-85.333333 149.333333-68.266667-4.266666zM512 554.666667c25.6 0 42.666667-17.066667 42.666667-42.666667s-17.066667-42.666667-42.666667-42.666667-42.666667 17.066667-42.666667 42.666667 17.066667 42.666667 42.666667 42.666667z m0 85.333333c-72.533333 0-128-55.466667-128-128s55.466667-128 128-128 128 55.466667 128 128-55.466667 128-128 128z" p-id="1119"></path>
          <title>{t('settings.title')}</title>
        </svg>
      </Button>
      <Modal
        style={{
            zIndex: 9999,
        }}
        show={show}
        onHide={handleClose}
        backdrop="static"
        keyboard={true}
        size='sm'
        centered
      >
        <Modal.Header closeButton>
          <Modal.Title>{t('settings.title')}</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <LanguagesSelector t={t} i18n={i18n}/>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={handleClose}>
          {t('common.cancel')}
          </Button>
          <Button variant="primary">{t('common.save')}</Button>
        </Modal.Footer>
      </Modal>
      
      <div style={before_control()}></div>
    </>
  );
}

export default Settings;

function LanguagesSelector({t, i18n}) {
  return (
    <Dropdown onSelect={(eventKey) => i18n.changeLanguage(eventKey)}>
      <Dropdown.Toggle>
        {t('settings.languages')}
      </Dropdown.Toggle>
      <Dropdown.Menu>
          {Object.keys(lngs).map((lng) => (
              <Dropdown.Item eventKey={lng} key={lng} style={{ fontWeight: i18n.resolvedLanguage === lng ? 'bold' : 'normal' }}>
                  {lngs[lng].nativeName}
              </Dropdown.Item>
          ))}
      </Dropdown.Menu>
    </Dropdown>
  )
}
