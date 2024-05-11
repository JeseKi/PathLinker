import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

i18n
  // detect user language
  // learn more: https://github.com/i18next/i18next-browser-languageDetector
  .use(LanguageDetector)
  // pass the i18n instance to react-i18next.
  .use(initReactI18next)
  // init i18next
  // for all options read: https://www.i18next.com/overview/configuration-options
  .init({
    debug: true,
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false, // not needed for react as it escapes by default
    },
    resources: {
      en: {
        translation: {
          settings: {
            languages: 'Languages',
            title : "Settings",
          },
          index: {
            name: "Name",
            origin_path: "Origin Path",
            url : "URL",
            select_files : "Select Files",
            refresh: 'Refresh page',
            change_folder: "Copy/move to other folder",
            copy_done: "URL copied to clipboard",
            delete : "Delete",
            confirm_delete: "Confirm deletion of mapping?",
          },
          common: {
            cancel: "Cancel",
            save: "Save",
            copy : "Copy",
            confirm: "Confirm",
          }
        }
      },
      'zh-CN' : {
        translation: {
          settings: {
            languages: '选择语言',
            title : "设置",
          },
          index: {
            name: "文件名",
            origin_path: "原始路径",
            url : "URL",
            select_files : "选择文件",
            refresh: "刷新页面",
            change_folder: "复制/转移到其他文件夹",
            copy_done: "URL已复制到剪切板",
            delete: "删除",
            confirm_delete: "确定删除映射？"
          },
          common: {
            cancel: "取消",
            save: "保存",
            copy : "复制",
            confirm: "确定",
          }
        }
      },
      ja: {
        translation: {
          settings: {
            languages: '言語',
            title: "設定",
          },
          index: {
            name: "名前",
            origin_path: "元のパス",
            url: "URL",
            select_files: "ファイルを選択",
            refresh: 'ページをリフレッシュ',
            change_folder:"他のフォルダにコピー/移動",
            copy_done: "URLをクリップボードにコピーしました",
            delete: "削除",
            confirm_delete: "マッピングを削除してもよろしいですか？",
          },
          common: {
            cancel: "キャンセル",
            save: "保存",
            copy : "コピー",
            confirm: "確認",
          }
        }
      },
      es: {
        translation: {
          settings: {
            languages: 'Idiomas',
            title: "Configuración",
          },
          index: {
            name: "Nombre",
            origin_path: "Ruta de origen",
            url: "URL",
            select_files: "Seleccionar archivos",
            refresh: 'Actualizar página',
            change_folder: "opiar/Mover a otra carpeta",
            copy_done: "URL copiada al portapapeles",
            delete: "Eliminar",
            confirm_delete: "¿Confirmar la eliminación del mapeo?",
          },
          common: {
            cancel: "Cancelar",
            save: "Guardar",
            copy : "Copiar",
            confirm: "Confirmar",
          }
        }
      },
      hi: {
        translation: {
          settings: {
            languages: 'भाषाएँ',
            title: "सेटिंग्स",
          },
          index: {
            name: "नाम",
            origin_path: "मूल पथ",
            url: "URL",
            select_files: "फ़ाइलें चुनें",
            refresh: 'पृष्ठ ताज़ा करें',
            change_folder: "दूसरे फ़ोल्डर में कॉपी/मूव करें",
            copy_done: "URL क्लिपबोर्ड पर कॉपी किया गया",
            delete: "हटाएं",
            confirm_delete: "मैपिंग हटाने की पुष्टि करें?",
          },
          common: {
            cancel: "रद्द करें",
            save: "सहेजें",
            copy : "कॉपी",
            confirm: "पुष्टि करें",
          }
        }
      },
      ar: {
        translation: {
          settings: {
            languages: 'اللغات',
            title: "الإعدادات",
          },
          index: {
            name: "الاسم",
            origin_path: "المسار الأصلي",
            url: "URL",
            select_files: "اختيار الملفات",
            refresh: 'تحديث الصفحة' ,
            change_folder: "نسخ/نقل إلى مجلد آخر",
            copy_done: "تم نسخ الرابط إلى الحافظة",
            delete: "حذف",
            confirm_delete: "تأكيد حذف التعيين؟",
          },
          common: {
            cancel: "إلغاء",
            save: "حفظ",
            copy : "نسخ",
            confirm: "تأكيد",
          }
        }
      },
      fr: {
        translation: {
          settings: {
            languages: 'Langues',
            title: "Paramètres",
          },
          index: {
            name: "Nom",
            origin_path: "Chemin d'origine",
            url: "URL",
            select_files: "Sélectionner des fichiers",
            refresh: 'Rafraîchir la page',
            change_folder: "Copier/Déplacer vers un autre dossier",
            copy_done: "URL copiée dans le presse-papier",
            delete: "Supprimer",
            confirm_delete: "Confirmer la suppression de la cartographie ?",
          },
          common: {
            cancel: "Annuler",
            save: "Enregistrer",
            copy : "Copier",
            confirm: "Confirmer",
          }
        }
      }
    }
  });

export default i18n;
