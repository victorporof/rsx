/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![feature(box_syntax)]
#![feature(proc_macro)]

extern crate rsx;
#[macro_use]
extern crate rsx_dom;
extern crate rsx_fonts;
extern crate rsx_images;
extern crate rsx_layout;
extern crate rsx_shared;
extern crate rsx_stylesheet;

use rsx::{css, load_font, load_image, rsx};
use rsx_dom::types::*;
use rsx_fonts::types::*;
use rsx_images::types::*;
use rsx_shared::traits::*;
use rsx_stylesheet::types::*;

type LayoutNode = rsx_layout::types::LayoutNode<StyleDeclarations, ComputedStyles, (), DOMText>;
type DOMTree = rsx_dom::types::DOMTree<(), StyleDeclarations, ComputedStyles, LayoutNode>;
type DOMNode = rsx_dom::types::DOMNode<(), StyleDeclarations, ComputedStyles, LayoutNode>;

#[test]
fn test_rsx_to_dom() {
    struct Props {
        visible: bool,
        menu: MenuProps
    }

    struct MenuProps {
        icon: String
    }

    fn should_do_something_fun() -> bool {
        true
    }

    fn what_fun() -> String {
        "Something Fun!".to_string()
    }

    fn what_else() -> String {
        "Something Else".to_string()
    }

    let props = Props {
        visible: true,
        menu: MenuProps {
            icon: "icon.png".to_string()
        }
    };

    let tree = rsx! {
        <Dropdown show={props.visible}>
            A dropdown list
            <Menu icon={props.menu.icon}>
                <MenuItem>Do Something</MenuItem>
                {
                    if should_do_something_fun() {
                        fragment! {
                            <MenuItem>Do{ what_fun() }</MenuItem>
                        }
                    } else {
                        fragment! {
                            <MenuItem>Do{ what_else() }</MenuItem>
                        }
                    }
                }
            </Menu>
        </Dropdown>
    };

    let expected = fragment! {
        DOMNode::from((
            DOMTagName::from("Dropdown"),
            vec![
                DOMAttribute::from((
                    DOMAttributeName::from("show"),
                    DOMAttributeValue::from(true)
                )),
            ],
            vec![
                DOMNode::from("A dropdown list"),
                DOMNode::from((
                    DOMTagName::from("Menu"),
                    vec![
                        DOMAttribute::from((
                            DOMAttributeName::from("icon"),
                            DOMAttributeValue::from("icon.png")
                        )),
                    ],
                    vec![
                        DOMNode::from((
                            DOMTagName::from("MenuItem"),
                            vec![],
                            vec![DOMNode::from("Do Something")]
                        )),
                        DOMNode::from((
                            DOMTagName::from("MenuItem"),
                            vec![],
                            vec![DOMNode::from("Do"), DOMNode::from("Something Fun!")]
                        )),
                    ]
                )),
            ]
        ))
    };

    assert_eq!(
        tree.root().traverse_iter().collect::<Vec<_>>(),
        expected.root().traverse_iter().collect::<Vec<_>>()
    );
}

#[test]
fn test_css_to_stylesheet_1() {
    let stylesheet = css! {
        .foo {
            margin: 0 auto;
            padding: 10px;
        }
    };

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![StyleSelector::from(".foo")])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10f32.into()))),
            ]))
        },
    ]));

    assert_eq!(stylesheet, expected);
}

#[test]
fn test_css_to_stylesheet_2() {
    let stylesheet = css! {
        .foo, .bar {
            margin: 0 auto;
            padding: 10px;
        }
    };

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector::from(".foo"),
                StyleSelector::from(".bar"),
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10f32.into()))),
            ]))
        },
    ]));

    assert_eq!(stylesheet, expected);
}

#[test]
fn test_css_to_stylesheet_3() {
    let stylesheet = css! {
        .foo, .bar-baz {
            margin: 0 auto;
            padding: 10px;
            flex-wrap: nowrap;
            flex-direction: row-reverse;
        }
    };

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector::from(".foo"),
                StyleSelector::from(".bar-baz"),
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::FlexWrap(Wrap::NoWrap)),
                StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::RowReverse)),
            ]))
        },
    ]));

    assert_eq!(stylesheet, expected);
}

