use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "root")]
struct Layout {
    export: Export,

}

#[derive(Serialize, Deserialize, Debug)]
struct Export {
    vars: Option<Vec<Var>>,
}


#[derive(Serialize, Deserialize, Debug)]
struct Var {
    name: String,
    _type: Option<VarType>,
    #[serde(rename = "$value")]
    dft_value: Option<VarValue>,
}

#[derive(Serialize, Deserialize, Debug)]
enum VarType {
    #[serde(rename = "str")]
    Str,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "list")]
    List,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum VarValue {
    Str(String),
    Bool(bool),
    List(Vec<String>),
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "input")]
struct InputView {
    phrase: String,
    bind: Option<String>,
    readonly: bool,
    length: i32,
    #[serde(rename = "$value")]
    dft_value: Option<VarValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "checkbox")]
struct CheckboxView {
    readonly: bool,
    bind: Option<String>,
    item: Vec<CheckboxItemView>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "radio")]
struct RadioView {
    readonly: bool,
    bind: Option<String>,
    item: Vec<CheckboxItemView>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "item")]
struct CheckboxItemView {
    phrase: String,
    id: String,
    set: Option<String>,
    selected: bool,
    disable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "text")]
struct TextView {
    #[serde(rename = "$value")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "switch")]
struct SwitchView {
    bind: Option<String>,
    phrase: String,
    readonly: bool,
    selected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
enum View1 {
    Input(InputView),
    Text(TextView),
    Switch(SwitchView),
    Checkbox(CheckboxView),
    Radio(RadioView),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "box")]
struct BoxView {
    children: Vec<View1>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "tabs")]
struct TabsView {

}