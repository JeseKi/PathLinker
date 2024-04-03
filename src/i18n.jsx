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
            cancel: "Cancel",
            save: "Save"
          },
          index: {
            name: "Name",
            origin_path: "Origin Path",
            url : "URL",
            select_files : "Select Files",
            copy : "Copy",
            copy_done: "URL copied to clipboard",
            delete : "Delete",
            confirm: "Confirm",
            confirm_delete: "Confirm deletion of mapping?",
          }
        }
      },
      'zh-CN' : {
        translation: {
          settings: {
            languages: '选择语言',
            title : "设置",
            cancel: "取消",
            save: "保存"
          },
          index: {
            name: "文件名",
            origin_path: "原始路径",
            url : "URL",
            select_files : "选择文件",
            copy: "复制",
            copy_done: "URL已复制到剪切板",
            delete: "删除",
            confirm: "确定",
            confirm_delete: "确定删除映射？"
          }
        }
      },
      ja: {
        translation: {
          settings: {
            languages: '言語',
            title: "設定",
            cancel: "キャンセル",
            save: "保存"
          },
          index: {
            name: "名前",
            origin_path: "元のパス",
            url: "URL",
            select_files: "ファイルを選択",
            copy: "コピー",
            copy_done: "URLをクリップボードにコピーしました",
            delete: "削除",
            confirm: "確認",
            confirm_delete: "マッピングを削除してもよろしいですか？",
          }
        }
      },
      es: {
        translation: {
          settings: {
            languages: 'Idiomas',
            title: "Configuración",
            cancel: "Cancelar",
            save: "Guardar"
          },
          index: {
            name: "Nombre",
            origin_path: "Ruta de origen",
            url: "URL",
            select_files: "Seleccionar archivos",
            copy: "Copiar",
            copy_done: "URL copiada al portapapeles",
            delete: "Eliminar",
            confirm: "Confirmar",
            confirm_delete: "¿Confirmar la eliminación del mapeo?",
          }
        }
      },
      hi: {
        translation: {
          settings: {
            languages: 'भाषाएँ',
            title: "सेटिंग्स",
            cancel: "रद्द करें",
            save: "सहेजें"
          },
          index: {
            name: "नाम",
            origin_path: "मूल पथ",
            url: "URL",
            select_files: "फ़ाइलें चुनें",
            copy: "कॉपी",
            copy_done: "URL क्लिपबोर्ड पर कॉपी किया गया",
            delete: "हटाएं",
            confirm: "पुष्टि करें",
            confirm_delete: "मैपिंग हटाने की पुष्टि करें?",
          }
        }
      },
      ar: {
        translation: {
          settings: {
            languages: 'اللغات',
            title: "الإعدادات",
            cancel: "إلغاء",
            save: "حفظ"
          },
          index: {
            name: "الاسم",
            origin_path: "المسار الأصلي",
            url: "URL",
            select_files: "اختيار الملفات",
            copy: "نسخ",
            copy_done: "تم نسخ الرابط إلى الحافظة",
            delete: "حذف",
            confirm: "تأكيد",
            confirm_delete: "تأكيد حذف التعيين؟",
          }
        }
      },
      fr: {
        translation: {
          settings: {
            languages: 'Langues',
            title: "Paramètres",
            cancel: "Annuler",
            save: "Enregistrer"
          },
          index: {
            name: "Nom",
            origin_path: "Chemin d'origine",
            url: "URL",
            select_files: "Sélectionner des fichiers",
            copy: "Copier",
            copy_done: "URL copiée dans le presse-papier",
            delete: "Supprimer",
            confirm: "Confirmer",
            confirm_delete: "Confirmer la suppression de la cartographie ?",
          }
        }
      }
    }
  });

export default i18n;
