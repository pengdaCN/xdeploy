use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Index};
use cursive::views::Dialog;
use serde::{Serialize, Deserialize};
use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "root")]
struct Layout {
    export: Export,
    #[serde(rename = "view")]
    views: Views,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "export")]
struct Export {
    vars: Option<Vars>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "vars")]
struct Vars {
    #[serde(rename = "$value")]
    data: Vec<Var>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "view")]
struct Views {
    #[serde(rename = "$value")]
    data: Vec<View>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Var {
    name: String,
    #[serde(rename = "type")]
    _type: Option<VarType>,
    #[serde(rename = "$value")]
    dft_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum VarType {
    #[serde(rename = "str")]
    Str,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "list")]
    List,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "input")]
struct InputView {
    phrase: String,
    bind: Option<String>,
    #[serde(default)]
    readonly: bool,
    length: Option<i32>,
    #[serde(rename = "$value")]
    dft_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "checkbox")]
struct CheckboxView {
    #[serde(default)]
    readonly: bool,
    bind: Option<String>,
    item: Vec<CheckboxItemView>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "radio")]
struct RadioView {
    #[serde(default)]
    readonly: bool,
    bind: Option<String>,
    item: Vec<CheckboxItemView>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "item")]
struct CheckboxItemView {
    phrase: String,
    id: String,
    set: Option<String>,
    #[serde(default)]
    selected: bool,
    #[serde(default)]
    disable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "text")]
struct TextView {
    #[serde(rename = "$value")]
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "switch")]
struct SwitchView {
    bind: Option<String>,
    phrase: String,
    #[serde(default)]
    readonly: bool,
    #[serde(default)]
    selected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum View1 {
    Input(InputView),
    Text(TextView),
    Switch(SwitchView),
    Checkbox(CheckboxView),
    Radio(RadioView),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "box")]
struct BoxView {
    #[serde(rename = "$value")]
    children: Vec<View1>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "tabs")]
struct TabsView {
    #[serde(rename = "$value")]
    children: Vec<TabsPaneView>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "tabs")]
