slint::slint!(import { Recipe } from "states.slint";);

fn main() {
    let recipe = Recipe::new();
    recipe.run();
}