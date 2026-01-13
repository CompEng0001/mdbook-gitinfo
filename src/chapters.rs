use mdbook_preprocessor::book::{BookItem,Chapter};

pub fn decorate_chapters<F>(item: &mut BookItem, decorate: &F)
where
    F: Fn(&mut Chapter),
{
    if let BookItem::Chapter(ch) = item {
        decorate(ch);
        for sub in &mut ch.sub_items {
            decorate_chapters(sub, decorate);
        }
    }
}
