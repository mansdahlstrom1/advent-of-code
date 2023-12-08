use super::custom_range::CustomRange;

fn split_section_to_map(section: &str) -> Vec<CustomRange> {
  let mut map = section.split(':').collect::<Vec<&str>>()[1]
    .split('\n')
    .filter(|n| !n.is_empty())
    .map(CustomRange::new)
    .collect::<Vec<CustomRange>>();

  map.sort_by(|a, b| a.source.start.cmp(&b.source.start));

  map
}

#[derive(Debug)]
pub struct Almanac {
  pub seeds: Vec<i64>,
  seed_to_soil_map: Vec<CustomRange>,
  soil_to_fertilizer_map: Vec<CustomRange>,
  fertilizer_to_water_map: Vec<CustomRange>,
  water_to_light_map: Vec<CustomRange>,
  light_to_temperature_map: Vec<CustomRange>,
  temperature_to_humidity_map: Vec<CustomRange>,
  humidity_to_location_map: Vec<CustomRange>,
}

impl Almanac {
  pub fn new(input: &str) -> Almanac {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<&str> = sections[0].split(':').collect();

    let seed_to_soil_map = split_section_to_map(sections[1]);
    let soil_to_fertilizer_map = split_section_to_map(sections[2]);
    let fertilizer_to_water_map = split_section_to_map(sections[3]);
    let water_to_light_map = split_section_to_map(sections[4]);
    let light_to_temperature_map = split_section_to_map(sections[5]);
    let temperature_to_humidity_map = split_section_to_map(sections[6]);
    let humidity_to_location_map = split_section_to_map(sections[7]);

    Almanac {
      seeds: seeds[1]
        .split_whitespace()
        .map(|n| n.trim().parse::<i64>().unwrap())
        .collect(),
      seed_to_soil_map,
      soil_to_fertilizer_map,
      fertilizer_to_water_map,
      water_to_light_map,
      light_to_temperature_map,
      temperature_to_humidity_map,
      humidity_to_location_map,
    }
  }

  fn get_destination_by_source(&self, source: i64, map: &[CustomRange]) -> i64 {
    for range in map {
      if range.source.contains(&source) {
        return source + range.diff;
      }
    }

    source
  }

  pub fn get_location_by_seed(&self, seed: i64) -> i64 {
    println!("Seed: {}", seed);

    let soil_location = self.get_destination_by_source(seed, &self.seed_to_soil_map);
    println!("soil_location: {}", soil_location);

    let fertilizer_location =
      self.get_destination_by_source(soil_location, &self.soil_to_fertilizer_map);
    println!("fertilizer_location: {}", fertilizer_location);

    let water_location =
      self.get_destination_by_source(fertilizer_location, &self.fertilizer_to_water_map);
    println!("water_location: {}", water_location);

    let light_location = self.get_destination_by_source(water_location, &self.water_to_light_map);
    println!("light_location: {}", light_location);

    let temperature_location =
      self.get_destination_by_source(light_location, &self.light_to_temperature_map);
    println!("temperature_location: {}", temperature_location);

    let humidity_location =
      self.get_destination_by_source(temperature_location, &self.temperature_to_humidity_map);
    println!("humidity_location: {}", humidity_location);

    let location =
      self.get_destination_by_source(humidity_location, &self.humidity_to_location_map);
    println!("location: {}", location);

    location
  }
}
