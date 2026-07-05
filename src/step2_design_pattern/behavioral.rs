// 행동 패턴

// /////////////////////////////////////////////////
// Adapter

use std::{collections::HashMap, rc::Rc};

trait Shape {
    fn get_area(&self) -> f64;
}

pub struct LegacyRoundPeg {
    radius: f64,
}
impl LegacyRoundPeg {
    pub fn new(radius: f64) -> Self {
        LegacyRoundPeg { radius }
    }

    fn get_redius(&self) -> f64 {
        self.radius
    }
}

struct SquarePegAdapter {
    round_peg: LegacyRoundPeg,
}
impl SquarePegAdapter {
    fn new(round_peg: LegacyRoundPeg) -> Self {
        SquarePegAdapter { round_peg }
    }
}
impl Shape for SquarePegAdapter {
    fn get_area(&self) -> f64 {
        let side = self.round_peg.get_redius() * 2.0;
        side * side
    }
}

// /////////////////////////////////////////////////
// Bridge

trait Device {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
}

struct Tv {
    on: bool,
}
impl Device for Tv {
    fn is_enabled(&self) -> bool {
        self.on
    }
    fn enable(&mut self) {
        self.on = true;
    }
    fn disable(&mut self) {
        self.on = false;
    }
}

struct Radio {
    on: bool,
}
impl Device for Radio {
    fn is_enabled(&self) -> bool {
        self.on
    }
    fn enable(&mut self) {
        self.on = true;
    }
    fn disable(&mut self) {
        self.on = false;
    }
}

struct Remote {
    device: Box<dyn Device>,
}
impl Remote {
    fn new(device: Box<dyn Device>) -> Self {
        Remote { device }
    }

    fn toggle_power(&mut self) {
        if self.device.is_enabled() {
            self.device.disable();
        } else {
            self.device.enable();
        }
    }

    // 테스트 검증을 위한 헬퍼 메서드
    fn is_device_on(&self) -> bool {
        self.device.is_enabled()
    }
}

// /////////////////////////////////////////////////
// Composite

trait FileSystemComponent {
    fn get_name(&self) -> &str;
    fn get_size(&self) -> u32;
}

struct File {
    name: String,
    size: u32,
}
impl File {
    fn new(name: &str, size: u32) -> Self {
        File {
            name: name.to_string(),
            size,
        }
    }
}

impl FileSystemComponent for File {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_size(&self) -> u32 {
        self.size
    }
}

struct Directory {
    name: String,
    children: Vec<Box<dyn FileSystemComponent>>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            children: Vec::new(),
        }
    }
    fn add(&mut self, component: Box<dyn FileSystemComponent>) {
        self.children.push(component);
    }
}

impl FileSystemComponent for Directory {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_size(&self) -> u32 {
        self.children.iter().map(|child| child.get_size()).sum()
    }
}

// /////////////////////////////////////////////////
// Decorator

trait Coffee {
    fn get_cost(&self) -> u32;
}

struct SimpleCoffee;
impl Coffee for SimpleCoffee {
    fn get_cost(&self) -> u32 {
        2000
    }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}
impl MilkDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        MilkDecorator { coffee }
    }
}

impl Coffee for MilkDecorator {
    fn get_cost(&self) -> u32 {
        self.coffee.get_cost() + 500
    }
}

struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}
impl SugarDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        SugarDecorator { coffee }
    }
}
impl Coffee for SugarDecorator {
    fn get_cost(&self) -> u32 {
        self.coffee.get_cost() + 200
    }
}

// /////////////////////////////////////////////////
// Facade

struct Cpu;
impl Cpu {
    fn freeze(&self) -> String {
        "CPU frozen.".to_string()
    }
    fn jump(&self) -> String {
        "CPU jumped to boot address.".to_string()
    }
    fn execute(&self) -> String {
        "CPU executing commands.".to_string()
    }
}

struct Memory;
impl Memory {
    fn load(&self) -> String {
        "Memory loaded bootloader.".to_string()
    }
}

struct HardDrive;
impl HardDrive {
    fn read_boot_sector(&self) -> String {
        "HardDrive read boot sector.".to_string()
    }
}

struct ComputerFacade {
    cpu: Cpu,
    memory: Memory,
    harddrive: HardDrive,
}

impl ComputerFacade {
    fn new() -> Self {
        ComputerFacade {
            cpu: Cpu,
            memory: Memory,
            harddrive: HardDrive,
        }
    }
    fn start_computer(&self) -> String {
        vec![
            self.cpu.freeze(),
            self.memory.load(),
            self.harddrive.read_boot_sector(),
            self.cpu.jump(),
            self.cpu.execute(),
        ]
        .join("\n")
    }
}

// /////////////////////////////////////////////////
// Flyweight

#[derive(Debug, PartialEq)]
struct TreeType {
    name: String,
    texture: Vec<u8>,
}

struct TreeFactory {
    tree_types: HashMap<String, Rc<TreeType>>,
}
impl TreeFactory {
    fn new() -> Self {
        TreeFactory {
            tree_types: HashMap::new(),
        }
    }
    fn get_tree_type(&mut self, name: &str) -> Rc<TreeType> {
        self.tree_types
            .entry(name.to_string())
            .or_insert_with(|| {
                Rc::new(TreeType {
                    name: name.to_string(),
                    texture: vec![1, 2, 3, 4],
                })
            })
            .clone()
    }
}

struct Tree {
    x: i32,
    y: i32,
    tree_type: Rc<TreeType>,
}

impl Tree {
    fn new(x: i32, y: i32, tree_type: Rc<TreeType>) -> Self {
        Tree { x, y, tree_type }
    }
}

// /////////////////////////////////////////////////
// Proxy

trait Internet {
    fn connect_to(&self, host: &str) -> String;
}

struct RealInternet;
impl Internet for RealInternet {
    fn connect_to(&self, host: &str) -> String {
        format!("Connecting to {}", host)
    }
}

struct ProxyInternet {
    real_internet: RealInternet,
    banned_sites: Vec<String>,
}
impl ProxyInternet {
    fn new() -> Self {
        ProxyInternet {
            real_internet: RealInternet,
            banned_sites: vec!["banned-site.com".to_string(), "malware.io".to_string()],
        }
    }
}

impl Internet for ProxyInternet {
    fn connect_to(&self, host: &str) -> String {
        if self.banned_sites.contains(&host.to_string()) {
            "Access Denied".to_string()
        } else {
            self.real_internet.connect_to(host)
        }
    }
    // 이 부분을 구현하세요.
}

#[cfg(test)]
#[path = "behavioral_test.rs"]
mod behavioral_test;
