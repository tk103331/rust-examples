slint::slint!(import { Recipe } from "clickable_button.slint";);

fn main() {
    let recipe = Recipe::new();
    let recipe_weak = recipe.as_weak();
    recipe.on_button_pressed(move || {
        let recipe = recipe_weak.upgrade().unwrap();
        let mut value = recipe.get_counter();
        value = value + 1;
        recipe.set_counter(value);
    });
    recipe.run();
}