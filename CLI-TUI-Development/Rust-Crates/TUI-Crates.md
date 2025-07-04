# Rust TUI Development - ターミナルUIクレート

## 🖥️ TUI フレームワーク

### ratatui - モダンTUIフレームワーク

**基本情報**
- **人気度**: ⭐⭐⭐⭐⭐ (最も人気)
- **特徴**: tui-rsの後継、活発に開発中
- **公式**: https://github.com/ratatui-org/ratatui
- **ドキュメント**: https://ratatui.rs/

**主な特徴**
```toml
[dependencies]
ratatui = "0.25"
crossterm = "0.27"
```

#### 基本構造

**最小限のTUIアプリ**
```rust
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

struct App {
    counter: usize,
}

impl App {
    fn new() -> App {
        App { counter: 0 }
    }

    fn on_tick(&mut self) {
        self.counter += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // ターミナル設定
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // アプリ作成とイベントループ
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // 復元
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }

        app.on_tick();
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let block = Block::default().title("Counter").borders(Borders::ALL);
    let paragraph = Paragraph::new(format!("Count: {}", app.counter)).block(block);
    f.render_widget(paragraph, chunks[0]);
}
```

#### ウィジェット例

**リストウィジェット**
```rust
use ratatui::{
    style::{Color, Style},
    widgets::{List, ListItem, ListState},
};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
```

**長所**
- 活発な開発コミュニティ
- 豊富なウィジェット
- クロスプラットフォーム対応
- 柔軟なレイアウトシステム

**短所**
- immediate mode rendering
- 学習コストが高い
- メモリ使用量がやや多い

---

### cursive - 高レベルTUIフレームワーク

**基本情報**
- **人気度**: ⭐⭐⭐⭐
- **特徴**: 使いやすさ重視の高レベルAPI
- **公式**: https://github.com/gyscos/cursive

```toml
[dependencies]
cursive = "0.20"
```

**基本使用例**
```rust
use cursive::views::{Dialog, TextView};
use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();

    siv.add_layer(Dialog::around(TextView::new("Hello World!"))
                     .title("Greeting")
                     .button("Quit", |s| s.quit()));

    siv.run();
}
```

**複雑なレイアウト**
```rust
use cursive::{
    views::{LinearLayout, TextView, EditView, Button, SelectView},
    traits::*,
    Cursive,
};

fn main() {
    let mut siv = Cursive::default();

    let layout = LinearLayout::vertical()
        .child(TextView::new("Enter your information:"))
        .child(EditView::new()
            .with_name("name")
            .fixed_width(20))
        .child(SelectView::<String>::new()
            .item("Option 1", "opt1".to_string())
            .item("Option 2", "opt2".to_string())
            .with_name("select"))
        .child(Button::new("Submit", |s| {
            let name = s.call_on_name("name", |view: &mut EditView| {
                view.get_content()
            }).unwrap();
            
            s.add_layer(Dialog::info(format!("Hello, {}!", name)));
        }));

    siv.add_layer(Dialog::around(layout)
        .title("Form")
        .button("Quit", |s| s.quit()));

    siv.run();
}
```

**イベント処理**
```rust
use cursive::{
    event::{Event, Key},
    views::TextView,
    Cursive,
};

fn main() {
    let mut siv = Cursive::default();
    
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Key::Esc, |s| s.quit());
    
    siv.add_layer(TextView::new("Press 'q' or Esc to quit"));
    
    siv.run();
}
```

**長所**
- 直感的で使いやすいAPI
- イベント駆動モデル
- レイアウト管理が自動
- ビューの再利用可能

**短所**
- カスタマイゼーションが限定的
- パフォーマンスがratatuiより劣る
- ウィジェットの種類が少ない

---

### crossterm - ターミナル制御

**基本情報**
- **人気度**: ⭐⭐⭐⭐⭐ (デファクトスタンダード)
- **特徴**: クロスプラットフォーム対応ターミナル制御
- **公式**: https://github.com/crossterm-rs/crossterm

```toml
[dependencies]
crossterm = "0.27"
```

**基本的な使用例**
```rust
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};
use std::{
    io::{stdout, Write},
    time::Duration,
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0),
        SetForegroundColor(Color::Blue),
        Print("Hello Crossterm!"),
        MoveTo(0, 1),
        SetForegroundColor(Color::Red),
        Print("Press any key to continue..."),
        ResetColor,
        Hide
    )?;
    
    // イベント待機
    loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => break,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    
    execute!(stdout, Show, ResetColor)?;
    disable_raw_mode()?;
    Ok(())
}
```

**長所**
- 完全なクロスプラットフォーム対応
- 低レベル制御が可能
- 軽量で高速
- 豊富なイベント処理

**短所**
- 低レベルAPI（高レベルフレームワークが必要）
- 直接使用は煩雑

---

## 🔌 TUI フレームワーク拡張

### tui-realm - Reactライク TUI

**基本情報**
- **人気度**: ⭐⭐⭐
- **特徴**: React/Elmインスパイアのコンポーネントシステム
- **公式**: https://github.com/veeso/tui-realm

```toml
[dependencies]
tuirealm = "1.9"
```

**基本概念**
```rust
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, Borders, Color, TextSpan},
    tui::style::Style,
    Component, Event, MockComponent, State, StateValue,
};

#[derive(MockComponent)]
pub struct Counter {
    component: tuirealm::Sub<tuirealm::SubClause>,
    count: isize,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            component: tuirealm::Sub::default(),
            count: 0,
        }
    }
}

impl Component<Msg, NoUserEvent> for Counter {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd_result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('+'),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.count += 1;
                CmdResult::Changed(self.state())
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('-'),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.count -= 1;
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        };
        
        match cmd_result {
            CmdResult::Changed(state) => Some(Msg::CounterChanged(state)),
            _ => None,
        }
    }
}
```