#[test]
fn test_css_to_stylesheet_4() {
    let stylesheet = css!("tests/fixtures/test_1.css");

    let expected = Stylesheet::from(InlineRules::from_vec(vec![
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector::from(".root"),
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                    red: 255,
                    green: 0,
                    blue: 0,
                    alpha: 255
                })),
                StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500.0.into()))),
                StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120.0.into()))),
                StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20.0.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20.0.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20.0.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20.0.into()))),
            ]))
        },
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector::from(".image"),
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                    red: 0,
                    green: 128,
                    blue: 0,
                    alpha: 255
                })),
                StyleDeclaration::Theme(ThemeStyle::Opacity(50)),
                StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80.0.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20.0.into()))),
            ]))
        },
        StyleRule {
            selectors: StyleSelectors(InlineSelectors::from_vec(vec![
                StyleSelector::from(".text"),
            ])),
            declarations: StyleDeclarations(InlineDeclarations::from_vec(vec![
                StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                    red: 0,
                    green: 0,
                    blue: 255,
                    alpha: 255
                })),
                StyleDeclaration::Theme(ThemeStyle::Color(Color {
                    red: 255,
                    green: 255,
                    blue: 0,
                    alpha: 255
                })),
                StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25.0.into()))),
                StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                StyleDeclaration::Layout(FlexStyle::FlexGrow(1.0.into())),
            ]))
        },
    ]));

    assert_eq!(stylesheet, expected);
}

#[test]
fn test_rsx_and_css_1() {
    let mut stylesheet = css! {
        .foo {
            margin: 0 auto;
            padding: 10px;
        }
    };

    let tree = rsx! {
        <div style={stylesheet.take(".foo")}>
            Hello world!
        </div>
    };

    let expected = fragment! {
        DOMNode::from((
            DOMTagName::from(KnownElementName::Div),
            vec![
                DOMAttribute::from((
                    DOMAttributeName::from(KnownAttributeName::Style),
                    DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                        StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                        StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                        StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10.0.into()))),
                    ])))
                )),
            ],
            vec![DOMNode::from("Hello world !")]
        ))
    };

    assert_eq!(
        tree.root().traverse_iter().collect::<Vec<_>>(),
        expected.root().traverse_iter().collect::<Vec<_>>()
    );
}

#[test]
fn test_rsx_and_css_2() {
    let mut stylesheet = css!("tests/fixtures/test_2.css");

    let tree = rsx! {
        <div style={stylesheet.take(".bar")}>
            Hello world!
        </div>
    };

    let expected = fragment! {
        DOMNode::from((
            DOMTagName::from(KnownElementName::Div),
            vec![
                DOMAttribute::from((
                    DOMAttributeName::from(KnownAttributeName::Style),
                    DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::default()))
                )),
            ],
            vec![DOMNode::from("Hello world !")]
        ))
    };

    assert_eq!(
        tree.root().traverse_iter().collect::<Vec<_>>(),
        expected.root().traverse_iter().collect::<Vec<_>>()
    );
}

#[test]
fn test_rsx_x_1() {
    let tree = rsx! {
        <fragment>
            Hello world!
        </fragment>
    };

    let expected = fragment! {
        DOMNode::from((
            DOMTagName::from(KnownElementName::Fragment),
            vec![],
            vec![DOMNode::from("Hello world !")]
        ))
    };

    assert_eq!(
        tree.root().traverse_iter().collect::<Vec<_>>(),
        expected.root().traverse_iter().collect::<Vec<_>>()
    );
}

#[test]
fn test_rsx_x_2() {
    let tree = rsx! {
        <x-image-display-item>
            <x-button-display-item>
                Hello world!
            </x-button-display-item>
        </x-image-display-item>
    };

    let expected = fragment! {
        DOMNode::from((
            DOMTagName::from("x-image-display-item"),
            vec![],
            vec![
                DOMNode::from((
                    DOMTagName::from("x-button-display-item"),
                    vec![],
                    vec![DOMNode::from("Hello world !")]
                )),
            ]
        ))
    };

    assert_eq!(
        tree.root().traverse_iter().collect::<Vec<_>>(),
        expected.root().traverse_iter().collect::<Vec<_>>()
    );
}

#[test]
fn test_example_1() {
    let mut stylesheet = css! {
        .root {
            width: 500px;
            height: 120px;
            flex-direction: row;
            padding: 20px;
        }
        .image {
            width: 80px;
            margin-right: 20px;
        }
        .text {
            height: 25px;
            align-self: center;
            flex-grow: 1;
        }
    };

    let tree = rsx! {
        <view style={stylesheet.take(".root")}>
            <image style={stylesheet.take(".image")} src="..." />
            <text style={stylesheet.take(".text")}>
                Hello world!
            </text>
        </view>
    };

    let expected = fragment! {
        DOMNode::from((
            DOMTagName::from(KnownElementName::View),
            vec![
                DOMAttribute::from((
                    DOMAttributeName::from(KnownAttributeName::Style),
                    DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                        StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                        StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20.0.into()))),
                    ])))
                )),
            ],
            vec![
                DOMNode::from((
                    DOMTagName::from(KnownElementName::Image),
                    vec![
                        DOMAttribute::from((
                            DOMAttributeName::from(KnownAttributeName::Style),
                            DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                                StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80.0.into()))),
                                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20.0.into()))),
                            ])))
                        )),
                        DOMAttribute::from((
                            DOMAttributeName::from(KnownAttributeName::Src),
                            DOMAttributeValue::from("...")
                        )),
                    ]
                )),
                DOMNode::from((
                    DOMTagName::from(KnownElementName::Text),
                    vec![
                        DOMAttribute::from((
                            DOMAttributeName::from(KnownAttributeName::Style),
                            DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                                StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25.0.into()))),
                                StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                                StyleDeclaration::Layout(FlexStyle::FlexGrow(1.0.into())),
                            ])))
                        )),
                    ],
                    vec![DOMNode::from("Hello world !")]
                )),
            ]
        ))
    };

    assert_eq!(
        tree.root().traverse_iter().collect::<Vec<_>>(),
        expected.root().traverse_iter().collect::<Vec<_>>()
    );
}

