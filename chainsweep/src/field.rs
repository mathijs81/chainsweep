use alloc::vec::Vec;

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
}

// We encode each field in 4 bits, so make sure to use at most 0-15
// 0-8 = opened, number of bugs around
// 9 = bug opened by player (i.e. game lost)
// 10 = unopened empty space
// 11 = bug, not opened yet

pub const BUG: u8 = 9;
pub const UNOPENED: u8 = 10;
pub const UNOPENED_BUGFREE: u8 = 11;
pub const UNOPENED_BUG: u8 = 12;

pub fn is_open(data: u8) -> bool {
    data < UNOPENED
}

pub fn next_rand(seed: u64) -> u64 {
    let mut x = seed;
    x = ((x + 1337) * 16807) % 0x7FFFFFFF;
    x
}

impl GameData {
    pub fn new(width: u8, height: u8, data: Vec<u8>) -> Self {
        let mut fields = Vec::new();
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
                        if field.data == BUG || field.data == UNOPENED_BUG {
                            bugs += 1;
                        }
                        if add_impact {                            
                            field.impacted_by.push(index);
                        }
                        // end of lifetime 'field'
                        if add_impact {
                            fields[index as usize].impacts.push(other_index);
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
        }
    }

    pub fn get(&self, x: u8, y: u8) -> &Field {
        &self.fields[(y as usize)*(self.width as usize) + (x as usize)]
    }

    pub fn set_data(&mut self, x: u8, y: u8, data: u8) {
        self.fields[(y as usize)*(self.width as usize) + (x as usize)].data = data;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let data = GameData::new(4, 4, vec![
            10, 9, 10, 10,
            9, 10, 10, 10,
            10, 10, 10, 10,
            9, 9, 9, 10
        ]);

        let adjacent = data.fields.iter().map(|field| field.adjacent_bugs).collect::<Vec<u8>>();
        assert_eq!(adjacent, vec![
            2, 1, 1, 0, 
            1, 2, 1, 0, 
            3, 4, 2, 1, 
            1, 2, 1, 1]);
    }
}