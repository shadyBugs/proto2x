use std::path::{PathBuf};
use std::rc::Rc;

macro_rules! add_comment_field {
    ($($s_name:ident),*) => {
        $(
            pub struct $s_name {
                pub comment: String,
            }

            impl $s_name {
                pub fn get_comment(&self)->String{
                    String::from(self.comment)
                }
                pub fn set_comment(&mut self, comment:&str) {
                    self.comment = String::from(comment);
                }
            }
        )*
    };
}

pub struct ProtoFile {
    pub name: String,
    pub path: String,
    pub service_list: Vec<Service>,
    pub import_file_list: Vec<ImportedFile>,
    pub message_list: Vec<Message>,
}
pub struct ImportedFile {
    pub path: PathBuf,
    pub name: String,
    pub alias: Option<String>,
    pub real_file: Rc<ProtoFile>,
    pub comment: String,
}
pub struct Service {
    pub func_list: Vec<Func>,
    pub comment: String,
}
pub struct Func {}
pub struct Message {
    pub name: String,
    pub fields: Vec<MessageField>,
}
pub enum MessageFieldType {
    ScalaType(ScalaTypeEnum),
    Enum(Enumeration),
    OtherMessage(Message),
    Map { key: ScalaTypeEnum, value: ScalaTypeEnum },
}
pub struct MessageField {
    pub is_repeat: bool,
    pub field_type: MessageFieldType,
    pub name: String,
    pub sequence: i32,
}
pub struct Enumeration {
    pub name: String,
    pub element_list: Vec<EnumerationElem>,
}
pub struct EnumerationElem {
    pub name: String,
    pub value: i32,
}
pub enum ScalaTypeEnum {
    DoubleType,
    FloatType,
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFix32,
    SFixed64,
    BoolType,
    StringType,
    BytesType,
}

add_comment_field!(ProtoFile,ImportedFile,Service,Func,Message,MessageField,Enumeration,EnumerationElem);