#[test]
fn test_example_2() {
    fn greeting_str(name: &str) -> String {
        format!("Hello {}!", name)
    }

    fn render_greeting(name: &str) -> DOMTree {
        let mut stylesheet = css!("tests/fixtures/test_1.css");

        rsx! {
            <text style={stylesheet.take(".text")}>
                { greeting_str(name) }
            </text>
        }
    }

    fn render_children(name: Option<&str>, image: DOMTree) -> DOMTree {
        rsx! {
            <view>
                { image }
                {
                    match name {
                        Some(ref n) => render_greeting(n),
                        None => fragment! {
                            <text>No greetings!</text>
                        }
                    }
                }
            </view>
        }
    }

    fn render_root() -> DOMTree {
        let mut stylesheet = css!("tests/fixtures/test_1.css");

        rsx! {
            <view style={stylesheet.take(".root")}>
                {
                    let name = Some("world");
                    let image = fragment! {
                        <image style={stylesheet.take(".image")} src="..." />
                    };
                    render_children(name, image)
                }
            </view>
        }
    }

    let tree = render_root();

    let expected = fragment! {
        DOMNode::from((
            DOMTagName::from(KnownElementName::View),
            vec![
                DOMAttribute::from((
                    DOMAttributeName::from(KnownAttributeName::Style),
                    DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                        StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                            red: 255,
                            green: 0,
                            blue: 0,
                            alpha: 255
                        })),
                        StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                        StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20.0.into()))),
                    ])))
                )),
            ],
            vec![
                DOMNode::from((
                    DOMTagName::from(KnownElementName::View),
                    vec![],
                    vec![
                        DOMNode::from((
                            DOMTagName::from(KnownElementName::Image),
                            vec![
                                DOMAttribute::from((
                                    DOMAttributeName::from(KnownAttributeName::Style),
                                    DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                                        StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                                            red: 0,
                                            green: 128,
                                            blue: 0,
                                            alpha: 255
                                        })),
                                        StyleDeclaration::Theme(ThemeStyle::Opacity(50)),
                                        StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80.0.into()))),
                                        StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20.0.into()))),
                                    ])))
                                )),
                                DOMAttribute::from((
                                    DOMAttributeName::from(KnownAttributeName::Src),
                                    DOMAttributeValue::from("...")
                                )),
                            ]
                        )),
                        DOMNode::from((
                            DOMTagName::from(KnownElementName::Text),
                            vec![
                                DOMAttribute::from((
                                    DOMAttributeName::from(KnownAttributeName::Style),
                                    DOMAttributeValue::from(StyleDeclarations(InlineDeclarations::from_vec(vec![
                                        StyleDeclaration::Theme(ThemeStyle::BackgroundColor(Color {
                                            red: 0,
                                            green: 0,
                                            blue: 255,
                                            alpha: 255
                                        })),
                                        StyleDeclaration::Theme(ThemeStyle::Color(Color {
                                            red: 255,
                                            green: 255,
                                            blue: 0,
                                            alpha: 255
                                        })),
                                        StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25.0.into()))),
                                        StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                                        StyleDeclaration::Layout(FlexStyle::FlexGrow(1.0.into())),
                                    ])))
                                )),
                            ],
                            vec![DOMNode::from("Hello world!")]
                        )),
                    ]
                )),
            ]
        ))
    };

    assert_eq!(
        tree.root().traverse_iter().collect::<Vec<_>>(),
        expected.root().traverse_iter().collect::<Vec<_>>()
    );
}

#[test]
fn test_image_load() {
    let image = load_image!("tests/fixtures/Quantum.png");

    if let EncodedImage::BytesAndDataUri {
        format,
        size_info,
        data_uri,
        ..
    } = image
    {
        assert_eq!(format, ImageEncodingFormat::PNG);
        assert_eq!(size_info, Some((512, 529)));
        assert!(data_uri.starts_with("data:image/png;base64,"));
    } else {
        assert!(false, "Improperly loaded image.");
    }
}

#[test]
fn test_font_load() {
    let font = load_font!("tests/fixtures/FreeSans.ttf");

    if let EncodedFont::BytesAndDataUri { data_uri, .. } = font {
        assert!(data_uri.starts_with("data:application/x-font-woff;base64,"));
    } else {
        assert!(false, "Improperly loaded font.");
    }
}
