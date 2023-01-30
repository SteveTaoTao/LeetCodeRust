pub fn complex_longest_palindrome(s: String) -> String {
    // Manacher's algorithm
    let mut bytes = Vec::new();
    for b in s.bytes() {
        bytes.push(b'|');
        bytes.push(b);
    }
    bytes.push(b'|');
    let len = bytes.len();
    let mut radii = vec![0; len];
    let mut center = 0;
    let mut radius = 0;

    while center < len {
        while center >= radius + 1
            && center + (radius + 1) < len
            && bytes[center - (radius + 1)] == bytes[center + (radius + 1)]
        {
            radius += 1;
        }

        radii[center] = radius;

        let old_center = center;
        let old_radius = radius;
        center += 1;
        radius = 0;
        while center <= old_center + old_radius {
            let mirror_center = old_center - (center - old_center);
            let mirror_radius = old_center + old_radius - center;
            if radii[mirror_center] < mirror_radius {
                radii[center] = radii[mirror_center];
                center += 1;
            } else if radii[mirror_center] > mirror_radius {
                radii[center] = mirror_radius;
                center += 1;
            } else {
                radius = mirror_radius;
                break;
            }
        }
    }

    //recreate string
    let mut max_radius = 0;
    let mut max_center = 0;
    for (i, r) in radii.iter().enumerate() {
        if *r > max_radius {
            max_radius = *r;
            max_center = i;
        }
    }

    let mut ans = String::new();
    for i in (max_center - max_radius + 1..max_center + max_radius).step_by(2) {
        ans.push(bytes[i] as char);
    }
    ans
}

pub fn longest_palindrome(s: String) -> String {
    fn is_palidrone(s: &[u8]) -> bool {
        // Iterate left to right along with iterating from right to left,
        // make sure each spot is the same.
        // Returns false once the left is not equal to the right
        s.iter().zip(s.iter().rev()).all(|(l, r)| l == r)
    }

    for size in (1..=s.len()).rev() {
        match s
            .as_bytes()
            .windows(size)
            .find(|substr| is_palidrone(substr))
        {
            Some(pal) => return String::from_utf8(pal.to_vec()).unwrap(),
            None => continue,
        }
    }
    // No palidrone found
    String::from("")
}

fn main() {
    let s_input = String::from("32lkkl");
    let s_easy = longest_palindrome(s_input);
    println!("easy output is {}", s_easy);

    let s_test = String::from("32lkkl");

    print!("s_test is {:?} \n", s_test.as_bytes());
    print!("s_test is {:?} \n", s_test.as_bytes());

    print!(
        "s_test.iter().zip(s_test.iter().rev()) is : {:?}",
        s_test.as_bytes().iter().zip(s_test.as_bytes().iter().rev())
    );
}
