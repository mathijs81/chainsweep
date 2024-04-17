use alloc::{vec::Vec, string::String};
use fastrand::Rng;

#[derive(Clone)]
pub struct Field {
    pub data: u8,
    impacted_by: Vec<u8>,
    impacts: Vec<u8>,
    pub adjacent_bugs: u8,
}

#[derive(Clone)]
pub struct GameData {
    fields: Vec<Field>,
    width: u8,
    height: u8,
    pub num_open: u8,
}

// We encode each field in 4 bits, so make sure to use at most 0-15
// 0-8 = opened, number of bugs around
// 9 = bug opened by player (i.e. game lost)
// 10 = unopened empty space
// 11 = bug, not opened yet

pub const BUG: u8 = 9;
pub const UNOPENED: u8 = 10;
pub const UNOPENED_BUGFREE: u8 = 11;

pub fn is_open(data: u8) -> bool {
    data < UNOPENED
}

impl GameData {
    pub fn new(width: u8, height: u8, data: Vec<u8>) -> Self {
        let mut fields = Vec::new();
        let mut num_open = 0;
        for y in 0..height {
            for x in 0..width {
                let field = Field { data: data[(y as usize)*(width as usize) + (x as usize)], impacted_by: Vec::new(), impacts: Vec::new(), adjacent_bugs: 0u8 };
                fields.push(field);
            }
        }

        for y in 0..height {
            for x in 0..width {
                let index = y * width + x;
                let add_impact = fields[index as usize].data < BUG;
                if add_impact {
                    num_open += 1;
                }

                let mut bugs = 0u8;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let x = x as i8 + i;
                        let y = y as i8 + j;
                        if x < 0 || x as u8 >= width || y < 0 || y as u8 >= height {
                            continue;
                        }
                        let other_index = (y as u8) * width + (x as u8);
                        let field = &mut fields[other_index as usize];
                        let add_impact = add_impact && field.data == UNOPENED;
                        if field.data == BUG {
                            bugs += 1;
                        }
                        if add_impact {                            
                            field.impacts.push(index);
                        }
                        // end of lifetime 'field'
                        if add_impact {
                            fields[index as usize].impacted_by.push(other_index);
                        }
                    }
                }
                fields[index as usize].adjacent_bugs = bugs;
            }
        }

        GameData {
            fields,
            width,
            height,
            num_open,
        }
    }

    pub fn get(&self, x: u8, y: u8) -> &Field {
        &self.fields[(y as usize)*(self.width as usize) + (x as usize)]
    }

    pub fn set_data(&mut self, x: u8, y: u8, data: u8) {
        self.fields[(y as usize)*(self.width as usize) + (x as usize)].data = data;
    }


    fn surrounding_fields(&self, x: u8, y: u8) -> Vec<usize> {
        let mut result = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let x = x as i8 + i;
                let y = y as i8 + j;
                if x < 0 || x as u8 >= self.width || y < 0 || y as u8 >= self.height {
                    continue;
                }
                let index = (y as usize)*(self.width as usize) + (x as usize);
                result.push(index);
            }
        }
        result
    }

    fn try_field(&mut self, handle_fields: &Vec<(u8, u8)>, proposed_bugs: u64, is_bug: bool) -> Result<(), ()> {
        let (x, y) = handle_fields[0];
        // Update all fields around and check that they're OK
        let my_index = y * self.width + x;
        assert!(self.fields[my_index as usize].data == UNOPENED);
        let mut all_ok = true;
        if is_bug {
            self.fields[my_index as usize].data = BUG;
        } else {
            self.fields[my_index as usize].data = UNOPENED_BUGFREE;
        }
        self.surrounding_fields(x, y).into_iter().for_each(|index| {
            let field = &mut self.fields[index];
            if is_bug {
                field.adjacent_bugs += 1;
            }
            // remove this index from the impacted_by
            field.impacted_by.retain(|&x| x != my_index);
            all_ok = all_ok && (field.data > 8 || (field.data >= field.adjacent_bugs && field.data <= field.adjacent_bugs + (field.impacted_by.len() as u8)));
        });

        if all_ok {
            let mut new_fields = handle_fields[1..].to_vec();
            // Add not-yet existing ones from impacted fields
            for index in self.fields[my_index as usize].impacts.iter() {
                for target in self.fields[*index as usize].impacted_by.iter() {
                    let (x, y) = (target % self.width, target / self.width);
                    if !new_fields.contains(&(x, y)) {
                        new_fields.push((x, y));
                    }
                }
            }

            if self.update_field(&new_fields, proposed_bugs).is_ok() {
                return Ok(());
            }
        }

        // Undo all changes
        self.surrounding_fields(x, y).into_iter().for_each(|index| {
            let field = &mut self.fields[index];
            if is_bug {
                field.adjacent_bugs -= 1;
            }
            field.impacted_by.push(my_index);
        });
        self.fields[my_index as usize].data = UNOPENED;

        Err(())
    }

    fn update_field(&mut self, fields: &Vec<(u8, u8)>, proposed_bugs: u64) -> Result<(), ()> {
        if fields.is_empty() {
            return Ok(());
        }
        let proposed_bug = ((proposed_bugs >> (fields[0].0 * self.height + fields[0].1)) & 1) == 1;
        if let Err(()) = self.try_field(fields, proposed_bugs, proposed_bug) {
            self.try_field(fields, proposed_bugs, !proposed_bug)
        } else {
            Ok(())
        }
    }

    // Fill in all unopened fields with either BUG or UNOPENED_BUGFREE so that the already opened 
    // fields are consistent with the number of adjacent bugs
    // Note: this is generating a mostly randomly sampled solution but not perfectly, especially when 
    // bug_percentage is close to 0 or 100.
    // We 'propose' a bug yes/no for every field and then try to 'fix up' the game to be consistent with
    // the opened fields. This fix-up process is not randomized and so if the proposals are not very random
    // (e.g. all 1 or all 0), the result will not be very random either.
    pub fn fill_in(&self, rand: u64, bug_percentage: u8) -> GameData {
        let mut r = Rng::with_seed(rand);
        // bit in bugs is 1 if we will start trying with a bug in that field, 0 if we start with no bug
        let mut bugs = 0u64;
        for _ in 0..self.fields.len() {
            let p = r.u8(0..100);
            bugs <<= 1;
            if p < bug_percentage {
                bugs += 1;
            }
        }

        let mut result = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let field = &result.fields[(y as usize)*(self.width as usize) + (x as usize)];
                if field.data == UNOPENED {
                    let mut process_fields = Vec::new();
                    process_fields.push((x,y));
                    if let Err(()) = result.update_field(&process_fields, bugs) {
                        panic!("Could not fill in field {} {}", x, y);
                    }
                }
            }
        }
        result
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let field = &self.fields[(y as usize)*(self.width as usize) + (x as usize)];
                if field.data == BUG {
                    result.push('X');
                } else {
                    result.push_str(".");
                }
            }
            result.push_str("\n");
        }
        result
    }

}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let X = BUG;
        let o = UNOPENED;
        let data = GameData::new(4, 4, vec![
            o, X, o, o,
            X, o, o, o,
            o, o, o, o,
            X, X, X, o
        ]);

        let adjacent = data.fields.iter().map(|field| field.adjacent_bugs).collect::<Vec<u8>>();
        assert_eq!(adjacent, vec![
            2, 1, 1, 0, 
            1, 2, 1, 0, 
            3, 4, 2, 1, 
            1, 2, 1, 1]);
    }

    #[test]
    fn test_impact() {
        let X = BUG;
        let o = UNOPENED;
        let data = GameData::new(
            4,4, vec![
            X, X, o, o,
            X, 4, 1, o,
            o, o, o, o,
            X, X, X, 2
        ]);

        assert_eq!(data.fields[0].impacts, vec![]);
        assert_eq!(data.fields[0].impacted_by, vec![]);

        assert_eq!(data.fields[2].impacts, vec![5, 6]);
        assert_eq!(data.fields[2].impacted_by, vec![]);

        assert_eq!(data.fields[5].impacts, vec![]);
        assert_eq!(data.fields[5].impacted_by, vec![8, 9, 2, 10]);

        assert_eq!(data.fields[12].impacts, vec![]);
        assert_eq!(data.fields[12].impacted_by, vec![]);
    }

    #[test]
    fn test_generate() {
        let X = BUG;
        let o = UNOPENED;
        let data = GameData::new(
            4,4, vec![
            X, X, o, o,
            X, 4, 1, o,
            o, o, o, o,
            X, X, X, 1
        ]);

        let z = UNOPENED_BUGFREE;

        let filled_in = data.fill_in(63515, 20);

        assert_eq!(filled_in.fields.iter().map(|field| field.data).collect::<Vec<u8>>(), vec![
            X, X, z, z,
            X, 4, 1, z,
            X, z, z, z,
            X, X, X, 1
        ]);

        let filled_in = GameData::new(5,5, vec![o; 25]).fill_in(666132615, 20);
        assert_eq!(filled_in.fields.iter().map(|field| field.data).collect::<Vec<u8>>(), vec![
            z, X, z, X, z,
            z, z, z, z, z,
            z, z, z, z, z,
            z, X, X, z, z,
            z, X, z, z, z
        ]);

        let filled_in = GameData::new(5,5, vec![o; 25]).fill_in(666132615, 80);
        assert_eq!(filled_in.fields.iter().map(|field| field.data).collect::<Vec<u8>>(), vec![
            X, X, z, X, X,
            X, X, X, X, X,
            z, X, X, X, z,
            X, X, X, X, X,
            X, X, X, X, z,
        ]);

        let data = GameData::new(
            4,4, vec![
            o, o, o, o,
            o, 4, 1, o,
            o, o, o, o,
            o, o, o, 1
        ]);

        let filled_in = data.fill_in(666132615, 25);
        assert_eq!(filled_in.fields.iter().map(|field| field.data).collect::<Vec<u8>>(), vec![
            X, X, z, z,
            X, 4, 1, z,
            X, z, z, z,
            z, z, X, 1
        ]);

        assert_eq!(data.fill_in(13371, 25).fields.iter().map(|field| field.data).collect::<Vec<u8>>(), vec![
            X, z, X, z,
            X, 4, 1, z,
            X, z, z, z,
            z, X, X, 1
        ]);
    }
  

    #[test]
    fn test_generate_at_start_bugfree() {
        let X = BUG;
        let o = UNOPENED;
        let z = UNOPENED_BUGFREE;
        let data = GameData::new(
            4,4, vec![
            o, o, o, o,
            o, o, o, o,
            o, o, z, o,
            o, o, o, o,
        ]);
        assert_eq!(data.fill_in(13371, 50).fields.iter().map(|field| field.data).collect::<Vec<u8>>(), vec![
            z, X, X, z,
            X, z, z, z,
            z, X, z, z,
            z, X, X, X,
        ]);
        assert_eq!(data.fill_in(13371, 100).fields.iter().map(|field| field.data).collect::<Vec<u8>>(), vec![
            X, X, X, X,
            X, X, X, X,
            X, X, z, X,
            X, X, X, X,
        ]);

    }
}