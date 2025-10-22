// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    app::Task,
    Action,
};
use std::{
    collections::VecDeque,
    path::PathBuf,
};
use tokio::sync::mpsc;
use crate::{
    app::Message,
    core::operations::ReplaceResult,
    core::services::mount::{MounterAuth, MounterItem, MounterKey},
    tab,
    utils::mime_app::MimeApp,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ArchiveType {
    Tgz,
    #[default]
    Zip,
}

impl ArchiveType {
    pub fn all() -> &'static [Self] {
        &[Self::Tgz, Self::Zip]
    }

    pub fn extension(&self) -> &str {
        match self {
            ArchiveType::Tgz => ".tgz",
            ArchiveType::Zip => ".zip",
        }
    }
}

impl AsRef<str> for ArchiveType {
    fn as_ref(&self) -> &str {
        self.extension()
    }
}

#[derive(Clone, Debug)]
pub enum DialogPage {
    Compress {
        paths: Vec<PathBuf>,
        to: PathBuf,
        name: String,
        archive_type: ArchiveType,
        password: Option<String>,
    },
    EmptyTrash,
    FailedOperation(u64),
    ExtractPassword {
        id: u64,
        password: String,
    },
    MountError {
        mounter_key: MounterKey,
        item: MounterItem,
        error: String,
    },
    NetworkAuth {
        mounter_key: MounterKey,
        uri: String,
        auth: MounterAuth,
        auth_tx: mpsc::Sender<MounterAuth>,
    },
    NetworkError {
        mounter_key: MounterKey,
        uri: String,
        error: String,
    },
    NewItem {
        parent: PathBuf,
        name: String,
        dir: bool,
    },
    OpenWith {
        path: PathBuf,
        mime: mime_guess::Mime,
        selected: usize,
        store_opt: Option<MimeApp>,
    },
    PermanentlyDelete {
        paths: Vec<PathBuf>,
    },
    RenameItem {
        from: PathBuf,
        parent: PathBuf,
        name: String,
        dir: bool,
    },
    Replace {
        from: tab::Item,
        to: tab::Item,
        multiple: bool,
        apply_to_all: bool,
        tx: mpsc::Sender<ReplaceResult>,
    },
    SetExecutableAndLaunch {
        path: PathBuf,
    },
    FavoritePathError {
        path: PathBuf,
        entity: cosmic::widget::segmented_button::Entity,
    },
}

pub struct DialogPages {
    pages: VecDeque<DialogPage>,
}

impl Default for DialogPages {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogPages {
    pub fn new() -> Self {
        Self {
            pages: VecDeque::new(),
        }
    }

    pub fn front(&self) -> Option<&DialogPage> {
        self.pages.front()
    }

    pub fn front_mut(&mut self) -> Option<&mut DialogPage> {
        self.pages.front_mut()
    }

    pub fn push_back(&mut self, page: DialogPage) -> Task<Message> {
        let task = if self.pages.is_empty() {
            Task::done(Action::App(Message::DesktopDialogs(true)))
        } else {
            Task::none()
        };
        self.pages.push_back(page);
        task
    }

    pub fn push_front(&mut self, page: DialogPage) -> Task<Message> {
        let task = if self.pages.is_empty() {
            Task::done(Action::App(Message::DesktopDialogs(true)))
        } else {
            Task::none()
        };
        self.pages.push_front(page);
        task
    }

    #[must_use]
    pub fn pop_front(&mut self) -> Option<(DialogPage, Task<Message>)> {
        let page = self.pages.pop_front()?;
        let task = if self.pages.is_empty() {
            Task::done(Action::App(Message::DesktopDialogs(false)))
        } else {
            Task::none()
        };
        Some((page, task))
    }

    pub fn update_front(&mut self, page: DialogPage) {
        if !self.pages.is_empty() {
            self.pages[0] = page;
        }
    }
}
