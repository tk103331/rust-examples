slint::slint!{
    import { HorizontalBox, VerticalBox, LineEdit } from "std-widgets.slint";
    
    export global Logic := {
        callback to-upper-case(string) -> string;
        // You can collect other global properties here
    }
    
    export Recipe := Window {
        VerticalBox {
            input := LineEdit {
                text: "Text to be transformed";
            }
            HorizontalBox {
                Text { text: "Transformed:"; }
                // Callback invoked in binding expression
                Text {
                    text: {
                        Logic.to-upper-case(input.text);
                    }
                }
            }
        }
    }
    }
    
    fn main() {
        let recipe = Recipe::new();
        recipe.global::<Logic>().on_to_upper_case(|string| {
            string.as_str().to_uppercase().into()
        });
        recipe.run();
    }