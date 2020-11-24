use indicatif;
fn main() {
    let progress_bar = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        progress_bar.println(format!("[+] finished #{}", i));
        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("done");
}