struct TabsPaneView {
    value: Option<String>,
    #[serde(default)]
    selected: bool,
    #[serde(rename = "$value")]
    children: Vec<View1>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum View2 {
    Box(BoxView),
    Tabs(TabsView),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum View {
    Input(InputView),
    Text(TextView),
    Switch(SwitchView),
    Checkbox(CheckboxView),
    Radio(RadioView),
    Box(BoxView),
    Tabs(TabsView),
}

impl View {
    fn binding_id(&self) -> Option<&String> {
        match self {
            View::Input(view) => {
                view.bind.as_ref()
            }
            View::Text(view) => {
                None
            }
            View::Switch(view) => {
                view.bind.as_ref()
            }
            View::Checkbox(view) => {
                view.bind.as_ref()
            }
            View::Radio(view) => {
                view.bind.as_ref()
            }
            View::Box(view) => {
                None
            }
            View::Tabs(view) => {
                None
            }
        }
    }

    fn expect_var_type(&self) -> Option<VarType> {
        match self {
            View::Input(_) | View::Radio(_) => {
                Some(VarType::Str)
            }
            View::Switch(_) => {
                Some(VarType::Bool)
            }
            View::Checkbox(_) => {
                Some(VarType::List)
            }
            _ => None
        }
    }

    fn visit_step(&self, step: usize) -> Option<&Self> {
        if step == 0 {
            return Some(self);
        }

        unreachable!()
    }
}


#[derive(Serialize, Deserialize, Debug)]
enum Value {
    Str(String),
    Bool(bool),
    List(Vec<String>),
}

struct Values(HashMap<String, Option<Value>>);

impl Deref for Values {
    type Target = HashMap<String, Option<Value>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Values {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct Layer {
    vars: Vec<(String, VarType)>,
    view: Dialog,
    values: Values,
}

impl Layer {
    pub fn from_layout(layout: Layout) -> crate::Result<Self> {
        let mut vars = Vec::new();
        // let mut values = HashMap::new();
        // 处理vars
        if let Some(v) = layout.export.vars {
            for x in v.data {
                // 获取当前id匹配的所有view
                let bindings: Vec<_> = layout.views.data
                    .iter()
                    .filter(|view| {
                        let binding = view.binding_id();

                        if let Some(binding_id) = binding {
                            *binding_id == x.name
                        } else {
                            false
                        }
                    }).collect();

                // 查询所有binding该id的view的期望类型
                let mut types: Vec<_> = bindings
                    .clone()
                    .into_iter()
                    .map(|binding| {
                        binding.expect_var_type().expect("bug!!!")
                    })
                    .collect();
                types.sort();
                types.dedup();

                if types.len() > 1 {
                    // 引用该id的变量推断除多个类型
                    return Err(Error::VarTypeConflict {
                        id: x.name,
                    });
                }

                // 推断出当前id的类型d
                let mut ty: VarType;
                if let Some(v) = x._type {
                    ty = v;
                    if let Some(infer_type) = types.get(0) {
                        if *infer_type != ty {
                            // 推断类型，与指定类型不匹配
                            return Err(Error::VarTypeConflict {
                                id: x.name,
                            });
                        }
                    }
                } else if let Some(infer_type) = types.get(0) {
                    ty = infer_type.clone()
                } else {
                    return Err(Error::VarTypeNonInfer {
                        id: x.name,
                    });
                }

                vars.push((x.name, ty));
            }
        }

        // 处理view中绑定的变量


        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use crate::ui::{BoxView, Export, Layout, View, View2};

    #[test]
    fn pwd() {
        println!("{:?}", std::env::current_dir().unwrap())
    }

    #[test]
    fn unmarshal() {
        let ui = fs::read_to_string("./ui.xml").unwrap();
        let layout: Layout = quick_xml::de::from_str(&ui).unwrap();

        println!("layout {layout:#?}")
    }

    #[test]
    fn unmarshal_box() {
        let _box = r#"
        <box>
            <input bind="site_name" phrase="网站名" length="10"/>
            <input bind="site_addr" phrase="网站地址" length="20"/>
            <switch phrase="是否公开网络"/>
            <switch phrase="是否允许所有连接" selected="true"/>
            <checkbox readonly="true">
                <item set="install_qq" phrase="安装qq" selected="true" id="qq" disable="true"/>
                <item set="install_wechat" phrase="安装wechat" id="wechat"/>
            </checkbox>

            <radio>
                <item id="set-1" phrase="设置1"/>
                <item id="set-2" phrase="设置2"/>
            </radio>
        </box>
        "#;

        let _box: BoxView = quick_xml::de::from_str(_box).unwrap();

        println!("{_box:#?}");
    }

    #[test]
    fn unmarshal_tabs() {
        let tabs = r#"
        <tabs bind="install_ty">
            <!--按钮攀旁边的字-->
            <pane value="install" phrase="安装数据库">
                <!--勾选按钮，只有true和false-->
                <input phrase="数据库密码"/>
                <switch phrase="是否公开网络"/>
                <switch phrase="是否允许所有连接" selected="true"/>
            </pane>

            <!--默认选中项目-->
            <pane value="conn" selected="true">
                <input phrase="地址" length="10"/>
                <input phrase="登录用户名" length="20">postgres</input>
                <input phrase="登录密码" length="20"/>
                <input phrase="连接数据库" length="10" readonly="true">{{c_db_name}}</input>
            </pane>
        </tabs>
        "#;

        let tabs: View2 = quick_xml::de::from_str(tabs).unwrap();

        println!("{tabs:#?}");
    }

    #[test]
    fn unmarshal_view() {
        let input = r#"
        <switch phrase="是否允许所有连接" selected="true"/>
        "#;

        let view: View = quick_xml::de::from_str(input).unwrap();

        println!("{view:#?}");
    }

    #[test]
    fn unmarshal_export() {
        let export = r#"<export>
        <vars>
            <var name="name"/>
            <var name="passwd">123456</var>
            <var name="site_name"/>
            <var name="site_addr"/>
            <var name="install_ty"/>
            <var name="i_db_passwd"/>
            <!--后续添加特性，可以省略，在bind参数时推断-->
            <var name="i_db_enable_pubnet" type="bool"/>
            <var name="i_db_enable_all_conn" type="bool"/>
            <var name="c_db_addr"/>
            <var name="c_db_username"/>
            <var name="c_db_passwd"/>
            <var name="c_db_name">my_db</var>

            <var name="install_qq" type="bool"/>
            <var name="install_wechat" type="bool"/>

        </vars>
    </export>"#;

        let export: Export = quick_xml::de::from_str(export).unwrap();

        println!("{export:#?}");
    }
}