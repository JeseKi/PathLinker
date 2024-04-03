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
        variant="secondary" 
        onClick={handleShow}
        style={{position:"absolute", left:"1vw", top:"1vh"}}
        >
        {t('settings.title')}
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
          {t('settings.cancel')}
          </Button>
          <Button variant="primary">{t('settings.save')}</Button>
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
