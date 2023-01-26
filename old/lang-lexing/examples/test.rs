use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let iter = "test.mig + 2+2.3 😀sds \n test  ".graphemes(true);

    for next in iter {
        println!("{:?}", next);
    }
}
