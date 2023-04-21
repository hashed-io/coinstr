// Copyright (c) 2022-2023 Coinstr
// Distributed under the MIT software license

use coinstr_core::bitcoin::Network;
use coinstr_core::types::WordCount;
use coinstr_core::util::dir;
use coinstr_core::Coinstr;
use iced::widget::{Checkbox, Column, Row};
use iced::{Command, Element, Length};

use super::view;
use crate::component::{button, rule, Text, TextInput};
use crate::constants::APP_NAME;
use crate::start::{Context, Message, Stage, State};
use crate::theme::color::DARK_RED;
use crate::KEYCHAINS_PATH;

#[derive(Debug, Clone)]
pub enum GenerateMessage {
    NameChanged(String),
    PasswordChanged(String),
    ConfirmPasswordChanged(String),
    UsePassphrase(bool),
    PassphraseChanged(String),
    Generate,
}

#[derive(Debug, Default)]
pub struct GenerateState {
    name: String,
    password: String,
    confirm_password: String,
    use_passphrase: bool,
    passphrase: String,
    // mnemonic: Option<Mnemonic>,
    error: Option<String>,
}

impl GenerateState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl State for GenerateState {
    fn title(&self) -> String {
        format!("{APP_NAME} - Generate")
    }

    fn update(&mut self, _ctx: &mut Context, message: Message) -> Command<Message> {
        if let Message::Generate(msg) = message {
            match msg {
                GenerateMessage::NameChanged(name) => self.name = name,
                GenerateMessage::PasswordChanged(passwd) => self.password = passwd,
                GenerateMessage::ConfirmPasswordChanged(passwd) => self.confirm_password = passwd,
                GenerateMessage::UsePassphrase(value) => {
                    self.use_passphrase = value;
                    self.passphrase = String::new();
                }
                GenerateMessage::PassphraseChanged(passphrase) => self.passphrase = passphrase,
                GenerateMessage::Generate => {
                    if self.password.eq(&self.confirm_password) {
                        // TODO: replace network
                        match dir::get_keychain_file(KEYCHAINS_PATH.as_path(), self.name.clone()) {
                            Ok(path) => match Coinstr::generate(
                                path,
                                || Ok(self.password.clone()),
                                WordCount::W12, // TODO: let user choose the len.
                                || Ok(Some(self.passphrase.clone())),
                                Network::Testnet,
                            ) {
                                Ok(keechain) => {
                                    return Command::perform(async {}, move |_| {
                                        Message::OpenResult(keechain)
                                    })
                                }
                                Err(e) => self.error = Some(e.to_string()),
                            },
                            Err(e) => self.error = Some(e.to_string()),
                        }
                    } else {
                        self.error = Some("Passwords not match".to_string())
                    }
                }
            }
        };

        Command::none()
    }

    fn view(&self, _ctx: &Context) -> Element<Message> {
        let name = TextInput::new("Name", &self.name)
            .on_input(|s| GenerateMessage::NameChanged(s).into())
            .placeholder("Name of keychain")
            .view();

        let password = TextInput::new("Password", &self.password)
            .on_input(|s| GenerateMessage::PasswordChanged(s).into())
            .placeholder("Password")
            .password()
            .view();

        let confirm_password = TextInput::new("Confirm password", &self.confirm_password)
            .on_input(|s| GenerateMessage::ConfirmPasswordChanged(s).into())
            .placeholder("Confirm password")
            .password()
            .view();

        let use_passphrase = Checkbox::new("Use a passphrase", self.use_passphrase, |value| {
            GenerateMessage::UsePassphrase(value).into()
        })
        .width(Length::Fill);

        let passphrase = if self.use_passphrase {
            TextInput::new("Passphrase", &self.passphrase)
                .on_input(|s| GenerateMessage::PassphraseChanged(s).into())
                .placeholder("Passphrase")
                .view()
        } else {
            Column::new()
        };

        let generate_keychain_btn = button::primary("Generate")
            .on_press(GenerateMessage::Generate.into())
            .width(Length::Fill);

        let open_btn = button::border("Open keychain")
            .width(Length::Fill)
            .on_press(Message::View(Stage::Open));

        let restore_keychain_btn = button::border("Restore keychain")
            .on_press(Message::View(Stage::Restore))
            .width(Length::Fill);

        let content = Column::new()
            .push(name)
            .push(password)
            .push(confirm_password)
            .push(use_passphrase)
            .push(passphrase)
            .push(if let Some(error) = &self.error {
                Row::new().push(Text::new(error).color(DARK_RED).view())
            } else {
                Row::new()
            })
            .push(generate_keychain_btn)
            .push(rule::horizontal())
            .push(open_btn)
            .push(restore_keychain_btn);

        view(content)
    }
}

impl From<GenerateState> for Box<dyn State> {
    fn from(s: GenerateState) -> Box<dyn State> {
        Box::new(s)
    }
}

impl From<GenerateMessage> for Message {
    fn from(msg: GenerateMessage) -> Self {
        Self::Generate(msg)
    }
}
