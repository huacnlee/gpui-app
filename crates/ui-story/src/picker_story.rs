use gpui::{
    actions, div, prelude::FluentBuilder as _, px, Div, Empty, Entity, InteractiveElement as _,
    IntoElement, ParentElement, Render, SharedString, Styled, Task, View, ViewContext,
    VisualContext as _, WeakView, WindowContext,
};

use ui::{
    button::{Button, ButtonStyle},
    h_flex,
    label::Label,
    list::ListItem,
    picker::{Picker, PickerDelegate},
    switch::{LabelSide, Switch},
    theme::{ActiveTheme, Colorize},
    v_flex, Clickable as _, Disableable as _, Selectable, StyledExt,
};

use super::story_case;

actions!(picker_story, [DismissPicker]);

pub struct ListItemDeletegate {
    story: WeakView<PickerStory>,
    selected_index: usize,
    matches: Vec<String>,
}

impl PickerDelegate for ListItemDeletegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, index: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = index
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        if let Some(item) = self.matches.get(ix) {
            let list_item = ListItem::new(("item", ix))
                .check_icon(ui::IconName::Check)
                .selected(selected)
                .py_1()
                .px_3()
                .child(item.clone());
            Some(list_item)
        } else {
            None
        }
    }

    fn update_matches(
        &mut self,
        query: &str,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        if let Some(story) = self.story.upgrade() {
            let matched_items = story
                .read(cx)
                .items
                .iter()
                .filter(|item| item.contains(query))
                .cloned()
                .collect();

            self.matches = matched_items;
            cx.notify();
        }

        Task::ready(())
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(story) = self.story.upgrade() {
            cx.update_view(&story, |story, cx| {
                story.open = false;
                cx.notify();
            });
        }
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(story) = self.story.upgrade() {
            cx.update_view(&story, |story, cx| {
                if let Some(item) = self.matches.get(self.selected_index) {
                    story.selected_value = Some(item.clone());
                }
                story.open = false;
                cx.notify();
            });
        }
    }
}

pub struct PickerStory {
    picker: View<Picker<ListItemDeletegate>>,
    open: bool,
    items: Vec<String>,
    selected_value: Option<String>,
}

impl PickerStory {
    pub(crate) fn new(cx: &mut ViewContext<Self>) -> Self {
        let items: Vec<String> = [
            "Baguette (France)",
            "Baklava (Turkey)",
            "Beef Wellington (UK)",
            "Biryani (India)",
            "Borscht (Ukraine)",
            "Bratwurst (Germany)",
            "Bulgogi (Korea)",
            "Burrito (USA)",
            "Ceviche (Peru)",
            "Chicken Tikka Masala (India)",
            "Churrasco (Brazil)",
            "Couscous (North Africa)",
            "Croissant (France)",
            "Dim Sum (China)",
            "Empanada (Argentina)",
            "Fajitas (Mexico)",
            "Falafel (Middle East)",
            "Feijoada (Brazil)",
            "Fish and Chips (UK)",
            "Fondue (Switzerland)",
            "Goulash (Hungary)",
            "Haggis (Scotland)",
            "Kebab (Middle East)",
            "Kimchi (Korea)",
            "Lasagna (Italy)",
            "Maple Syrup Pancakes (Canada)",
            "Moussaka (Greece)",
            "Pad Thai (Thailand)",
            "Paella (Spain)",
            "Pancakes (USA)",
            "Pasta Carbonara (Italy)",
            "Pavlova (Australia)",
            "Peking Duck (China)",
            "Pho (Vietnam)",
            "Pierogi (Poland)",
            "Pizza (Italy)",
            "Poutine (Canada)",
            "Pretzel (Germany)",
            "Ramen (Japan)",
            "Rendang (Indonesia)",
            "Sashimi (Japan)",
            "Satay (Indonesia)",
            "Shepherd's Pie (Ireland)",
            "Sushi (Japan)",
            "Tacos (Mexico)",
            "Tandoori Chicken (India)",
            "Tortilla (Spain)",
            "Tzatziki (Greece)",
            "Wiener Schnitzel (Austria)",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let story = cx.view().downgrade();
        let picker = cx.new_view(|cx| {
            let mut picker = Picker::uniform_list(
                ListItemDeletegate {
                    story,
                    selected_index: 0,
                    matches: items.clone(),
                },
                cx,
            )
            .modal(true)
            .max_height(Some(px(350.0).into()));
            picker.focus(cx);
            picker.set_query("c", cx);
            picker
        });

        Self {
            items,
            picker,
            open: false,
            selected_value: None,
        }
    }
}

impl Render for PickerStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        story_case("Picker", "Picker is a list of items that can be selected.")
            .child(
                v_flex().items_start().child(
                    Button::new("show-picker", "Show Picker...")
                        .style(ButtonStyle::Primary)
                        .on_click(cx.listener(|this, _, cx| {
                            this.open = !this.open;
                            cx.notify();
                        })),
                ),
            )
            .when_some(self.selected_value.clone(), |this, selected_value| {
                this.child("Selected: ").child(Label::new(selected_value))
            })
            .when(self.open, |this| {
                this.child(
                    div().absolute().size_full().top_0().left_0().child(
                        v_flex()
                            // .h(px(0.0))
                            .top_10()
                            .flex()
                            .flex_col()
                            .items_center()
                            .track_focus(&self.picker.focus_handle(cx))
                            .child(h_flex().w(px(450.)).occlude().child(self.picker.clone())),
                    ),
                )
            })
    }
}
