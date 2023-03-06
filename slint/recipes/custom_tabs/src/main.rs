slint::slint!(import { Recipe } from "custom_tabs.slint";);

fn main() {
    let recipe = Recipe::new();
    recipe.run();
}