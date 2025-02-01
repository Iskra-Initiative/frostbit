struct Sidebar {
    width: u32,
    height: u32,
    items: Vec<String>,
}

enum SidebarMessage {
    ItemSelected(usize),
}

impl Sidebar {
    fn new() -> Sidebar {
        Sidebar {
            width: 200,
            height: 600,
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
        }
    }

    fn update(&mut self, message: SidebarMessage) {
        match message {
            SidebarMessage::ItemSelected(index) => {
                println!("Item selected: {}", self.items[index]);
            }
        }
    }

    fn view(&self) -> Element<SidebarMessage> {
        let items = self.items.iter().enumerate().fold(
            Column::new().spacing(10),
            |column, (index, item)| {
                column.push(
                    Button::new(
                        Text::new(item)
                            .horizontal_alignment(HorizontalAlignment::Center)
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    .on_press(SidebarMessage::ItemSelected(index)),
                )
            },
        );

        Container::new(items)
            .width(Length::Units(self.width))
            .height(Length::Units(self.height))
            .padding(10)
            .into()
    }
}
