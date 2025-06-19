use bitcoin::hashes::{sha256d, Hash};

pub fn build_merkle_tree(hashes: Vec<[u8; 32]>) -> Vec<Vec<[u8; 32]>> {
    let mut levels = Vec::new();
    levels.push(hashes);
    while levels.last().unwrap().len() > 1 {
        let current = levels.last().unwrap();
        let mut next_level = Vec::new();
        for i in (0..current.len()).step_by(2) {
            let left = current[i];
            let right = current.get(i + 1).unwrap_or(&left);
            next_level.push(double_sha256(&[left, *right].concat()).to_byte_array());
        }
        levels.push(next_level);
    }
    levels
}

pub fn get_merkle_path(levels: Vec<Vec<[u8; 32]>>, tx_index: usize) -> Vec<[u8; 32]> {
    let mut path = Vec::new();
    let mut idx = tx_index;
    for level in 0..levels.len() - 1 {
        let level_hashes = levels[level].clone();
        let mut sibling_idx;
        if idx % 2 == 0 {
            sibling_idx = idx + 1;
            if sibling_idx >= level_hashes.len() {
                sibling_idx = idx;
            }
        } else {
            sibling_idx = idx - 1;
        }
        path.push(level_hashes[sibling_idx]);
        idx /= 2;
    }
    path
}

pub fn double_sha256(data: &[u8]) -> sha256d::Hash {
    sha256d::Hash::hash(data)
}

mod tests {
    use bitcoin::hex::DisplayHex;

    #[test]
    fn test_double_sha256() {
        use super::*;
        let three = [0x03];
        let four = [0x04];

        let three_four = double_sha256(&[three, four].concat());
        assert_eq!(
            three_four,
            sha256d::Hash::from_byte_array([
                0x2d, 0x53, 0x28, 0xbf, 0x2b, 0x93, 0xb1, 0xc4, 0xf8, 0x64, 0x70, 0x69, 0xb2, 0x99,
                0x0a, 0x73, 0x31, 0x5a, 0x55, 0x4d, 0x99, 0x28, 0x22, 0xde, 0xb0, 0xdc, 0x3a, 0x67,
                0x4d, 0x07, 0xbf, 0xc9
            ])
        );
    }

    #[test]
    fn test_odd_number_of_hashes() {
        use super::*;
        let one = [0x01; 32];
        let two = [0x02; 32];
        let three = [0x03; 32];
        let four = [0x04; 32];
        let five = [0x05; 32];

        let hashes = vec![one, two, three, four, five];
        let levels = build_merkle_tree(hashes);
        let path = get_merkle_path(levels, 0);

        let expected = vec![
            "0202020202020202020202020202020202020202020202020202020202020202".to_string(),
            "c3a6b10a4ddcdf9561d3c6f44a09c6ecd380486275674e74f5503429230b7cf2".to_string(),
            "b93bfcfb094ddf2b2ef0feb482bbd018bf1ef5c08bd9620727d212fcb11311dd".to_string(),
        ];

        for (i, p) in path.iter().enumerate() {
            assert_eq!(p.to_lower_hex_string(), expected[i]);
        }
    }
}
