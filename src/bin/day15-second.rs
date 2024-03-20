use aoc_2023::day15;
use std::collections::HashMap;
use std::fs;

type Box = u8;
type Lens = u8;
type Label = String;
type FocusingPower = u64;
enum Step {
    Remove(RemoveInfo),
    Put(PutInfo),
}
struct RemoveInfo {
    label: Label,
}
struct PutInfo {
    label: Label,
    focal_length: Lens,
}
//#[derive(Debug)]
struct LensConfiguration {
    box_to_lenses: BoxToLenses,
    lens_to_box: LensToBox,
}
type BoxToLenses = HashMap<Box, Vec<(Label, Lens)>>;
type LensToBox = HashMap<Lens, Box>;

fn main() {
    let input = fs::read_to_string("data/day15/input").unwrap();
    let steps = parse_input(&input);
    let mut lens_configuration = LensConfiguration {
        box_to_lenses: HashMap::new(),
        lens_to_box: HashMap::new(),
    };
    lens_configuration.apply(steps);
    let answer = lens_configuration.focusing_power();

    println!("{answer:?}");
}

impl LensConfiguration {
    fn focusing_power(self) -> FocusingPower {
        self.box_to_lenses
            .values()
            .map(|label_to_lens| {
                label_to_lens
                    .iter()
                    .enumerate()
                    .map(|(idx, (label, focal_length))| {
                        let target_box = day15::hash(label);
                        (1 + target_box as FocusingPower)
                            * (1 + idx as FocusingPower)
                            * *focal_length as FocusingPower
                    })
                    .sum::<FocusingPower>()
            })
            .sum()
    }

    fn apply(&mut self, steps: Vec<Step>) {
        for step in steps {
            match step {
                Step::Remove(r) => self.remove(r),
                Step::Put(p) => self.put(p),
            };
        }
    }

    fn remove(&mut self, remove_info: RemoveInfo) {
        let target_box = day15::hash(&remove_info.label);

        // First find which focal length (if any) it currently refers to and unassign it
        if let Some(label_to_lens) = self.box_to_lenses.get(&target_box) {
            label_to_lens
                .iter()
                .find(|(label, _)| label == &remove_info.label)
                .and_then(|(_, focal_length)| self.lens_to_box.remove(focal_length));
        }

        // Then remove the entry from the box
        if let Some(label_to_lens) = self.box_to_lenses.get_mut(&target_box) {
            if let Some((idx, _)) = label_to_lens
                .iter()
                .enumerate()
                .find(|(_, (label, _))| label == &remove_info.label)
            {
                label_to_lens.remove(idx);
            }
        }
    }

    fn put(&mut self, put_info: PutInfo) {
        let target_box = day15::hash(&put_info.label);

        // First find which focal length (if any) it currently refers to and unassign it
        self.box_to_lenses
            .entry(target_box)
            .and_modify(|label_to_lens| {
                if let Some((_, focal_length)) = label_to_lens
                    .iter()
                    .find(|(label, _)| label == &put_info.label)
                {
                    self.lens_to_box.remove(&focal_length);
                    self.lens_to_box.insert(*focal_length, target_box);
                }
            });

        // Then add the entry to the box
        self.box_to_lenses
            .entry(target_box)
            .and_modify(|label_to_lens| {
                if let Some((idx, _)) = label_to_lens
                    .iter()
                    .enumerate()
                    .find(|(_, (label, _))| label == &put_info.label)
                {
                    label_to_lens[idx].1 = put_info.focal_length;
                } else {
                    label_to_lens.push((put_info.label.to_string(), put_info.focal_length));
                }
            })
            .or_insert(vec![(put_info.label.to_string(), put_info.focal_length)]);
    }
}

fn parse_input(input: &str) -> Vec<Step> {
    input
        .split(',')
        .map(|step| Step::parse(step.trim()))
        .collect()
}

impl Step {
    fn parse(raw_step: &str) -> Step {
        let label = raw_step.chars().take_while(|c| c.is_alphabetic()).collect();
        let focal_length = raw_step.chars().skip_while(|c| c.is_alphabetic()).nth(1);

        match focal_length {
            None => Step::Remove(RemoveInfo { label }),
            Some(f) => Step::Put(PutInfo {
                label,
                focal_length: f.to_digit(10).unwrap() as Lens,
            }),
        }
    }
}
