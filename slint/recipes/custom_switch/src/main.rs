slint::slint!(import { Recipe } from "custom_switch.slint";);

fn main() {
    let recipe = Recipe::new();
    recipe.run();
}