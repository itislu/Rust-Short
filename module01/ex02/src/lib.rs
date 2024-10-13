#[allow(dead_code)]
const fn color_name(color: &[u8; 3]) -> &'static str {
    match (color[0], color[1], color[2]) {
        (0, 0, 0) => "pure black",
        (255, 255, 255) => "pure white",
        (255, 0, 0) => "pure red",
        (0, 255, 0) => "pure green",
        (0, 0, 255) => "pure blue",
        (128, 128, 128) => "perfect grey",
        (0..31, 0..31, 0..31) => "almost black",
        (129..=255, 0..=127, 0..=127) => "redish",
        (0..=127, 129..=255, 0..=127) => "greenish",
        (0..=127, 0..=127, 129..=255) => "blueish",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetimes() {
        let name_of_the_best_color;

        {
            let the_best_color = [42, 42, 42];
            name_of_the_best_color = color_name(&the_best_color);
        }

        assert_eq!(name_of_the_best_color, "unknown");
    }

    #[test]
    fn test_pure_colors() {
        assert_eq!(color_name(&[0, 0, 0]), "pure black");
        assert_eq!(color_name(&[255, 255, 255]), "pure white");
        assert_eq!(color_name(&[255, 0, 0]), "pure red");
        assert_eq!(color_name(&[0, 255, 0]), "pure green");
        assert_eq!(color_name(&[0, 0, 255]), "pure blue");
        assert_eq!(color_name(&[128, 128, 128]), "perfect grey");
    }

    #[test]
    fn test_almost_black() {
        assert_eq!(color_name(&[30, 30, 30]), "almost black");
        assert_eq!(color_name(&[0, 0, 0]), "pure black");
    }

    #[test]
    fn test_redish() {
        assert_eq!(color_name(&[130, 0, 0]), "redish");
        assert_eq!(color_name(&[255, 127, 127]), "redish");
    }

    #[test]
    fn test_greenish() {
        assert_eq!(color_name(&[0, 130, 0]), "greenish");
        assert_eq!(color_name(&[127, 255, 127]), "greenish");
    }

    #[test]
    fn test_blueish() {
        assert_eq!(color_name(&[0, 0, 130]), "blueish");
        assert_eq!(color_name(&[127, 127, 255]), "blueish");
    }

    #[test]
    fn test_unknown() {
        assert_eq!(color_name(&[200, 200, 200]), "unknown");
    }
}
