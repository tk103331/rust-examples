slint::slint!(import { Recipe } from "custom_table.slint";);

fn main() {
    let recipe = Recipe::new();
    recipe.run();
}