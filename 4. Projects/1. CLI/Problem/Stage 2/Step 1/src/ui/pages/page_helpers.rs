use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    // use the truncate_ellipse function from the ellipse crate
    if width == 0 {
        return "".to_owned();
    } else if width <= 3 {
        return ".".repeat(width);
    }
    let mut truncated = String::new();
    if text.len() > width {
        truncated = text.truncate_ellipse(width-3).to_string();
    } else {
        truncated = text.to_string();
    }
    let mut padding  = 0;
    if width >= truncated.len() {
        padding = width - truncated.len();
    }

    format!("{}{}", truncated, " ".repeat(padding))
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;
        

        assert_eq!(get_column_string(text4, width), "".to_owned());
        
        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());
        
        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());
        
        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());
        
        let width = 4;
        
        assert_eq!(get_column_string(text4, width), "t...".to_owned());
        
        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}