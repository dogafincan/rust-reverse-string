use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(bla: &str) -> String {
    bla.graphemes(true).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_korean() {
        let result = reverse("안녕하세요");
        assert_eq!(result, "요세하녕안");
    }

    #[test]
    fn it_works_for_japanese() {
        let result = reverse("こんにちは");
        assert_eq!(result, "はちにんこ");
    }

    #[test]
    fn it_works_for_mandarin() {
        let result = reverse("你好");
        assert_eq!(result, "好你");
    }

    #[test]
    fn it_works_for_thai() {
        let result = reverse("สวัสดีครับ");
        assert_eq!(result, "บรัคดีสวัส");
    }

    #[test]
    fn it_works_for_vietnamese() {
        let result = reverse("xin chào");
        assert_eq!(result, "oàhc nix");
    }
}
