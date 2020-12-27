use core::panic;
use std::collections::{btree_map::Iter, HashMap, HashSet};
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("input.txt");
    let mut ingredientsLists = Vec::new();
    let mut allergensLists = Vec::new();
    for line in input.lines() {
        let mut newThing = line.split_terminator("(contains ");
        let ingridients = newThing.next().unwrap();
        let allergens = newThing.next().unwrap();
        let allergens = allergens.split_terminator(")").next().unwrap();
        ingredientsLists.push(ingridients.split_terminator(" ").collect::<HashSet<&str>>());
        allergensLists.push(allergens.split_terminator(", ").collect::<HashSet<&str>>());
    }
    let mut checkedAllergens = HashSet::new();
    let mut suspiciousIngredients = HashSet::new();
    for (index, ingredientsList) in ingredientsLists.iter().enumerate() {
        for &ingredient in ingredientsList {
            suspiciousIngredients.insert(ingredient);
        }
    }

    for (index, allergensList) in allergensLists.iter().enumerate() {
        for &allergen in allergensList {
            if checkedAllergens.contains(allergen) {
                continue;
            }
            let mut ingredientsIntersection = ingredientsLists[index].clone();
            for (index2, otherList) in allergensLists.iter().enumerate() {
                if otherList.contains(allergen) {
                    ingredientsIntersection = ingredientsIntersection
                        .intersection(&ingredientsLists[index2])
                        .cloned()
                        .collect();
                }
            }
            suspiciousIngredients = suspiciousIngredients
                .difference(&ingredientsIntersection)
                .cloned()
                .collect();
            checkedAllergens.insert(allergen);
        }
    }
    // for sus in &suspiciousIngredients {
    //     println!("{}", sus);
    // }
    for (index, ingredientsList) in ingredientsLists.iter().enumerate() {
        let mut count = 0;
        for ingredients in ingredientsList {
            if suspiciousIngredients.contains(ingredients) {
                // println!("{}", ingredients);
            } else {
                count = count + 1;
            }
        }
        println!("{} {}", count, allergensLists[index].len());
    }
    let mut result = HashMap::new();
    let mut result2 = HashMap::new();
    guessWork(
        &ingredientsLists,
        &allergensLists,
        &suspiciousIngredients,
        &mut result,
        &mut result2,
        0,
    );
    // let count = ingredientsLists.iter().flat_map(|ingredientsList| ingredientsList.iter().filter(|&ingredients| suspiciousIngredients.contains(ingredients))).count();
    // println!("{}", count);
    let mut keys: Vec<&str> = result.keys().cloned().collect();
    keys.sort_by(|a, b| result[a].cmp(result[b]));
    for key in &keys {
        print!("{},", key);
    }
    println!("");
    for key in &keys {
        print!("{},", result[key]);
    }
}

fn guessWork<'a>(
    ingerdientsLists: &'a Vec<HashSet<&'a str>>,
    allergensLists: &'a Vec<HashSet<&'a str>>,
    suspiciousIngredients: &HashSet<&str>,
    AllergensIngredientsList: &mut HashMap<&'a str, &'a str>,
    IngredientsAllergensList: &mut HashMap<&'a str, &'a str>,
    line: usize,
) -> bool {
    if line == ingerdientsLists.len() {
        for ingredientList in ingerdientsLists {
            for ingredient in ingredientList {
                if suspiciousIngredients.contains(ingredient) {
                    continue;
                }
                if AllergensIngredientsList.contains_key(ingredient) {
                    continue;
                }
                return false;
            }
        }
        return true;
    }
    let mut count = 0;
    for allergen in &allergensLists[line] {
        if IngredientsAllergensList.contains_key(allergen) {
            continue;
        }
        count += 1;
    }
    let mut res = false;
    for allergen in &allergensLists[line] {
        if !IngredientsAllergensList.contains_key(allergen) {
            continue;
        }
        if !ingerdientsLists[line].contains(IngredientsAllergensList[allergen]) {
            return false;
        }
    }
    if count == 0 {
        return guessWork(
            ingerdientsLists,
            allergensLists,
            suspiciousIngredients,
            AllergensIngredientsList,
            IngredientsAllergensList,
            line + 1,
        );
    }
    for allergen in &allergensLists[line] {
        if IngredientsAllergensList.contains_key(allergen) {
            continue;
        }
        for ingredient in &ingerdientsLists[line] {
            if suspiciousIngredients.contains(ingredient) || AllergensIngredientsList.contains_key(ingredient) {
                continue;
            }
            AllergensIngredientsList.insert(ingredient, allergen);
            IngredientsAllergensList.insert(allergen, ingredient);
            // println!("insert {} {}", ingredient, allergen);
            let found = guessWork(
                ingerdientsLists,
                allergensLists,
                suspiciousIngredients,
                AllergensIngredientsList,
                IngredientsAllergensList,
                line
            );
            if found {
                return true;
            }
            // println!("removed {} {}", ingredient, allergen);
            AllergensIngredientsList.remove(ingredient);
            IngredientsAllergensList.remove(allergen);
        }
        return false;
    }
    return true;
}
