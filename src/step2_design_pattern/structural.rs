// 구조 패턴

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct Request {
    username: String,
    role: String,
}

trait Handler {
    fn set_next(&mut self, next: Box<dyn Handler>);
    fn handle(&self, request: &Request) -> Result<String, String>;
}

struct ClientCheckHandler {
    next: Option<Box<dyn Handler>>,
}
impl ClientCheckHandler {
    fn new() -> Self {
        ClientCheckHandler { next: None }
    }
}
impl Handler for ClientCheckHandler {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }

    fn handle(&self, request: &Request) -> Result<String, String> {
        if request.username.is_empty() {
            return Err("Invalid Username".to_string());
        }

        match &self.next {
            Some(handler) => handler.handle(request),
            None => Ok("Success".to_string()),
        }
    }
}

struct RoleCheckHandler {
    next: Option<Box<dyn Handler>>,
}

impl RoleCheckHandler {
    fn new() -> Self {
        RoleCheckHandler { next: None }
    }
}

impl Handler for RoleCheckHandler {
    fn set_next(&mut self, next: Box<dyn Handler>) {
        self.next = Some(next);
    }

    fn handle(&self, request: &Request) -> Result<String, String> {
        if request.role != "admin" {
            return Err("Access Denied for Role".to_string());
        }

        match &self.next {
            Some(handler) => handler.handle(request),
            None => Ok("Success".to_string()),
        }
    }
}

// Command

struct Editor {
    value: i32,
}

impl Editor {
    fn new() -> Self {
        Editor { value: 0 }
    }
}

trait Command {
    fn execute(&mut self, editor: &mut Editor);
    fn undo(&mut self, editor: &mut Editor);
}

struct IncrementCommand {
    amount: i32,
}

impl IncrementCommand {
    fn new(amount: i32) -> Self {
        IncrementCommand { amount }
    }
}

impl Command for IncrementCommand {
    fn execute(&mut self, editor: &mut Editor) {
        editor.value += self.amount
    }

    fn undo(&mut self, editor: &mut Editor) {
        editor.value -= self.amount
    }
}

// Interpreter

trait Expression {
    fn interpret(&self) -> i32;
}

struct Number {
    value: i32,
}
impl Number {
    fn new(value: i32) -> Self {
        Number { value }
    }
}

impl Expression for Number {
    fn interpret(&self) -> i32 {
        self.value
    }
}

struct Add {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}
impl Add {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Add { left, right }
    }
}
impl Expression for Add {
    fn interpret(&self) -> i32 {
        self.left.interpret() + self.right.interpret()
    }
}
struct Sub {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}
impl Sub {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Sub { left, right }
    }
}

impl Expression for Sub {
    fn interpret(&self) -> i32 {
        self.left.interpret() - self.right.interpret()
    }
}

// Iterator

struct EvenIterator {
    cursor: usize,
    data: Vec<i32>,
}

impl EvenIterator {
    fn new(data: Vec<i32>) -> Self {
        EvenIterator { cursor: 0, data }
    }
}

impl Iterator for EvenIterator {
    type Item = i32;

    // next 메서드를 구현하여 커서를 이동시키면서 짝수 값만 순차적으로 반환하도록 만드세요.
    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < self.data.len() {
            let current_val = self.data[self.cursor];
            self.cursor += 1;

            if current_val % 2 == 0 {
                return Some(current_val);
            }
        }
        None
    }
}

// Mediator

struct ChatRoom {
    participants: Vec<String>,
}
impl ChatRoom {
    fn new() -> Self {
        ChatRoom {
            participants: Vec::new(),
        }
    }

    fn register(&mut self, user_name: &str) {
        self.participants.push(user_name.to_string());
    }

    fn broadcase(&self, sender: &str, message: &str) -> Vec<String> {
        self.participants
            .iter()
            .filter(|&participant| participant != sender)
            .map(|participant| format!("{} received from {}: {}", participant, sender, message))
            .collect()
    }
}

struct User<'a> {
    name: String,
    mediator: &'a ChatRoom,
}

impl<'a> User<'a> {
    fn new(name: &str, mediator: &'a ChatRoom) -> Self {
        User {
            name: name.to_string(),
            mediator,
        }
    }

    fn send_message(&self, message: &str) -> Vec<String> {
        self.mediator.broadcase(&self.name, message)
    }
}

// Memento

#[derive(Clone)]
struct EditorMemento {
    state: String,
}
struct EditorM {
    contents: String,
}

impl EditorM {
    fn new() -> Self {
        EditorM {
            contents: String::new(),
        }
    }

    fn type_text(&mut self, text: &str) {
        self.contents.push_str(text);
    }