### tui-input - 入力フィールド

**基本情報**
- **特徴**: ratatui用の高機能入力フィールド
- **機能**: カーソル制御、選択、クリップボード対応

```toml
[dependencies]
tui-input = "0.8"
```

**使用例**
```rust
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use crossterm::event::{Event, KeyCode};

let mut input = Input::default();

// イベント処理
match crossterm::event::read()? {
    Event::Key(key) => {
        match key.code {
            KeyCode::Enter => {
                let text = input.value();
                // 入力処理
            }
            _ => {
                input.handle_event(&Event::Key(key));
            }
        }
    }
    _ => {}
}

// レンダリング（ratatui内で）
use ratatui::widgets::Paragraph;

let paragraph = Paragraph::new(input.value())
    .style(Style::default())
    .block(Block::default().borders(Borders::ALL));
f.render_widget(paragraph, area);
```

---

## 🎨 TUI ユーティリティ

### tui-tree-widget - ツリービュー

```rust
use tui_tree_widget::{Tree, TreeItem, TreeState};

let items = vec![
    TreeItem::new_leaf("Leaf 1"),
    TreeItem::new(
        "Parent",
        vec![
            TreeItem::new_leaf("Child 1"),
            TreeItem::new_leaf("Child 2"),
        ]
    ).expect("Failed to create tree item"),
];

let mut state = TreeState::default();
let tree = Tree::new(items)
    .block(Block::default().borders(Borders::ALL).title("Tree"))
    .highlight_style(Style::default().fg(Color::Yellow))
    .highlight_symbol("> ");

f.render_stateful_widget(tree, area, &mut state);
```

### tui-logger - ログ表示

```rust
use tui_logger::*;

// 初期化
init_logger(log::LevelFilter::Debug).unwrap();
set_default_level(log::LevelFilter::Debug);

// ログ出力
log::info!("This is an info message");
log::warn!("This is a warning");
log::error!("This is an error");

// TUIでの表示
let tui_widget = TuiLoggerWidget::default()
    .style_error(Style::default().fg(Color::Red))
    .style_debug(Style::default().fg(Color::Green))
    .style_warn(Style::default().fg(Color::Yellow))
    .style_trace(Style::default().fg(Color::Gray))
    .style_info(Style::default().fg(Color::Blue))
    .block(
        Block::default()
            .title("Logs")
            .border_style(Style::default().fg(Color::White).bg(Color::Black))
            .borders(Borders::ALL),
    )
    .output_separator(' ')
    .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()));

f.render_widget(tui_widget, area);
```

---

## 📊 TUI フレームワーク比較

| フレームワーク | 学習コスト | パフォーマンス | カスタマイゼーション | ウィジェット数 |
|---------------|------------|---------------|---------------------|---------------|
| **ratatui** | 高 | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **cursive** | 低 | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **tui-realm** | 中 | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ |

## 🎯 使い分けガイド

### ratatui を選ぶべき場合
- 高性能なTUIアプリケーション
- 豊富なウィジェットが必要
- カスタマイゼーションを重視
- リアルタイムデータ表示

### cursive を選ぶべき場合
- 簡単にTUIを作成したい
- イベント駆動のアプリケーション
- フォーム中心のUI
- 学習コストを抑えたい

### tui-realm を選ぶべき場合
- コンポーネント指向の開発
- React/Elmの経験がある
- 再利用可能なコンポーネント
- 状態管理を重視

### crossterm を選ぶべき場合
- 既存フレームワークでは不十分
- 低レベル制御が必要
- カスタムTUIフレームワーク作成
- クロスプラットフォーム対応が重要

---

## 🏗️ アーキテクチャパターン

### イベントループパターン
```rust
use crossterm::event::{Event, KeyCode};
use ratatui::Terminal;
use std::time::{Duration, Instant};

struct App {
    should_quit: bool,
    tick_rate: Duration,
    last_tick: Instant,
}

impl App {
    fn new(tick_rate: Duration) -> Self {
        Self {
            should_quit: false,
            tick_rate,
            last_tick: Instant::now(),
        }
    }
    
    fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            // その他のキー処理
            _ => {}
        }
    }
    
    fn on_tick(&mut self) {
        // 定期的な更新処理
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
        
        let timeout = app.tick_rate
            .checked_sub(app.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
            
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = crossterm::event::read()? {
                app.on_key(key.code);
            }
        }
        
        if app.last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
            app.last_tick = Instant::now();
        }
        
        if app.should_quit {
            return Ok(());
        }
    }
}
```

### State Management パターン
```rust
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Screen {
    Home,
    Settings,
    Help,
}

struct AppState {
    current_screen: Screen,
    data: HashMap<String, String>,
    selected_index: usize,
}

impl AppState {
    fn new() -> Self {
        Self {
            current_screen: Screen::Home,
            data: HashMap::new(),
            selected_index: 0,
        }
    }
    
    fn handle_input(&mut self, key: KeyCode) {
        match (self.current_screen.clone(), key) {
            (Screen::Home, KeyCode::Char('s')) => {
                self.current_screen = Screen::Settings;
            }
            (Screen::Settings, KeyCode::Esc) => {
                self.current_screen = Screen::Home;
            }
            (_, KeyCode::Up) => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            (_, KeyCode::Down) => {
                self.selected_index += 1;
            }
            _ => {}
        }
    }
}
```

---

**関連ドキュメント**:
- [CLI-Crates.md](./CLI-Crates.md) - CLI開発用クレート
- [Utility-Crates.md](./Utility-Crates.md) - ユーティリティクレート
- [Best-Practices.md](./Best-Practices.md) - ベストプラクティス
