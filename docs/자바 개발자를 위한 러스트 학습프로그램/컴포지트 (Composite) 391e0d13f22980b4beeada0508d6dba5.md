# 컴포지트 (Composite):

- **목표:** 개별 객체(Leaf)와 복합 객체(Composite)가 동일한 인터페이스(`trait`)를 공유하여, 클라이언트가 이들을 단일 구조처럼 투포크/트리 형태로 일관되게 다룰 수 있도록 하는 컴포지트 패턴을 구현합니다.
- **조건:** 복합 객체는 내부에 동일한 인터페이스를 구현하는 하위 자식들의 리스트(`Vec<Box<dyn Trait>>`)를 소유하고 관리해야 합니다.

### [연습 문제] 컴포지트 패턴 구현

파일 시스템의 파일(File)과 디렉터리(Directory) 계층 구조를 컴포지트 패턴으로 구현하세요. 디렉터리는 내부에 파일과 다른 디렉터리를 모두 포함할 수 있어야 합니다.

1. `FileSystemComponent` 트레이트는 이름(`get_name`)과 크기(`get_size`)를 반환하는 메서드를 가집니다.
2. `File` 구조체는 개별 객체(Leaf)로서 이름과 고정된 크기를 가집니다.
3. `Directory` 구조체는 복합 객체(Composite)로서 이름과 하위 컴포넌트들을 담는 `Vec<Box<dyn FileSystemComponent>>` 필드를 가집니다. 디렉터리의 크기는 내부에 포함된 모든 자식 컴포넌트 크기의 총합입니다.

```rust
// 1. 공통 인터페이스 (Component)
trait FileSystemComponent {
    fn get_name(&self) -> &str;
    fn get_size(&self) -> u32;
}

// 2. 단일 파일 객체 (Leaf)
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

// 3. 복합 디렉터리 객체 (Composite)
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

    // 자식 컴포넌트를 추가하는 메서드
    fn add(&mut self, component: Box<dyn FileSystemComponent>) {
        self.children.push(component);
    }
}

// Directory에 FileSystemComponent 트레이트를 구현하세요.
// get_size 메서드는 내부 children에 포함된 모든 요소의 get_size() 합계를 구하여 반환해야 합니다.
impl FileSystemComponent for Directory {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_size(&self) -> u32 {
        // 이 부분을 구현하세요.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_pattern() {
        let mut root = Directory::new("root");
        let file1 = Box::new(File::new("file1.txt", 100));
        let file2 = Box::new(File::new("file2.txt", 200));

        root.add(file1);
        root.add(file2);

        let mut sub_dir = Directory::new("sub");
        let file3 = Box::new(File::new("file3.txt", 50));
        sub_dir.add(file3);

        root.add(Box::new(sub_dir));

        // 전체 크기 계산: file1(100) + file2(200) + sub_dir(file3(50)) = 350
        assert_eq!(root.get_size(), 350);
    }
}
```