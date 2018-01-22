
fn main() {

    let mut top_score = 0;
    enumerate_recipes_example(&mut |recipe| {
        let score = score_recipe_example(&recipe);
        if score > top_score {
            top_score = score;
        }
    });
    println!("example top score = {}", top_score);

    let top_score = find_best_recipe(&score_recipe);
    println!("part 1 top score = {}", top_score);

    let top_score = find_best_recipe(&score_recipe_calorie_constraint);
    println!("part 2 top score = {}", top_score);
}

fn score_recipe_example(r: &[u8]) -> i32 {
    let r = [r[0] as i32, r[1] as i32];
    let capacity   = zero_if_negative(-1 * r[0] +  2 * r[1]);
    let durability = zero_if_negative(-2 * r[0] +  3 * r[1]);
    let flavor     = zero_if_negative( 6 * r[0] + -2 * r[1]);
    let texture    = zero_if_negative( 3 * r[0] + -1 * r[1]);
    capacity * durability * flavor * texture
}

fn find_best_recipe<F: Fn(&[u8]) -> i32>(f: F) -> i32 {
    let mut top_score = 0;
    enumerate_recipes(&mut |recipe| {
        let score = f(&recipe);
        if score > top_score {
            top_score = score;
        }
    });
    top_score
}

fn score_recipe_calorie_constraint(r: &[u8]) -> i32 {
    let r32 = [r[0] as i32, r[1] as i32, r[2] as i32, r[3] as i32];
    let calories = 5 * r32[0] + 8 * r32[1] + 6 * r32[2] + 1 * r32[3];
    match calories {
        500 => score_recipe(r),
        _   => 0
    }
}

fn score_recipe(r: &[u8]) -> i32 {
    let r = [r[0] as i32, r[1] as i32, r[2] as i32, r[3] as i32];
    let capacity   = zero_if_negative( 4 * r[0] +  0 * r[1] + -1 * r[2] +  0 * r[3]);
    let durability = zero_if_negative(-2 * r[0] +  5 * r[1] +  0 * r[2] +  0 * r[3]);
    let flavor     = zero_if_negative( 0 * r[0] + -1 * r[1] +  5 * r[2] + -2 * r[3]);
    let texture    = zero_if_negative( 0 * r[0] +  0 * r[1] +  0 * r[2] +  2 * r[3]);
    capacity * durability * flavor * texture
}

fn zero_if_negative(i: i32) -> i32 {
    if i < 0 {
        0
    } else {
        i
    }
}

fn enumerate_recipes_example<F: FnMut(&[u8])>(f: &mut F) {
    let mut ingredients = [0u8; 2];
    let right = ingredients.len() - 1;
    enumerate_recipes_helper(&mut ingredients, right, 100, f);
}

fn enumerate_recipes<F: FnMut(&[u8])>(f: &mut F) {
    let mut ingredients = [0u8; 4];
    let right = ingredients.len() - 1;
    enumerate_recipes_helper(&mut ingredients, right, 100, f);
}

fn enumerate_recipes_helper<F: FnMut(&[u8])>(recipes: &mut [u8], right: usize, total: u8, f: &mut F) {
    if right == 1 {
        for i in 0..total + 1 {
            recipes[0] = i;
            recipes[1] = total - i;
            f(recipes);
        }
    } else {
        for i in 0..total + 1 {
            recipes[right] = i;
            enumerate_recipes_helper(recipes, right - 1, total - i, f);
        }
    }
}
