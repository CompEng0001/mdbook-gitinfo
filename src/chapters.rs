use mdbook::book::BookItem;

pub fn decorate_chapters<F>(item: &mut BookItem, decorate: &F)
where
    F: Fn(&mut mdbook::book::Chapter),
{
    if let BookItem::Chapter(ch) = item {
        decorate(ch);
        for sub in &mut ch.sub_items {
            decorate_chapters(sub, decorate);
        }
    }
}
