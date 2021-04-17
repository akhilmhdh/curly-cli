use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout as TuiLayout, Rect},
    widgets::{Block, Borders, Widget},
    Frame,
};

struct BlockAtom<IdType> {
    id: IdType,
    block: Box<dyn Widget>,
}

enum Atom<T> {
    BlockAtom(Box<BlockAtom<T>>),
    Layout(Layout<T>),
}

pub struct Layout<T> {
    component: Vec<Rect>,
    atoms: Vec<Atom<T>>,
}

impl<T> Layout<T> {
    pub fn layout_builder<B: Backend>(layout: &Layout<T>, f: &mut Frame<B>, size: Rect) {
        let block_data = Box::new(BlockAtom {
            id: "search",
            block: Box::new(Block::default().title("Body").borders(Borders::ALL)),
        });
        Layout {
            component: TuiLayout::default()
                .direction(Direction::Vertical)
                .horizontal_margin(1)
                .constraints([Constraint::Length(3), Constraint::Percentage(80)].as_ref())
                .split(size),
            atoms: vec![Atom::BlockAtom(block_data)],
        };
    }
}
