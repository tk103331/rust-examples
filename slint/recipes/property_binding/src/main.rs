slint::slint!(import { Recipe } from "property_binding.slint";);

fn main() {
    let recipe = Recipe::new();
    recipe.run();
}