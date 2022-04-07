use druid::widget::prelude::*;
use druid::Lens;

use std::sync::Arc;
use rune::{Sources, Vm};

//Re-export druid
pub use druid;

//Re-export rune
pub use rune;

//Re-export simplecss
pub use simplecss;

//Re-export visdom
pub use visdom;

use simplecss::StyleSheet;
use visdom::Vis;


#[derive(Default,Clone,Data,Lens)]
struct DruidRuneState {

}

impl DruidRuneState {
    pub fn new(css:StyleSheet, dom:Vis, script:Sources) {

    }
}

#[cfg(test)]
mod test {
    use simplecss::StyleSheet;
    use visdom::Vis;

    #[test]
    fn ugly_css_html_parse() {
        let dom = r#"
        <html>
        <style>
        div { padding: 10px; }
        button { background-color: #6f6a6a; }
        #btn_red { background-color: red; }
        </style>

        <style>
        anotehr {}
        </style>
        <body>
        <div id="container">
            Button Test
            <button id="mybtn">OK</button>
            <button id="btn_red">Red Button</button>
        </div>
        </body>
        </html>
        "#;
        let vis = Vis::load( dom ).unwrap();
        let css = vis.find("style").into_iter().fold( String::new(), |mut sv, s| {
            sv.push_str( &s.html() );
            sv
        });
        let mut css = StyleSheet::parse(&css);

        println!("{:?}",css);

    }
}