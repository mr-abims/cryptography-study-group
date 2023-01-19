use typenum::U3;

use crate::GenArrayWrapper;

#[test]
fn test_gen_array_wrapper() {
    let wrapper = GenArrayWrapper::<u32, U3>::new(generic_array::arr![u32; 1, 2, 3]);

    assert_eq!(wrapper.len(), 3);
    assert_eq!(wrapper.get_index(0).unwrap(), &1);
    assert!(!wrapper.is_empty());
}
