use std::str::FromStr;

type Error = Box<std::error::Error>;

fn main() -> Result<(), Error> {
    let seed = "37";
    let recipe_list: RecipeList = seed.parse()?;

    let mut lab = Lab::new(recipe_list);
    // println!("{:?}", lab.get_postfix_after(5, 10));
    // println!("{:?}", lab.get_postfix_after(18, 10));
    // println!("{:?}", lab.get_postfix_after(2018, 10));
    println!("{:?}", lab.get_postfix_after(540561, 10));

    let recipe_list: RecipeList = seed.parse()?;
    let mut lab = Lab::new(recipe_list);
    let index = lab.get_match_index(vec![5, 4, 0, 5, 6, 1]);
    // let index = lab.get_match_index(vec![5, 1, 5, 8, 9]);
    println!("{:?}", index); // 20254833
    println!("{:?}", lab.get_postfix_after(index, 6)); // 540561

    Ok(())
}

type RecipeIndex = usize;

#[derive(Debug)]
struct Lab {
    recipe_list: RecipeList,
    elves: Vec<RecipeIndex>
}

impl Lab {
    fn new(recipe_list: RecipeList) -> Lab {
        Lab {
            elves: vec![0, 1],
            recipe_list
        }
    }

    fn get_match_index(&mut self, needle: Vec<u8>) -> usize {
        let mut matches = 0;
        let mut index = None;
        while matches < needle.len() {
            let mut idx = self.recipe_list.len();
            let new_scores = self.generate_new();
            for &n in new_scores {
                if n == needle[matches] {
                    matches += 1;
                    match index {
                        Some(_) => {},
                        None => index = Some(idx)
                    }
                    if matches == needle.len() {
                        break;
                    }
                } else {
                    idx += 1;
                    matches = 0;
                    index = None;
                }
            }
            self.move_elves();
        }
        index.unwrap() 
    }

    fn move_elves(&mut self) {
        self.elves[0] = self.recipe_list.get_new_index(self.elves[0]);
        self.elves[1] = self.recipe_list.get_new_index(self.elves[1]);
    }

    fn generate_new(&mut self) -> &[u8] {
        let news = self.recipe_list.generate_new(self.elves[0], self.elves[1]);
        news
    }

    fn len(&self) -> usize {
        self.recipe_list.len()
    }

    fn get_postfix_after(&mut self, offset: usize, count: usize) -> String {
        while self.len() < offset + 10 {
            let _ = self.generate_new();
            self.move_elves();
        }
        self.recipe_list
            .get_list(offset, count)
            .iter()
            .fold(String::new(), |mut buf, d| {
                buf.push_str(&d.to_string());
                buf
            })
    }
}

type Score = u8;

#[derive(Debug)]
struct RecipeList {
    recipes: Vec<Score>
}

impl RecipeList {
    fn get_list(&self, start: usize, count: usize) -> &[Score] {
        let end = start + count;
        &self.recipes[start..end]
    }

    fn len(&self) -> usize {
        self.recipes.len()
    }

    fn get_new_index(&self, origin: RecipeIndex) -> RecipeIndex {
        let offset = self.get(origin) as usize + 1usize;
        (origin + offset) % self.recipes.len()
    }

    fn get(&self, index: RecipeIndex) -> Score {
        self.recipes[index]
    }

    fn generate_new(&mut self, a: RecipeIndex, b: RecipeIndex) -> &[Score] {
        let sum = self.recipes[a] + self.recipes[b];
        let mut count = 0;
        let n = sum % 10;
        if sum >= 10 {
            let m = (sum - n) / 10;
            self.recipes.push(m);
            count += 1;
        }
        self.recipes.push(n);
        count += 1;

        &self.recipes[(self.recipes.len() - count)..]
    }
}

impl FromStr for RecipeList {
    type Err = Error;

    fn from_str(s: &str) -> Result<RecipeList, Error> {
        let scores = s.chars().try_fold(vec![], |mut acc, c| {
            match c.to_digit(10) {
                Some(d) => {
                    acc.push(d as Score);
                    Ok(acc)
                },
                None => return Err(Box::<std::error::Error>::from("Could not parse digit"))
            }
        })?;
        Ok(RecipeList {
            recipes: scores
        })
    }
}
