use handlebars::{handlebars_helper, Handlebars, Renderable};
use serde_json;
use serde_json::{json, Value as Json};
use std::fs;

fn main() {
    let mut handlebars = Handlebars::new();

    let source = fs::read_to_string("test.hbs").unwrap();

    handlebars_helper!(fallback: |*args| {
        let mut result = Json::Null;
        for value in args {
            match value {
                Json::Null => (),
                _ => {
                    result = value.clone();
                    break;
                }
            };
        }
        result
    });

    fn repeat_block_helper<'r, 'reg, 'rc, 's, 't0>(
        helper: &'r handlebars::Helper<'reg, 'rc>,
        registry: &'reg Handlebars<'reg>,
        context: &'rc handlebars::Context,
        render_context: &'s mut handlebars::RenderContext<'reg, 'rc>,
        out: &'t0 mut (dyn handlebars::Output + 't0),
     ) -> Result<(), handlebars::RenderError> {
         helper
    .template()
             .map(|template| {
                 let mut result = Ok(());
                 let repeat_count = match helper.param(0) {
                     Some(param) => match param.value().as_u64() {
                         Some(value) => value as u32,
                         None => 1,
                     },
                     None => 1,
                 };
                 for i in 0..repeat_count {
                   let mut new_context = context.clone();
                       new_context.data_mut().as_object_mut().unwrap().insert("index".to_string(), json!(i));
                    render_context.set_context(new_context);
                     match template.render(registry, context, render_context, out) {
                         Ok(_) => (),
                         Err(err) => {
                             result = Err(err);
                         }
                     
                     };
                     render_context.set_context(context.clone());
                 };
                 result
             })
             .unwrap_or(Ok(()))
    }

    handlebars
        .register_template_string("template", &source)
        .unwrap();
    handlebars.register_helper("fallback", Box::new(fallback));
    handlebars.register_helper("repeat", Box::new(repeat_block_helper));

    let result = handlebars
        .render(
            "template",
            &json!({
                "b": "lolz"
            }),
        )
        .unwrap();

    fs::write("result", result.as_bytes()).unwrap();

    println!("{}", result);
}
