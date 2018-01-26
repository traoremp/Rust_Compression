use std::collections::HashMap;

pub fn hash_map_reducer(hash_maps: Vec<HashMap<u8, usize>>) -> HashMap<u8, usize> {
  let mut result: HashMap<u8, usize> = HashMap::new();

  for hash_map in hash_maps {
    for (key, val) in hash_map.iter() {
      let count = result.entry(*key).or_insert(0);
      *count += *val;
    }
  }

  result
}

pub fn string_to_substrings(input: &[u8], subarray_count: usize) -> Vec<&[u8]> {
  let input_length = input.len();
  let length_per_thread = input_length / subarray_count;

  let mut subarray_offsets = Vec::with_capacity(subarray_count);

  for n in 1..subarray_count {
    let offset = n * length_per_thread;
    subarray_offsets.push(offset);
  }
  subarray_offsets.push(input_length);

  let mut subarrays = vec![];
  let mut from = 0;

  for subarray_offset in subarray_offsets {
    subarrays.push(&input[from..subarray_offset]);
    from = subarray_offset;
  }

  subarrays
}