    fn save(&self) -> EditorMemento {
        // eiditor는 contents 를 계속 소유하고 있어야하므로 clone()을 사용해야 한다.
        EditorMemento {
            state: self.contents.clone(),
        }
    }

    fn restore(&mut self, memento: &EditorMemento) {
        self.contents = memento.state.clone();
    }
}

// Observer

trait Observer {
    fn update(&mut self, temperature: f32);
}

struct CurrentConditionsDisplay {
    temperature: f32,
}
impl CurrentConditionsDisplay {
    fn new() -> Self {
        CurrentConditionsDisplay { temperature: 0.0 }
    }
}
impl Observer for CurrentConditionsDisplay {
    fn update(&mut self, temperature: f32) {
        self.temperature = temperature;
    }
}

struct WeatherStation {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    temperature: f32,
}

impl WeatherStation {
    fn new() -> Self {
        WeatherStation {
            observers: Vec::new(),
            temperature: 0.0,
        }
    }

    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn set_measurements(&mut self, temperature: f32) {
        self.temperature = temperature;
        for observer in &self.observers {
            observer.borrow_mut().update(temperature);
        }
    }
}

// State

trait State {
    fn press_button(&self) -> (String, Box<dyn State>);
}

struct OffState;
impl State for OffState {
    fn press_button(&self) -> (String, Box<dyn State>) {
        (
            "Turning fan on to low speed.".to_string(),
            Box::new(LowState),
        )
    }
}

struct LowState;
impl State for LowState {
    fn press_button(&self) -> (String, Box<dyn State>) {
        ("Turning fan off.".to_string(), Box::new(OffState))
    }
}

struct Fan {
    state: Option<Box<dyn State>>,
}
impl Fan {
    fn new() -> Self {
        Fan {
            state: Some(Box::new(OffState)),
        }
    }

    fn press_button(&mut self) -> String {
        if let Some(current_state) = self.state.take() {
            let (log, next_state) = current_state.press_button();
            self.state = Some(next_state);
            log
        } else {
            String::new()
        }
    }
}

// Strategy

trait PaymentStrategy {
    fn calculate_price(&self, price: u32) -> u32;
}

struct NormalStrategy;
impl PaymentStrategy for NormalStrategy {
    fn calculate_price(&self, price: u32) -> u32 {
        price
    }
}

struct DiscountStrategy;
impl PaymentStrategy for DiscountStrategy {
    fn calculate_price(&self, price: u32) -> u32 {
        price * 90 / 100
    }
}

struct ShoppingCart {
    strategy: Box<dyn PaymentStrategy>,
}
impl ShoppingCart {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        ShoppingCart { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.strategy = strategy
    }

    fn checkout(&self, price: u32) -> u32 {
        self.strategy.calculate_price(price)
    }
}

// Template Method

trait DataMiner {
    fn open_file(&self) -> String;
    fn close_file(&self) -> String;

    fn mine(&self) -> String {
        vec![
            self.open_file(),
            "Extracting data...".to_string(),
            self.close_file(),
        ]
        .join("\n")
    }
}

struct PdfMiner;
impl DataMiner for PdfMiner {
    fn open_file(&self) -> String {
        "Opening PDF file.".to_string()
    }
    fn close_file(&self) -> String {
        "Closing PDF file.".to_string()
    }
}

struct CsvMiner;
impl DataMiner for CsvMiner {
    fn open_file(&self) -> String {
        "Opening CSV file.".to_string()
    }
    fn close_file(&self) -> String {
        "Closing CSV file.".to_string()
    }
}

// Visitor

trait Visitor {
    fn visit_circle(&self, circle: &Circle);
    fn visit_rectangle(&self, rectangle: &Rectangle);
}

trait ShapeElement {
    fn accept(&self, visitor: &dyn Visitor);
}

struct Circle {
    radius: f64,
}
impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }
}
impl ShapeElement for Circle {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_circle(self);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}
impl ShapeElement for Rectangle {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_rectangle(self);
    }
}

struct AreaVisitor {
    total_area: Cell<f64>,
}

impl AreaVisitor {
    fn new() -> Self {
        AreaVisitor {
            total_area: Cell::new(0.0),
        }
    }

    fn get_total_area(&self) -> f64 {
        self.total_area.get()
    }
}
impl Visitor for AreaVisitor {
    fn visit_circle(&self, circle: &Circle) {
        let area = circle.radius * circle.radius * 3.14;
        self.total_area.set(self.total_area.get() + area);
    }

    fn visit_rectangle(&self, rectangle: &Rectangle) {
        let area = rectangle.width * rectangle.height;
        self.total_area.set(self.total_area.get() + area);
    }
}

#[cfg(test)]
#[path = "structural_test.rs"]
mod structural_test;
