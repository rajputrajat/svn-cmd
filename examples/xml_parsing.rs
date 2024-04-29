use svn_cmd::SvnList;

fn main() {
    #[cfg(feature = "attach_debugger")]
    {
        use bugsalot::debugger;
        debugger::wait_until_attached(None).expect("Could't attach the debugger");
    }

    let xml_text: SvnList = serde_xml_rs::from_str(LIST_XML.trim()).unwrap();
    println!("{:?}", xml_text);
}

const LIST_XML: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<lists>
<list>
<entry
   kind="file">
<name>BaoZhuZhaoFu_PurpleCelebration.vcproj</name>
<size>20617</size>
<commit
   revision="373439">
<author>sa102001</author>
<date>2023-07-21T05:31:45.995541Z</date>
</commit>
</entry>
</list>
</lists>"##;
