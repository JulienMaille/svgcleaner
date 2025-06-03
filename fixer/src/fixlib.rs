use crate::fixes::{
    task::Task,
    // pointless_transform::PointlessTransform,
    empty_property::EmptyProperty,
    view_box_pt::ViewBoxPt,
    external_style::ExternalStyle,
    font_data::FontData,
    add_view_box::AddViewBox,
    pointless_xmlns::PointlessXmlns,
};

pub fn fix_svg(svg_data:String) -> String {

    let mut stack: Vec<Box<dyn Task>> = Vec::new();
    stack.push(Box::new(PointlessXmlns));
    stack.push(Box::new(EmptyProperty));
    // stack.push(Box::new(PointlessTransform)); // Note: Turns out the fix isn't as simple
    stack.push(Box::new(AddViewBox)); // <-- Best before ViewBoxPt
    stack.push(Box::new(ViewBoxPt));
    stack.push(Box::new(ExternalStyle));
    stack.push(Box::new(FontData));

    let mut data = svg_data;

    while let Some(top) = stack.pop() {
        data = top.fix(&data);
    };
    
    return data;
}