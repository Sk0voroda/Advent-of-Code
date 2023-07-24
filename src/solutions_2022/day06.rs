pub fn tuning_troble() -> usize {
    let file_data =
        std::fs::read_to_string("inputs/2022/06.txt").expect("puzzle for day 6 file is missing");

    let mut marker = String::new();
    let mut marker_end = 0;

    for i in 0..file_data.len() {
        for ch in file_data[i..i + 4].chars() {
            if marker.contains(ch) {
                marker.clear();
                break;
            }

            marker.push(ch);
        }

        if marker.len() == 4 {
            marker_end = i + 4;
            break;
        }
    }

    marker_end
}
