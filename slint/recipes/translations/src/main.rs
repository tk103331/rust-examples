slint::slint!{
import { HorizontalBox , Button } from "std-widgets.slint";

export global Tr := {
    // Do the translation of the first argument, with an array of string as supstitution
    callback gettext(string, [string]) -> string;
}

Example := Window {
    property <int> count;
    HorizontalBox {
        Button {
            text: Tr.gettext("Button pressed {0} times", [count]);
        }
    }
}
}

fn main() {
    let app = Example::new();
    app.global::<Tr>().on_gettext(|string, model| {
        use slint::Model;
        let mut str = gettextrs::gettext(string.as_str());
        for (idx, to) in model.iter().enumerate() {
            str = str.replace(&format!("%{}", idx + 1), to.as_str());
        }
        str.into()
    });
    //...
    app.run()
}